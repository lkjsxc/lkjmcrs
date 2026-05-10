use crate::probe::ProbeError;
use crate::probe::inventory_packets;
use crate::probe::live_play;
use crate::probe::play_bootstrap::complete_configuration;
use crate::probe::scale_chunk_stream_packets as packets;
use crate::probe::validation::{
    PositionPacket, decode_login_packet, decode_position_packet, validate_game_state_change,
    validate_login_success,
};
use crate::protocol::types::{LoginStart, NextState};
use crate::protocol::{codec, ids};
use std::collections::HashSet;
use tokio::net::TcpStream;
use uuid::Uuid;

const NAME: &str = "ScaleChunkStream";
const INITIAL_CHUNKS: usize = 25;
const TOTAL_CHUNKS: usize = 81;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = connect(host).await?;
    let mut seen = read_bootstrap(&mut stream, packets::RADIUS).await?;
    collect_until(&mut stream, &mut seen, TOTAL_CHUNKS).await?;
    move_east_and_collect_new_column(&mut stream).await
}

pub(super) async fn connect(host: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let host = host.to_string();
    super::retry_connect(|| {
        let host = host.clone();
        async move {
            let mut stream = TcpStream::connect(&host).await?;
            super::send_handshake(&mut stream, &host, NextState::Login).await?;
            let login = LoginStart::encode(NAME, Uuid::from_u128(0));
            codec::write_packet(&mut stream, ids::login::START, &login).await?;
            let success = super::expect(&mut stream, ids::login::SUCCESS, "login success").await?;
            validate_login_success(success.data, NAME)?;
            codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
            complete_configuration(&mut stream).await?;
            Ok(stream)
        }
    })
    .await
}

pub(super) async fn read_bootstrap(
    stream: &mut TcpStream,
    expected_radius: i32,
) -> Result<HashSet<(i32, i32)>, Box<dyn std::error::Error>> {
    Ok(read_bootstrap_with_position(stream, expected_radius)
        .await?
        .0)
}

pub(super) async fn read_bootstrap_with_position(
    stream: &mut TcpStream,
    expected_radius: i32,
) -> Result<(HashSet<(i32, i32)>, PositionPacket), Box<dyn std::error::Error>> {
    let login = super::expect(stream, ids::play::LOGIN, "play login").await?;
    let login = decode_login_packet(login.data)?;
    if login.view_distance != expected_radius {
        return Err(Box::new(ProbeError::Phase("login view distance")));
    }
    super::expect(stream, ids::play::DEFAULT_SPAWN_POSITION, "spawn").await?;
    super::expect(stream, ids::play::SET_TIME, "time").await?;
    super::expect(stream, ids::play::PLAYER_ABILITIES, "abilities").await?;
    super::vitals_packets::expect_update_health(stream).await?;
    super::expect(stream, ids::play::DECLARE_COMMANDS, "declare commands").await?;
    inventory_packets::expect_held_item_slot(stream).await?;
    inventory_packets::expect_player_inventory(stream).await?;
    let ready = super::expect(stream, ids::play::GAME_STATE_CHANGE, "chunk readiness").await?;
    validate_game_state_change(ready.data)?;
    super::expect(stream, ids::play::CHUNK_CACHE_CENTER, "chunk center").await?;
    let radius = super::expect(stream, ids::play::CHUNK_CACHE_RADIUS, "chunk radius").await?;
    packets::expect_radius_value(radius.data, expected_radius)?;
    let seen = packets::expect_batch(stream, INITIAL_CHUNKS)
        .await?
        .positions;
    let position = super::expect(stream, ids::play::PLAYER_POSITION, "position").await?;
    let position = decode_position_packet(position.data)?;
    packets::confirm_and_ack_keepalive(stream).await?;
    Ok((seen, position))
}

async fn collect_until(
    stream: &mut TcpStream,
    seen: &mut HashSet<(i32, i32)>,
    total: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    while seen.len() < total {
        let batch = packets::read_next_batch(stream).await?;
        seen.extend(batch.positions);
    }
    Ok(())
}

async fn move_east_and_collect_new_column(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    live_play::send_position_look_at(stream, 16.5, 80.0, 0.5, 0.0, 0.0).await?;
    packets::expect_cache_center(stream, 1, 0).await?;
    let mut unloaded = HashSet::new();
    for _ in 0..9 {
        unloaded.insert(packets::read_unload(stream).await?);
    }
    let expected_unload: HashSet<_> = (-4..=4).map(|z| (-4, z)).collect();
    if unloaded != expected_unload {
        return Err(Box::new(ProbeError::Phase("scale unload column")));
    }
    let mut loaded = HashSet::new();
    while loaded.len() < 9 {
        loaded.extend(packets::read_next_batch(stream).await?.positions);
    }
    let expected_load: HashSet<_> = (-4..=4).map(|z| (5, z)).collect();
    if loaded != expected_load {
        return Err(Box::new(ProbeError::Phase("scale load column")));
    }
    Ok(())
}
