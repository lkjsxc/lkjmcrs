use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::inventory_packets;
use crate::probe::play_bootstrap::complete_configuration;
use crate::probe::terrain_chunk::DecodedChunk;
use crate::probe::validation::{validate_chunk_batch_finished, validate_chunk_radius};
use crate::protocol::types::{LoginStart, NextState};
use crate::protocol::{codec, ids};
use tokio::net::TcpStream;
use uuid::Uuid;

type ErrorBox = Box<dyn std::error::Error>;

pub(super) async fn run(host: &str) -> Result<(), ErrorBox> {
    let mut stream = super::retry_connect(|| async move {
        Ok::<TcpStream, ErrorBox>(TcpStream::connect(host).await?)
    })
    .await?;
    super::send_handshake(&mut stream, host, NextState::Login).await?;
    let login = LoginStart::encode("CaveProbe", Uuid::from_u128(0));
    codec::write_packet(&mut stream, ids::login::START, &login).await?;
    super::expect(&mut stream, ids::login::SUCCESS, "cave login").await?;
    codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
    complete_configuration(&mut stream).await?;
    expect_cave_bootstrap(&mut stream).await
}

async fn expect_cave_bootstrap(stream: &mut TcpStream) -> Result<(), ErrorBox> {
    for (id, phase) in [
        (ids::play::LOGIN, "cave play login"),
        (ids::play::DEFAULT_SPAWN_POSITION, "cave spawn"),
        (ids::play::SET_TIME, "cave time"),
        (ids::play::PLAYER_ABILITIES, "cave abilities"),
        (ids::play::UPDATE_HEALTH, "cave health"),
        (ids::play::DECLARE_COMMANDS, "cave commands"),
    ] {
        super::expect(stream, id, phase).await?;
    }
    inventory_packets::expect_held_item_slot(stream).await?;
    inventory_packets::expect_player_inventory(stream).await?;
    super::expect(stream, ids::play::GAME_STATE_CHANGE, "cave readiness").await?;
    super::expect(stream, ids::play::CHUNK_CACHE_CENTER, "cave center").await?;
    let radius = super::expect(stream, ids::play::CHUNK_CACHE_RADIUS, "cave radius").await?;
    let chunk_count = validate_chunk_radius(radius.data)?;
    super::expect(stream, ids::play::CHUNK_BATCH_START, "cave batch").await?;
    let mut cave_air = false;
    let mut natural = false;
    for _ in 0..chunk_count {
        let packet = super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "cave chunk").await?;
        chunk::validate_level_chunk_with_light(packet.data.clone())?;
        let decoded = DecodedChunk::from_packet(packet.data)?;
        cave_air |= decoded.has_enclosed_underground_air();
        natural |= decoded.has_non_flat_surface();
    }
    if !cave_air {
        return Err(Box::new(ProbeError::Phase("cave underground air")));
    }
    if !natural {
        return Err(Box::new(ProbeError::Phase("cave natural terrain")));
    }
    let finished = super::expect(stream, ids::play::CHUNK_BATCH_FINISHED, "cave done").await?;
    validate_chunk_batch_finished(finished.data, chunk_count)
}
