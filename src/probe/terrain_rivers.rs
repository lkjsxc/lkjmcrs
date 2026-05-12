use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::inventory_packets;
use crate::probe::play_bootstrap::complete_configuration;
use crate::probe::terrain_chunk::DecodedChunk;
use crate::probe::validation::{validate_chunk_batch_finished, validate_chunk_radius};
use crate::protocol::chunk::WATER_ID;

const WATER_LEVEL: i32 = 72;
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
    let login = LoginStart::encode("RiverProbe", Uuid::from_u128(0));
    codec::write_packet(&mut stream, ids::login::START, &login).await?;
    super::expect(&mut stream, ids::login::SUCCESS, "river login").await?;
    codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
    complete_configuration(&mut stream).await?;
    expect_river_bootstrap(&mut stream).await
}

async fn expect_river_bootstrap(stream: &mut TcpStream) -> Result<(), ErrorBox> {
    for (id, phase) in [
        (ids::play::LOGIN, "river play login"),
        (ids::play::DEFAULT_SPAWN_POSITION, "river spawn"),
        (ids::play::SET_TIME, "river time"),
        (ids::play::PLAYER_ABILITIES, "river abilities"),
        (ids::play::UPDATE_HEALTH, "river health"),
        (ids::play::DECLARE_COMMANDS, "river commands"),
    ] {
        super::expect(stream, id, phase).await?;
    }
    inventory_packets::expect_held_item_slot(stream).await?;
    inventory_packets::expect_player_inventory(stream).await?;
    super::expect(stream, ids::play::GAME_STATE_CHANGE, "river readiness").await?;
    super::expect(stream, ids::play::CHUNK_CACHE_CENTER, "river center").await?;
    let radius = super::expect(stream, ids::play::CHUNK_CACHE_RADIUS, "river radius").await?;
    let chunk_count = validate_chunk_radius(radius.data)?;
    super::expect(stream, ids::play::CHUNK_BATCH_START, "river batch").await?;
    let mut has_water = false;
    let mut has_level_water = false;
    let mut natural = false;
    for _ in 0..chunk_count {
        let packet =
            super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "river chunk").await?;
        chunk::validate_level_chunk_with_light(packet.data.clone())?;
        let decoded = DecodedChunk::from_packet(packet.data)?;
        has_water |= decoded.contains_state(WATER_ID);
        has_level_water |= decoded.contains_state_at_y(WATER_ID, WATER_LEVEL);
        natural |= decoded.has_non_flat_surface();
    }
    if !has_water {
        return Err(Box::new(ProbeError::Phase("river water")));
    }
    if !has_level_water {
        return Err(Box::new(ProbeError::Phase("river water level")));
    }
    if !natural {
        return Err(Box::new(ProbeError::Phase("river natural terrain")));
    }
    let finished = super::expect(stream, ids::play::CHUNK_BATCH_FINISHED, "river done").await?;
    validate_chunk_batch_finished(finished.data, chunk_count)
}
