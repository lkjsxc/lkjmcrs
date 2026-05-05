use crate::protocol::codec;
use crate::protocol::configuration::{self, KnownPack};
use crate::protocol::ids;
use crate::protocol::play;
use crate::protocol::types::{Handshake, LoginStart, NextState};
use crate::protocol::{MINECRAFT_VERSION, PROTOCOL_VERSION};
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
    expect(&mut stream, ids::play::CHUNK_CACHE_CENTER, "chunk center").await?;
    expect(&mut stream, ids::play::CHUNK_CACHE_RADIUS, "chunk radius").await?;
    expect(&mut stream, ids::play::CHUNK_BATCH_START, "chunk batch").await?;
    for _ in 0..9 {
        expect(
            &mut stream,
            ids::play::LEVEL_CHUNK_WITH_LIGHT,
            "level chunk with light",
        )
        .await?;
        expect(&mut stream, ids::play::UPDATE_LIGHT, "update light").await?;
    }
    expect(
        &mut stream,
        ids::play::CHUNK_BATCH_FINISHED,
        "chunk batch finished",
    )
    .await?;
    expect(&mut stream, ids::play::DEFAULT_SPAWN_POSITION, "spawn").await?;
    expect(&mut stream, ids::play::SET_TIME, "time").await?;
    expect(&mut stream, ids::play::PLAYER_ABILITIES, "abilities").await?;
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

fn validate_status_json(json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let version = &value["version"];
    if version["name"] != MINECRAFT_VERSION || version["protocol"] != PROTOCOL_VERSION {
        return Err(Box::new(ProbeError::Phase("status version")));
    }
    Ok(())
}

fn validate_known_packs(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let packs = configuration::decode_known_packs(data)?;
    if packs != vec![KnownPack::vanilla_core()] {
        return Err(Box::new(ProbeError::Phase("known packs payload")));
    }
    Ok(())
}

fn validate_login_success(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let _uuid = codec::read_uuid(&mut cursor)?;
    let username = codec::read_string(&mut cursor)?;
    let properties = codec::read_var_i32(&mut cursor)?;
    if username != "Probe" || properties != 0 {
        return Err(Box::new(ProbeError::Phase("login success payload")));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("login success trailing bytes")));
    }
    Ok(())
}

fn validate_position_packet(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let teleport_id = codec::read_var_i32(&mut cursor)?;
    if teleport_id != play::Bootstrap::new(100).teleport_id() {
        return Err(Box::new(ProbeError::Phase("teleport id")));
    }
    Ok(())
}
