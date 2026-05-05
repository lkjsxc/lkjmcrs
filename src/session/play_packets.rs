use crate::protocol::block_interaction::{self, BlockInteraction, PlayerAction};
use crate::protocol::chunk;
use crate::protocol::codec::{self, Packet};
use crate::protocol::ids;
use crate::protocol::movement::Movement;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::{codec_error, write_packet};
use crate::session::play_state::PlaySession;
use crate::world::{BlockPos, BlockState};
use std::io::Cursor;
use tokio::io::AsyncWrite;

pub async fn handle_play_packet<W>(
    packet: Packet,
    phase: SessionState,
    session: &mut PlaySession,
    region: &RegionHandle,
    writer: &mut W,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if let Some(movement) = Movement::decode(packet.id, packet.data.clone())
        .map_err(|error| codec_error(phase, error))?
    {
        session.apply_movement(movement);
        tracing::debug!(phase = %phase, packet_id = packet.id, "movement packet applied");
        return Ok(());
    }

    if let Some(interaction) = BlockInteraction::decode(packet.id, packet.data.clone())
        .map_err(|error| codec_error(phase, error))?
    {
        handle_block_interaction(interaction, phase, region, writer).await?;
        return Ok(());
    }

    match packet.id {
        ids::play::SERVERBOUND_TELEPORT_CONFIRM
        | ids::play::SERVERBOUND_CHUNK_BATCH_RECEIVED
        | ids::play::SERVERBOUND_SETTINGS
        | ids::play::SERVERBOUND_PLAYER_LOADED
        | ids::play::SERVERBOUND_PONG => {
            tracing::debug!(phase = %phase, packet_id = packet.id, "play packet accepted");
        }
        ids::play::SERVERBOUND_KEEPALIVE => {
            let id = decode_keepalive(packet.data).map_err(|error| codec_error(phase, error))?;
            if session.keepalive_matches(id) {
                tracing::debug!(phase = %phase, id, "keepalive response accepted");
            } else {
                tracing::warn!(
                    phase = %phase,
                    id,
                    expected = session.last_keepalive_id,
                    "keepalive response id mismatch"
                );
            }
        }
        _ => {
            tracing::debug!(phase = %phase, packet_id = packet.id, "play packet ignored");
        }
    }
    Ok(())
}

async fn handle_block_interaction<W>(
    interaction: BlockInteraction,
    phase: SessionState,
    region: &RegionHandle,
    writer: &mut W,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    match interaction {
        BlockInteraction::UseItemOn {
            hand: _,
            pos,
            face,
            sequence,
        } => {
            let target = pos.offset(face);
            let state = place_fixed_block(region, target, phase).await?;
            send_prediction_result(writer, phase, sequence, target, state).await?;
        }
        BlockInteraction::PlayerAction {
            action,
            pos,
            face: _,
            sequence,
        } => {
            let state = apply_player_action(region, action, pos, phase).await?;
            send_prediction_result(writer, phase, sequence, pos, state).await?;
        }
        BlockInteraction::Swing { hand: _ } => {
            tracing::debug!(phase = %phase, "swing packet accepted");
        }
    }
    Ok(())
}

async fn place_fixed_block(
    region: &RegionHandle,
    pos: BlockPos,
    phase: SessionState,
) -> Result<BlockState, ConnectionError> {
    let current = region
        .get_block(pos)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    if current == Some(BlockState::Air) {
        set_block(region, pos, BlockState::Stone, phase).await
    } else {
        Ok(current.unwrap_or(BlockState::Air))
    }
}

async fn apply_player_action(
    region: &RegionHandle,
    action: PlayerAction,
    pos: BlockPos,
    phase: SessionState,
) -> Result<BlockState, ConnectionError> {
    match action {
        PlayerAction::StartDestroyBlock | PlayerAction::StopDestroyBlock => {
            set_block(region, pos, BlockState::Air, phase).await
        }
        PlayerAction::AbortDestroyBlock | PlayerAction::Other(_) => region
            .get_block(pos)
            .await
            .map(|state| state.unwrap_or(BlockState::Air))
            .map_err(|source| ConnectionError::Region { phase, source }),
    }
}

async fn set_block(
    region: &RegionHandle,
    pos: BlockPos,
    state: BlockState,
    phase: SessionState,
) -> Result<BlockState, ConnectionError> {
    region
        .set_block(pos, state)
        .await
        .map(|state| state.unwrap_or(BlockState::Air))
        .map_err(|source| ConnectionError::Region { phase, source })
}

async fn send_prediction_result<W>(
    writer: &mut W,
    phase: SessionState,
    sequence: i32,
    pos: BlockPos,
    state: BlockState,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::BLOCK_CHANGED_ACK,
        &block_interaction::encode_block_changed_ack(sequence),
    )
    .await?;
    write_packet(
        writer,
        phase,
        ids::play::BLOCK_UPDATE,
        &block_interaction::encode_block_update(pos, chunk::block_state_id(state)),
    )
    .await
}

pub(super) fn decode_keepalive(data: Vec<u8>) -> Result<i64, codec::CodecError> {
    let mut cursor = Cursor::new(data);
    let id = codec::read_i64(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(codec::CodecError::Eof);
    }
    Ok(id)
}
