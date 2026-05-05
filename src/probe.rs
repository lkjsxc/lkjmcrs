use crate::protocol::codec;
mod chunk;
mod validation;

use crate::probe::validation::{
    validate_chunk_batch_finished, validate_chunk_radius, validate_game_event,
    validate_known_packs, validate_login_success, validate_position_packet, validate_status_json,
};
use crate::protocol::PROTOCOL_VERSION;
use crate::protocol::configuration;
use crate::protocol::ids;
use crate::protocol::play;
use crate::protocol::types::{Handshake, LoginStart, NextState};
use std::io::Cursor;
use thiserror::Error;
use tokio::net::TcpStream;
use uuid::Uuid;

#[derive(Debug, Error)]
enum ProbeError {
    #[error("probe phase failed: {0}")]
    Phase(&'static str),
}

pub async fn status(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(host).await?;
    send_handshake(&mut stream, host, NextState::Status).await?;
    codec::write_packet(&mut stream, ids::status::REQUEST, &[]).await?;
    let response = codec::read_packet(&mut stream).await?;
    if response.id != ids::status::RESPONSE {
        return Err(Box::new(ProbeError::Phase("status response id")));
    }
    let json = codec::read_string(&mut Cursor::new(response.data))?;
    validate_status_json(&json)?;
    let mut ping = Vec::new();
    codec::write_i64(&mut ping, 42);
    codec::write_packet(&mut stream, ids::status::PING, &ping).await?;
    let pong = codec::read_packet(&mut stream).await?;
    if pong.id != ids::status::PONG || codec::read_i64(&mut Cursor::new(pong.data))? != 42 {
        return Err(Box::new(ProbeError::Phase("status pong")));
    }
    println!("status probe ok");
    Ok(())
}

pub async fn login_play(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(host).await?;
    send_handshake(&mut stream, host, NextState::Login).await?;
    let profile_id = Uuid::from_u128(0);
    let login = LoginStart::encode("Probe", profile_id);
    codec::write_packet(&mut stream, ids::login::START, &login).await?;
    let success = expect(&mut stream, ids::login::SUCCESS, "login success").await?;
    validate_login_success(success.data)?;
    codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
    let known = expect(
        &mut stream,
        ids::config::SELECT_KNOWN_PACKS,
        "select known packs",
    )
    .await?;
    validate_known_packs(known.data)?;
    codec::write_packet(
        &mut stream,
        ids::config::SERVERBOUND_SELECT_KNOWN_PACKS,
        &configuration::encode_select_known_packs(),
    )
    .await?;
    for _ in 0..configuration::registry_packet_count() {
        expect(&mut stream, ids::config::REGISTRY_DATA, "registry data").await?;
    }
    expect(&mut stream, ids::config::TAGS, "configuration tags").await?;
    let features = expect(&mut stream, ids::config::FEATURE_FLAGS, "feature flags").await?;
    if features.data != configuration::encode_enabled_features() {
        return Err(Box::new(ProbeError::Phase("feature flags payload")));
    }
    expect(&mut stream, ids::config::FINISH, "finish config").await?;
    codec::write_packet(&mut stream, ids::config::FINISH, &[]).await?;
    expect(&mut stream, ids::play::LOGIN, "play login").await?;
    expect(&mut stream, ids::play::DEFAULT_SPAWN_POSITION, "spawn").await?;
    expect(&mut stream, ids::play::SET_TIME, "time").await?;
    expect(&mut stream, ids::play::PLAYER_ABILITIES, "abilities").await?;
    let game_event = expect(&mut stream, ids::play::GAME_EVENT, "chunk readiness").await?;
    validate_game_event(game_event.data)?;
    expect(&mut stream, ids::play::CHUNK_CACHE_CENTER, "chunk center").await?;
    let radius = expect(&mut stream, ids::play::CHUNK_CACHE_RADIUS, "chunk radius").await?;
    let chunk_count = validate_chunk_radius(radius.data)?;
    expect(&mut stream, ids::play::CHUNK_BATCH_START, "chunk batch").await?;
    for _ in 0..chunk_count {
        let chunk = expect(
            &mut stream,
            ids::play::LEVEL_CHUNK_WITH_LIGHT,
            "level chunk with light",
        )
        .await?;
        chunk::validate_level_chunk_with_light(chunk.data)?;
        expect(&mut stream, ids::play::UPDATE_LIGHT, "update light").await?;
    }
    let finished = expect(
        &mut stream,
        ids::play::CHUNK_BATCH_FINISHED,
        "chunk batch finished",
    )
    .await?;
    validate_chunk_batch_finished(finished.data, chunk_count)?;
    let position = expect(&mut stream, ids::play::PLAYER_POSITION, "position").await?;
    validate_position_packet(position.data)?;
    let mut confirm = Vec::new();
    codec::write_var_i32(&mut confirm, play::Bootstrap::new(100).teleport_id());
    codec::write_packet(
        &mut stream,
        ids::play::SERVERBOUND_TELEPORT_CONFIRM,
        &confirm,
    )
    .await?;
    let keepalive = expect(&mut stream, ids::play::KEEPALIVE, "keepalive").await?;
    if codec::read_i64(&mut Cursor::new(keepalive.data))? != 1 {
        return Err(Box::new(ProbeError::Phase("keepalive id")));
    }
    let next_keepalive = expect(&mut stream, ids::play::KEEPALIVE, "periodic keepalive").await?;
    if codec::read_i64(&mut Cursor::new(next_keepalive.data))? != 2 {
        return Err(Box::new(ProbeError::Phase("periodic keepalive id")));
    }
    println!("login-play probe ok");
    Ok(())
}

async fn send_handshake(
    stream: &mut TcpStream,
    host: &str,
    next_state: NextState,
) -> Result<(), codec::CodecError> {
    let handshake = Handshake {
        protocol: PROTOCOL_VERSION,
        address: host.to_string(),
        port: 25565,
        next_state,
    };
    codec::write_packet(stream, ids::HANDSHAKE, &handshake.encode()).await
}

async fn expect(
    stream: &mut TcpStream,
    id: i32,
    phase: &'static str,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    let packet = codec::read_packet(stream).await?;
    if packet.id == id {
        Ok(packet)
    } else {
        Err(Box::new(ProbeError::Phase(phase)))
    }
}
