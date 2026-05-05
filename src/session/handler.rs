use crate::config::Config;
use crate::protocol::codec::{self, Packet};
use crate::protocol::ids;
use crate::protocol::types::{Handshake, LoginStart, NextState};
use crate::protocol::{PROTOCOL_VERSION, status};
use crate::session::profile::{offline_uuid, validate_name};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct ServerContext {
    pub config: Config,
    players: AtomicUsize,
}

impl ServerContext {
    pub fn new(config: Config) -> Arc<Self> {
        Arc::new(Self {
            config,
            players: AtomicUsize::new(0),
        })
    }

    fn player_count(&self) -> usize {
        self.players.load(Ordering::Relaxed)
    }
}

pub async fn handle_connection(
    mut stream: TcpStream,
    context: Arc<ServerContext>,
) -> Result<(), Box<dyn std::error::Error>> {
    let first = codec::read_packet(&mut stream).await?;
    if first.id != ids::HANDSHAKE {
        return Ok(());
    }
    let handshake = Handshake::decode(first.data)?;
    match handshake.next_state {
        NextState::Status => handle_status(stream, context).await?,
        NextState::Login => handle_login(stream, context, handshake.protocol).await?,
    }
    Ok(())
}

async fn handle_status(
    mut stream: TcpStream,
    context: Arc<ServerContext>,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = codec::read_packet(&mut stream).await?;
    if request.id != ids::status::REQUEST {
        return Ok(());
    }
    let json = status::response_json(
        &context.config.motd,
        context.player_count(),
        context.config.max_players,
    )?;
    let mut payload = Vec::new();
    codec::write_string(&mut payload, &json);
    codec::write_packet(&mut stream, ids::status::RESPONSE, &payload).await?;

    if let Ok(ping) = codec::read_packet(&mut stream).await
        && ping.id == ids::status::PING
    {
        codec::write_packet(&mut stream, ids::status::PONG, &ping.data).await?;
    }
    Ok(())
}

async fn handle_login(
    mut stream: TcpStream,
    context: Arc<ServerContext>,
    protocol: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    if protocol != PROTOCOL_VERSION {
        send_disconnect(&mut stream, "Unsupported Minecraft protocol").await?;
        return Ok(());
    }
    let start = codec::read_packet(&mut stream).await?;
    if start.id != ids::login::START {
        return Ok(());
    }
    let login = LoginStart::decode(start.data)?;
    validate_name(&login.name)?;
    let uuid = offline_uuid(&login.name);
    send_login_success(&mut stream, &login.name, uuid).await?;
    expect_packet(&mut stream, ids::login::ACKNOWLEDGED).await?;
    codec::write_packet(&mut stream, ids::config::FINISH, &[]).await?;
    expect_packet(&mut stream, ids::config::FINISH).await?;
    context.players.fetch_add(1, Ordering::Relaxed);
    send_play_ready(&mut stream, &login.name).await?;
    context.players.fetch_sub(1, Ordering::Relaxed);
    Ok(())
}

async fn send_disconnect(stream: &mut TcpStream, reason: &str) -> Result<(), codec::CodecError> {
    let json = serde_json::json!({ "text": reason }).to_string();
    let mut payload = Vec::new();
    codec::write_string(&mut payload, &json);
    codec::write_packet(stream, ids::login::DISCONNECT, &payload).await
}

async fn send_login_success(
    stream: &mut TcpStream,
    name: &str,
    uuid: uuid::Uuid,
) -> Result<(), codec::CodecError> {
    let mut payload = Vec::new();
    codec::write_uuid(&mut payload, uuid);
    codec::write_string(&mut payload, name);
    codec::write_var_i32(&mut payload, 0);
    codec::write_bool(&mut payload, false);
    codec::write_packet(stream, ids::login::SUCCESS, &payload).await
}

async fn expect_packet(stream: &mut TcpStream, expected: i32) -> Result<Packet, codec::CodecError> {
    let packet = codec::read_packet(stream).await?;
    if packet.id == expected {
        Ok(packet)
    } else {
        Err(codec::CodecError::Eof)
    }
}

async fn send_play_ready(stream: &mut TcpStream, name: &str) -> Result<(), codec::CodecError> {
    let mut ready = Vec::new();
    codec::write_string(&mut ready, &format!("play-ready:{name}"));
    codec::write_packet(stream, ids::play::READY, &ready).await?;
    let mut keepalive = Vec::new();
    codec::write_i64(&mut keepalive, 1);
    codec::write_packet(stream, ids::play::KEEPALIVE, &keepalive).await
}
