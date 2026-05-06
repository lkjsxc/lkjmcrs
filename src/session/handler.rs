use crate::config::Config;
use crate::player::{PlayerDefaults, PlayerStore};
use crate::protocol::codec;
use crate::protocol::ids;
use crate::protocol::types::{Handshake, LoginStart, NextState};
use crate::protocol::{PROTOCOL_VERSION, login, status};
use crate::scheduler::RegionActor;
use crate::session::SessionState;
use crate::session::configuration::handle_configuration;
use crate::session::error::ConnectionError;
use crate::session::io::{
    codec_error, expect_packet, is_connection_closed, protocol_error, read_packet, write_packet,
};
use crate::session::play::{PlaySettings, handle_play};
use crate::session::profile::{offline_uuid, validate_name};
use crate::session::registry::SessionRegistry;
use crate::world::{RegionId, WorldStorage};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct ServerContext {
    pub config: Config,
    pub region: crate::scheduler::RegionHandle,
    pub sessions: SessionRegistry,
    pub player_store: PlayerStore,
    players: AtomicUsize,
}

#[derive(Debug, Error)]
pub enum ServerStartupError {
    #[error("player store failed: {0}")]
    Player(#[from] crate::player::PlayerStoreError),
    #[error("world storage failed: {0}")]
    World(#[from] crate::world::WorldStorageError),
}

impl ServerContext {
    pub fn new(config: Config) -> Result<Arc<Self>, ServerStartupError> {
        let player_store = PlayerStore::open(&config.data_dir)?;
        let world_storage = WorldStorage::new(&config.data_dir);
        world_storage.validate()?;
        let region = RegionActor::spawn_persistent(RegionId(0), world_storage);
        Ok(Arc::new(Self {
            config,
            region,
            sessions: SessionRegistry::default(),
            player_store,
            players: AtomicUsize::new(0),
        }))
    }

    fn player_count(&self) -> usize {
        self.players.load(Ordering::Relaxed)
    }
}

pub async fn handle_connection(
    mut stream: TcpStream,
    context: Arc<ServerContext>,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Handshake;
    let first = read_packet(&mut stream, phase).await?;
    if first.id != ids::HANDSHAKE {
        return Err(protocol_error(phase, "expected handshake packet"));
    }
    let handshake = Handshake::decode(first.data).map_err(|error| codec_error(phase, error))?;
    tracing::debug!(
        phase = %phase,
        protocol = handshake.protocol,
        next_state = ?handshake.next_state,
        "handshake received"
    );
    match handshake.next_state {
        NextState::Status => handle_status(stream, context).await?,
        NextState::Login => handle_login(stream, context, handshake.protocol).await?,
    }
    Ok(())
}

async fn handle_status(
    mut stream: TcpStream,
    context: Arc<ServerContext>,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Status;
    let request = read_packet(&mut stream, phase).await?;
    if request.id != ids::status::REQUEST {
        return Err(protocol_error(phase, "expected status request"));
    }
    let json = status::response_json(
        &context.config.motd,
        context.player_count(),
        context.config.max_players,
    )
    .map_err(|source| ConnectionError::Json { phase, source })?;
    let mut payload = Vec::new();
    codec::write_string(&mut payload, &json);
    write_packet(&mut stream, phase, ids::status::RESPONSE, &payload).await?;
    handle_optional_status_ping(&mut stream).await
}

async fn handle_optional_status_ping(stream: &mut TcpStream) -> Result<(), ConnectionError> {
    let phase = SessionState::Status;
    match read_packet(stream, phase).await {
        Ok(ping) if ping.id == ids::status::PING => {
            write_packet(stream, phase, ids::status::PONG, &ping.data).await?;
        }
        Ok(_) => return Err(protocol_error(phase, "expected status ping")),
        Err(error) if is_connection_closed(&error) => {}
        Err(error) => return Err(error),
    }
    Ok(())
}

async fn handle_login(
    mut stream: TcpStream,
    context: Arc<ServerContext>,
    protocol: i32,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Login;
    if protocol != PROTOCOL_VERSION {
        tracing::warn!(phase = %phase, protocol, "unsupported protocol");
        send_disconnect(&mut stream, "Unsupported Minecraft protocol").await?;
        return Ok(());
    }
    let start = expect_packet(&mut stream, phase, ids::login::START).await?;
    let login = LoginStart::decode(start.data).map_err(|error| codec_error(phase, error))?;
    validate_name(&login.name).map_err(|source| ConnectionError::Profile { phase, source })?;
    let uuid = offline_uuid(&login.name);
    let profile = context
        .player_store
        .load_or_create(uuid, login.name.clone(), player_defaults(&context.config))
        .await
        .map_err(|source| ConnectionError::Player { phase, source })?;
    send_login_success(&mut stream, &login.name, uuid).await?;
    expect_packet(&mut stream, phase, ids::login::ACKNOWLEDGED).await?;
    handle_configuration(&mut stream).await?;
    context.players.fetch_add(1, Ordering::Relaxed);
    let is_op = context.config.is_op(&login.name);
    let play_result = handle_play(
        &mut stream,
        PlaySettings {
            max_players: context.config.max_players,
            view_distance: context.config.view_distance,
            simulation_distance: context.config.simulation_distance,
        },
        context.region.clone(),
        context.sessions.clone(),
        profile,
        context.player_store.clone(),
        is_op,
    )
    .await;
    context.players.fetch_sub(1, Ordering::Relaxed);
    play_result
}

fn player_defaults(config: &Config) -> PlayerDefaults {
    PlayerDefaults {
        game_mode: config.default_game_mode,
    }
}

async fn send_disconnect(stream: &mut TcpStream, reason: &str) -> Result<(), ConnectionError> {
    let phase = SessionState::Login;
    let json = serde_json::json!({ "text": reason }).to_string();
    let mut payload = Vec::new();
    codec::write_string(&mut payload, &json);
    write_packet(stream, phase, ids::login::DISCONNECT, &payload).await
}

async fn send_login_success(
    stream: &mut TcpStream,
    name: &str,
    uuid: uuid::Uuid,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Login;
    let payload = login::encode_success(uuid, name);
    write_packet(stream, phase, ids::login::SUCCESS, &payload).await
}
