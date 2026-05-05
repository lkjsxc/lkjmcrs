use crate::protocol::block_interaction::{self, BlockInteraction, PlayerAction};
use crate::protocol::chunk;
use crate::protocol::ids;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::registry::SessionRegistry;
use crate::world::{BlockPos, BlockState, ChunkPos};
use tokio::io::AsyncWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InteractionResult {
    pos: BlockPos,
    state: BlockState,
    broadcast: Option<ChunkPos>,
}

pub async fn handle_block_interaction<W>(
    interaction: BlockInteraction,
    phase: SessionState,
    region: &RegionHandle,
    sessions: &SessionRegistry,
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
            let result = place_fixed_block(region, pos.offset(face), phase).await?;
            send_prediction_ack(writer, phase, sequence).await?;
            publish_or_reconcile(result, phase, sessions, writer).await?;
        }
        BlockInteraction::PlayerAction {
            action,
            pos,
            face: _,
            sequence,
        } => {
            let result = apply_player_action(region, action, pos, phase).await?;
            send_prediction_ack(writer, phase, sequence).await?;
            publish_or_reconcile(result, phase, sessions, writer).await?;
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
) -> Result<InteractionResult, ConnectionError> {
    let current = region
        .get_block(pos)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    if current == Some(BlockState::Air) {
        set_block(region, pos, BlockState::Stone, phase).await
    } else {
        Ok(InteractionResult {
            pos,
            state: current.unwrap_or(BlockState::Air),
            broadcast: None,
        })
    }
}

async fn apply_player_action(
    region: &RegionHandle,
    action: PlayerAction,
    pos: BlockPos,
    phase: SessionState,
) -> Result<InteractionResult, ConnectionError> {
    match action {
        PlayerAction::StartDestroyBlock | PlayerAction::StopDestroyBlock => {
            set_block(region, pos, BlockState::Air, phase).await
        }
        PlayerAction::AbortDestroyBlock | PlayerAction::Other(_) => region
            .get_block(pos)
            .await
            .map(|state| InteractionResult {
                pos,
                state: state.unwrap_or(BlockState::Air),
                broadcast: None,
            })
            .map_err(|source| ConnectionError::Region { phase, source }),
    }
}

async fn set_block(
    region: &RegionHandle,
    pos: BlockPos,
    state: BlockState,
    phase: SessionState,
) -> Result<InteractionResult, ConnectionError> {
    let mutation = region
        .set_block(pos, state)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    let broadcast = (mutation.changed && mutation.accepted()).then_some(mutation.chunk);
    Ok(InteractionResult {
        pos: mutation.pos,
        state: mutation.state,
        broadcast,
    })
}

async fn publish_or_reconcile<W>(
    result: InteractionResult,
    phase: SessionState,
    sessions: &SessionRegistry,
    writer: &mut W,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if let Some(chunk) = result.broadcast {
        sessions
            .broadcast_block_update(chunk, result.pos, result.state)
            .await;
        Ok(())
    } else {
        send_block_update(writer, phase, result.pos, result.state).await
    }
}

async fn send_prediction_ack<W>(
    writer: &mut W,
    phase: SessionState,
    sequence: i32,
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
    .await
}

pub async fn send_block_update<W>(
    writer: &mut W,
    phase: SessionState,
    pos: BlockPos,
    state: BlockState,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::BLOCK_UPDATE,
        &block_interaction::encode_block_update(pos, chunk::block_state_id(state)),
    )
    .await
}
