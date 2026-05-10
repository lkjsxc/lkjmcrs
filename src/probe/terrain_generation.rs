use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::inventory_packets;
use crate::probe::play_bootstrap::complete_configuration;
use crate::probe::terrain_chunk::DecodedChunk;
use crate::probe::validation::{validate_chunk_batch_finished, validate_chunk_radius};
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
    let login = LoginStart::encode("TerrainProbe", Uuid::from_u128(0));
    codec::write_packet(&mut stream, ids::login::START, &login).await?;
    super::expect(&mut stream, ids::login::SUCCESS, "terrain login").await?;
    codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
    complete_configuration(&mut stream).await?;
    expect_bootstrap_terrain(&mut stream).await
}

async fn expect_bootstrap_terrain(stream: &mut TcpStream) -> Result<(), ErrorBox> {
    super::expect(stream, ids::play::LOGIN, "terrain play login").await?;
    let spawn_packet =
        super::expect(stream, ids::play::DEFAULT_SPAWN_POSITION, "terrain spawn").await?;
    let spawn = decode_default_spawn(spawn_packet.data)?;
    for (id, phase) in [
        (ids::play::SET_TIME, "terrain time"),
        (ids::play::PLAYER_ABILITIES, "terrain abilities"),
        (ids::play::UPDATE_HEALTH, "terrain health"),
        (ids::play::DECLARE_COMMANDS, "terrain commands"),
    ] {
        super::expect(stream, id, phase).await?;
    }
    inventory_packets::expect_held_item_slot(stream).await?;
    inventory_packets::expect_player_inventory(stream).await?;
    super::expect(
        stream,
        ids::play::GAME_STATE_CHANGE,
        "terrain chunk readiness",
    )
    .await?;
    super::expect(stream, ids::play::CHUNK_CACHE_CENTER, "terrain center").await?;
    let radius = super::expect(stream, ids::play::CHUNK_CACHE_RADIUS, "terrain radius").await?;
    let chunk_count = validate_chunk_radius(radius.data)?;
    super::expect(stream, ids::play::CHUNK_BATCH_START, "terrain batch").await?;
    let mut natural = false;
    let mut dry_spawn = false;
    for _ in 0..chunk_count {
        let packet =
            super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "terrain chunk").await?;
        chunk::validate_level_chunk_with_light(packet.data.clone())?;
        let decoded = DecodedChunk::from_packet(packet.data)?;
        if decoded.has_non_flat_surface() {
            natural = true;
        }
        if decoded.position() == (spawn.0.div_euclid(16), spawn.2.div_euclid(16)) {
            dry_spawn = decoded.has_dry_headroom(
                spawn.0.rem_euclid(16) as usize,
                spawn.1 - 1,
                spawn.2.rem_euclid(16) as usize,
            );
        }
    }
    if !natural {
        return Err(Box::new(ProbeError::Phase("terrain natural variation")));
    }
    if !dry_spawn {
        return Err(Box::new(ProbeError::Phase("terrain dry spawn")));
    }
    let finished = super::expect(
        stream,
        ids::play::CHUNK_BATCH_FINISHED,
        "terrain batch finished",
    )
    .await?;
    validate_chunk_batch_finished(finished.data, chunk_count)
}

fn decode_default_spawn(data: Vec<u8>) -> Result<(i32, i32, i32), ErrorBox> {
    let mut cursor = Cursor::new(data);
    if codec::read_string(&mut cursor)? != "minecraft:overworld" {
        return Err(Box::new(ProbeError::Phase("terrain spawn dimension")));
    }
    let pos = codec::read_position(&mut cursor)?;
    codec::read_f32(&mut cursor)?;
    codec::read_f32(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("terrain spawn trailing bytes")));
    }
    Ok(pos)
}
