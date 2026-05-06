use crate::protocol::codec;
mod block_mutation;
mod chunk;
mod chunk_stream;
mod inventory_packets;
mod inventory_sync;
mod live_play;
mod multiplayer_mutation;
mod persistence;
mod play_client;
mod profile_reconnect;
mod smp_commands;
mod survival_expect;
mod survival_item;
mod validation;

use crate::probe::play_client::PlayClient;
use crate::probe::validation::validate_status_json;
use crate::protocol::PROTOCOL_VERSION;
use crate::protocol::ids;
use crate::protocol::types::{Handshake, NextState};
use std::io::Cursor;
use thiserror::Error;
use tokio::net::TcpStream;

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
    let mut client = PlayClient::connect(host, "Probe").await?;
    block_mutation::place_and_break(&mut client.stream).await?;
    let next_keepalive = live_play::expect_keepalive_after_time(&mut client.stream).await?;
    if next_keepalive != 2 {
        return Err(Box::new(ProbeError::Phase("periodic keepalive id")));
    }
    println!("login-play probe ok");
    Ok(())
}

pub async fn multiplayer_mutation(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    multiplayer_mutation::run(host).await?;
    println!("multiplayer-mutation probe ok");
    Ok(())
}

pub async fn persist_place(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    persistence::place(host).await?;
    println!("persist-place probe ok");
    Ok(())
}

pub async fn persist_check(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    persistence::check(host).await?;
    println!("persist-check probe ok");
    Ok(())
}

pub async fn profile_reconnect(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    profile_reconnect::run(host).await?;
    println!("profile-reconnect probe ok");
    Ok(())
}

pub async fn chunk_stream(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    chunk_stream::run(host).await?;
    println!("chunk-stream probe ok");
    Ok(())
}

pub async fn survival_item(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    survival_item::run(host).await?;
    println!("survival-item probe ok");
    Ok(())
}

pub async fn inventory_sync(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    inventory_sync::run(host).await?;
    println!("inventory-sync probe ok");
    Ok(())
}

pub async fn smp_commands(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    smp_commands::run(host).await?;
    println!("smp-commands probe ok");
    Ok(())
}

pub(super) async fn send_handshake(
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

pub(super) async fn expect(
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
