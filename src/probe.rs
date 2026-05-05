use crate::protocol::codec;
use crate::protocol::ids;
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
    expect(&mut stream, ids::login::SUCCESS, "login success").await?;
    codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
    expect(&mut stream, ids::config::FINISH, "finish config").await?;
    codec::write_packet(&mut stream, ids::config::FINISH, &[]).await?;
    let ready = expect(&mut stream, ids::play::READY, "play ready").await?;
    let text = codec::read_string(&mut Cursor::new(ready.data))?;
    if text != "play-ready:Probe" {
        return Err(Box::new(ProbeError::Phase("play ready text")));
    }
    let keepalive = expect(&mut stream, ids::play::KEEPALIVE, "keepalive").await?;
    if codec::read_i64(&mut Cursor::new(keepalive.data))? != 1 {
        return Err(Box::new(ProbeError::Phase("keepalive id")));
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
