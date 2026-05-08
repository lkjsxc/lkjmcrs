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
    let mut stream = TcpStream::connect(host).await?;
    super::send_handshake(&mut stream, host, NextState::Login).await?;
    let login = LoginStart::encode("TerrainProbe", Uuid::from_u128(0));
    codec::write_packet(&mut stream, ids::login::START, &login).await?;
    super::expect(&mut stream, ids::login::SUCCESS, "terrain login").await?;
    codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
    complete_configuration(&mut stream).await?;
    expect_bootstrap_terrain(&mut stream).await
}

async fn expect_bootstrap_terrain(stream: &mut TcpStream) -> Result<(), ErrorBox> {
    for (id, phase) in [
        (ids::play::LOGIN, "terrain play login"),
        (ids::play::DEFAULT_SPAWN_POSITION, "terrain spawn"),
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
    for _ in 0..chunk_count {
        let packet =
            super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "terrain chunk").await?;
        chunk::validate_level_chunk_with_light(packet.data.clone())?;
        let decoded = DecodedChunk::from_packet(packet.data)?;
        if decoded.is_spawn_plateau() {
            decoded.assert_flat_surface()?;
        } else if decoded.has_non_flat_surface() {
            natural = true;
        }
    }
    if !natural {
        return Err(Box::new(ProbeError::Phase("terrain natural variation")));
    }
    let finished = super::expect(
        stream,
        ids::play::CHUNK_BATCH_FINISHED,
        "terrain batch finished",
    )
    .await?;
    validate_chunk_batch_finished(finished.data, chunk_count)
}
