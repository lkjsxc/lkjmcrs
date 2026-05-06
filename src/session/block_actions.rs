use crate::player::PlayerProfile;
use crate::protocol::block_interaction::{self, BlockInteraction};
use crate::protocol::chunk;
use crate::protocol::ids;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_rules;
use crate::session::error::ConnectionError;
use crate::session::inventory_sync;
use crate::session::io::write_packet;
use crate::session::play_state::PlaySession;
use crate::session::reach::can_reach_block;
use crate::session::registry::SessionRegistry;
use crate::world::{BlockPos, BlockState, ChunkPos, DroppedItemEntity};
use tokio::io::AsyncWrite;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct InteractionResult {
    pub(super) pos: BlockPos,
    pub(super) state: BlockState,
    pub(super) broadcast: Option<ChunkPos>,
    pub(super) spawned_item: Option<DroppedItemEntity>,
}

pub async fn handle_block_interaction<W>(
    interaction: BlockInteraction,
    phase: SessionState,
    region: &RegionHandle,
    sessions: &SessionRegistry,
    writer: &mut W,
    session: &PlaySession,
    profile: &mut PlayerProfile,
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
            let before = profile.inventory.clone();
            let target = pos.offset(face);
            let result = if can_reach_block(session, target) {
                block_rules::place_block(
                    region,
                    target,
                    phase,
                    profile.game_mode,
                    &mut profile.inventory,
                )
                .await?
            } else {
                reconcile(region, target, phase).await?
            };
            send_prediction_ack(writer, phase, sequence).await?;
            send_inventory_delta(writer, phase, &before, profile, &result).await?;
            publish_or_reconcile(result, phase, sessions, writer).await?;
        }
        BlockInteraction::PlayerAction {
            action,
            pos,
            face: _,
            sequence,
        } => {
            let before = profile.inventory.clone();
            let result = if can_reach_block(session, pos) {
                block_rules::apply_player_action(
                    region,
                    action,
                    pos,
                    phase,
                    profile.game_mode,
                    &mut profile.inventory,
                )
                .await?
            } else {
                reconcile(region, pos, phase).await?
            };
            send_prediction_ack(writer, phase, sequence).await?;
            send_inventory_delta(writer, phase, &before, profile, &result).await?;
            publish_or_reconcile(result, phase, sessions, writer).await?;
        }
        BlockInteraction::Swing { hand: _ } => {
            tracing::debug!(phase = %phase, "swing packet accepted");
        }
    }
    Ok(())
}

async fn send_inventory_delta<W>(
    writer: &mut W,
    phase: SessionState,
    before: &crate::player::Inventory,
    profile: &PlayerProfile,
    result: &InteractionResult,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if result.broadcast.is_some() {
        inventory_sync::send_changed_slots(writer, phase, before, &profile.inventory).await?;
    }
    Ok(())
}

async fn reconcile(
    region: &RegionHandle,
    pos: BlockPos,
    phase: SessionState,
) -> Result<InteractionResult, ConnectionError> {
    let state = region
        .get_block(pos)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    Ok(InteractionResult {
        pos,
        state: state.unwrap_or(BlockState::Air),
        broadcast: None,
        spawned_item: None,
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
    } else {
        send_block_update(writer, phase, result.pos, result.state).await?;
    }
    if let Some(item) = result.spawned_item {
        sessions.broadcast_item_spawn(item.chunk, item).await;
    }
    Ok(())
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
