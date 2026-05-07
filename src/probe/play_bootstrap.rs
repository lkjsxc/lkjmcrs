use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::inventory_packets;
use crate::probe::inventory_packets::PlayerInventorySlot;
use crate::probe::live_play;
use crate::probe::persistence;
use crate::probe::registry_assert;
use crate::probe::validation::{
    LoginPacket, PositionPacket, decode_login_packet, decode_position_packet,
    validate_chunk_batch_finished, validate_chunk_radius, validate_game_state_change,
    validate_known_packs,
};
use crate::probe::vitals_packets::HealthState;
use crate::protocol::configuration;
use crate::protocol::{codec, ids, play};
use tokio::io::{AsyncRead, AsyncWrite};

type PlayBootstrap = (
    LoginPacket,
    i32,
    Vec<PlayerInventorySlot>,
    HealthState,
    PositionPacket,
);

pub(super) async fn complete_configuration<S>(
    stream: &mut S,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
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
    registry_assert::expect_configuration_registries(stream).await?;
    let features = super::expect(stream, ids::config::FEATURE_FLAGS, "feature flags").await?;
    if features.data != configuration::encode_enabled_features() {
        return Err(Box::new(ProbeError::Phase("feature flags payload")));
    }
    super::expect(stream, ids::config::FINISH, "finish config").await?;
    codec::write_packet(stream, ids::config::FINISH, &[]).await?;
    Ok(())
}

pub(super) async fn complete_play_bootstrap<S>(
    stream: &mut S,
    expected_block: Option<i32>,
) -> Result<PlayBootstrap, Box<dyn std::error::Error>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let login = super::expect(stream, ids::play::LOGIN, "play login").await?;
    let login = decode_login_packet(login.data)?;
    super::expect(stream, ids::play::DEFAULT_SPAWN_POSITION, "spawn").await?;
    super::expect(stream, ids::play::SET_TIME, "time").await?;
    super::expect(stream, ids::play::PLAYER_ABILITIES, "abilities").await?;
    let health = super::vitals_packets::expect_update_health(stream).await?;
    super::expect(stream, ids::play::DECLARE_COMMANDS, "declare commands").await?;
    let selected_hotbar_slot = inventory_packets::expect_held_item_slot(stream).await?;
    let inventory_slots = inventory_packets::expect_player_inventory(stream).await?;
    let game_state_change =
        super::expect(stream, ids::play::GAME_STATE_CHANGE, "chunk readiness").await?;
    validate_game_state_change(game_state_change.data)?;
    super::expect(stream, ids::play::CHUNK_CACHE_CENTER, "chunk center").await?;
    let radius = super::expect(stream, ids::play::CHUNK_CACHE_RADIUS, "chunk radius").await?;
    let chunk_count = validate_chunk_radius(radius.data)?;
    expect_chunks(stream, chunk_count, expected_block).await?;
    let position = super::expect(stream, ids::play::PLAYER_POSITION, "position").await?;
    let initial_position = decode_position_packet(position.data)?;
    confirm_and_keepalive(stream).await?;
    Ok((
        login,
        selected_hotbar_slot,
        inventory_slots,
        health,
        initial_position,
    ))
}

async fn expect_chunks<S>(
    stream: &mut S,
    chunk_count: usize,
    expected_block: Option<i32>,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    super::expect(stream, ids::play::CHUNK_BATCH_START, "chunk batch").await?;
    let mut observed = None;
    for _ in 0..chunk_count {
        let chunk_packet =
            super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "level chunk").await?;
        let persisted = expected_block
            .is_some()
            .then(|| persistence::target_block_state(chunk_packet.data.clone()))
            .transpose()?
            .flatten();
        if persisted.is_none() {
            chunk::validate_level_chunk_with_light(chunk_packet.data)?;
        }
        observed = observed.or(persisted);
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

async fn confirm_and_keepalive<S>(stream: &mut S) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
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
