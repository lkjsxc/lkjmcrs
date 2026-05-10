use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::inventory_packets;
use crate::probe::play_bootstrap::complete_configuration;
use crate::probe::terrain_chunk::DecodedChunk;
use crate::probe::validation::{validate_chunk_batch_finished, validate_chunk_radius};
use crate::protocol::chunk::{SPRUCE_LEAVES_ID, SPRUCE_LOG_ID, WATER_ID};
use crate::protocol::types::{LoginStart, NextState};
use crate::protocol::{codec, ids};
use std::io::Cursor;
use tokio::net::TcpStream;
use uuid::Uuid;

type ErrorBox = Box<dyn std::error::Error>;

pub(super) async fn run(host: &str) -> Result<(), ErrorBox> {
    let mut stream = super::retry_connect(|| async move {
        Ok::<TcpStream, ErrorBox>(TcpStream::connect(host).await?)
    })
    .await?;
    super::send_handshake(&mut stream, host, NextState::Login).await?;
    let login = LoginStart::encode("QualityProbe", Uuid::from_u128(0));
    codec::write_packet(&mut stream, ids::login::START, &login).await?;
    super::expect(&mut stream, ids::login::SUCCESS, "quality login").await?;
    codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
    complete_configuration(&mut stream).await?;
    expect_quality_bootstrap(&mut stream).await
}

async fn expect_quality_bootstrap(stream: &mut TcpStream) -> Result<(), ErrorBox> {
    super::expect(stream, ids::play::LOGIN, "quality play login").await?;
    let spawn = spawn_from_packet(
        super::expect(stream, ids::play::DEFAULT_SPAWN_POSITION, "quality spawn")
            .await?
            .data,
    )?;
    for (id, phase) in [
        (ids::play::SET_TIME, "quality time"),
        (ids::play::PLAYER_ABILITIES, "quality abilities"),
        (ids::play::UPDATE_HEALTH, "quality health"),
        (ids::play::DECLARE_COMMANDS, "quality commands"),
    ] {
        super::expect(stream, id, phase).await?;
    }
    inventory_packets::expect_held_item_slot(stream).await?;
    inventory_packets::expect_player_inventory(stream).await?;
    super::expect(stream, ids::play::GAME_STATE_CHANGE, "quality ready").await?;
    super::expect(stream, ids::play::CHUNK_CACHE_CENTER, "quality center").await?;
    let radius = super::expect(stream, ids::play::CHUNK_CACHE_RADIUS, "quality radius").await?;
    let chunk_count = validate_chunk_radius(radius.data)?;
    let seen = read_quality_batch(stream, chunk_count, spawn).await?;
    seen.require_all()
}

async fn read_quality_batch(
    stream: &mut TcpStream,
    chunk_count: usize,
    spawn: (i32, i32, i32),
) -> Result<QualitySeen, ErrorBox> {
    super::expect(stream, ids::play::CHUNK_BATCH_START, "quality batch").await?;
    let mut seen = QualitySeen::default();
    for _ in 0..chunk_count {
        let packet =
            super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "quality chunk").await?;
        chunk::validate_level_chunk_with_light(packet.data.clone())?;
        seen.record(DecodedChunk::from_packet(packet.data)?, spawn);
    }
    let finished = super::expect(stream, ids::play::CHUNK_BATCH_FINISHED, "quality done").await?;
    validate_chunk_batch_finished(finished.data, chunk_count)?;
    Ok(seen)
}

#[derive(Default)]
struct QualitySeen {
    dry_spawn: bool,
    natural: bool,
    water: bool,
    wood: bool,
}

impl QualitySeen {
    fn record(&mut self, chunk: DecodedChunk, spawn: (i32, i32, i32)) {
        self.natural |= chunk.has_non_flat_surface();
        self.water |= chunk.contains_state(WATER_ID);
        self.wood |= chunk.contains_state(SPRUCE_LOG_ID) || chunk.contains_state(SPRUCE_LEAVES_ID);
        if chunk.position() == (spawn.0.div_euclid(16), spawn.2.div_euclid(16)) {
            self.dry_spawn = chunk.has_dry_headroom(
                spawn.0.rem_euclid(16) as usize,
                spawn.1 - 1,
                spawn.2.rem_euclid(16) as usize,
            );
        }
    }

    fn require_all(self) -> Result<(), ErrorBox> {
        require(self.dry_spawn, "quality dry spawn")?;
        require(self.natural, "quality non-flat terrain")?;
        require(self.water, "quality water access")?;
        require(self.wood, "quality generated wood")
    }
}

fn require(value: bool, phase: &'static str) -> Result<(), ErrorBox> {
    value
        .then_some(())
        .ok_or_else(|| Box::new(ProbeError::Phase(phase)).into())
}

fn spawn_from_packet(data: Vec<u8>) -> Result<(i32, i32, i32), ErrorBox> {
    let mut cursor = Cursor::new(data);
    if codec::read_string(&mut cursor)? != "minecraft:overworld" {
        return Err(Box::new(ProbeError::Phase("quality spawn dimension")));
    }
    let pos = codec::read_position(&mut cursor)?;
    codec::read_f32(&mut cursor)?;
    codec::read_f32(&mut cursor)?;
    Ok(pos)
}
