use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::live_play;
use crate::probe::persistence;
use crate::probe::validation::{
    validate_chunk_batch_finished, validate_chunk_radius, validate_game_state_change,
    validate_known_packs, validate_login_success, validate_position_packet,
};
use crate::protocol::configuration;
use crate::protocol::types::{LoginStart, NextState};
use crate::protocol::{codec, ids, play};
use tokio::net::TcpStream;
use uuid::Uuid;

pub(super) struct PlayClient {
    pub stream: TcpStream,
}

impl PlayClient {
    pub async fn connect(host: &str, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::connect_with_block(host, name, None).await
    }

    pub async fn connect_with_block(
        host: &str,
        name: &str,
        expected_block: Option<i32>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(host).await?;
        super::send_handshake(&mut stream, host, NextState::Login).await?;
        let login = LoginStart::encode(name, Uuid::from_u128(0));
        codec::write_packet(&mut stream, ids::login::START, &login).await?;
        let success = super::expect(&mut stream, ids::login::SUCCESS, "login success").await?;
        validate_login_success(success.data, name)?;
        codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
        complete_configuration(&mut stream).await?;
        complete_play_bootstrap(&mut stream, expected_block).await?;
        Ok(Self { stream })
    }
}

async fn complete_configuration(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let known = super::expect(
        stream,
        ids::config::SELECT_KNOWN_PACKS,
        "select known packs",
    )
    .await?;
    validate_known_packs(known.data)?;
    codec::write_packet(
        stream,
        ids::config::SERVERBOUND_SELECT_KNOWN_PACKS,
        &configuration::encode_select_known_packs(),
    )
    .await?;
    for _ in 0..configuration::registry_packet_count() {
        super::expect(stream, ids::config::REGISTRY_DATA, "registry data").await?;
    }
    super::expect(stream, ids::config::TAGS, "configuration tags").await?;
    let features = super::expect(stream, ids::config::FEATURE_FLAGS, "feature flags").await?;
    if features.data != configuration::encode_enabled_features() {
        return Err(Box::new(ProbeError::Phase("feature flags payload")));
    }
    super::expect(stream, ids::config::FINISH, "finish config").await?;
    codec::write_packet(stream, ids::config::FINISH, &[]).await?;
    Ok(())
}

async fn complete_play_bootstrap(
    stream: &mut TcpStream,
    expected_block: Option<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    super::expect(stream, ids::play::LOGIN, "play login").await?;
    super::expect(stream, ids::play::DEFAULT_SPAWN_POSITION, "spawn").await?;
    super::expect(stream, ids::play::SET_TIME, "time").await?;
    super::expect(stream, ids::play::PLAYER_ABILITIES, "abilities").await?;
    let game_state_change =
        super::expect(stream, ids::play::GAME_STATE_CHANGE, "chunk readiness").await?;
    validate_game_state_change(game_state_change.data)?;
    super::expect(stream, ids::play::CHUNK_CACHE_CENTER, "chunk center").await?;
    let radius = super::expect(stream, ids::play::CHUNK_CACHE_RADIUS, "chunk radius").await?;
    let chunk_count = validate_chunk_radius(radius.data)?;
    expect_chunks(stream, chunk_count, expected_block).await?;
    let position = super::expect(stream, ids::play::PLAYER_POSITION, "position").await?;
    validate_position_packet(position.data)?;
    confirm_and_keepalive(stream).await
}

async fn expect_chunks(
    stream: &mut TcpStream,
    chunk_count: usize,
    expected_block: Option<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    super::expect(stream, ids::play::CHUNK_BATCH_START, "chunk batch").await?;
    let mut observed = None;
    for _ in 0..chunk_count {
        let chunk_packet =
            super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "level chunk").await?;
        chunk::validate_level_chunk_with_light(chunk_packet.data.clone())?;
        observed = observed.or(persistence::target_block_state(chunk_packet.data)?);
        let light = super::expect(stream, ids::play::UPDATE_LIGHT, "update light").await?;
        chunk::validate_update_light(light.data)?;
    }
    if let Some(expected) = expected_block
        && observed != Some(expected)
    {
        return Err(Box::new(ProbeError::Phase("persisted block state")));
    }
    let finished = super::expect(
        stream,
        ids::play::CHUNK_BATCH_FINISHED,
        "chunk batch finished",
    )
    .await?;
    validate_chunk_batch_finished(finished.data, chunk_count)
}

async fn confirm_and_keepalive(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut confirm = Vec::new();
    codec::write_var_i32(&mut confirm, play::Bootstrap::new(100).teleport_id());
    codec::write_packet(stream, ids::play::SERVERBOUND_TELEPORT_CONFIRM, &confirm).await?;
    live_play::send_position_look(stream).await?;
    let keepalive = super::expect(stream, ids::play::KEEPALIVE, "keepalive").await?;
    let keepalive_id = codec::read_i64(&mut std::io::Cursor::new(keepalive.data))?;
    if keepalive_id != 1 {
        return Err(Box::new(ProbeError::Phase("keepalive id")));
    }
    let mut keepalive_response = Vec::new();
    codec::write_i64(&mut keepalive_response, keepalive_id);
    codec::write_packet(
        stream,
        ids::play::SERVERBOUND_KEEPALIVE,
        &keepalive_response,
    )
    .await?;
    Ok(())
}
