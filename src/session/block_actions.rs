use crate::player::PlayerProfile;
use crate::protocol::block_interaction::{self, BlockInteraction};
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_destroy;
use crate::session::block_packets::{send_block_update, send_prediction_ack};
use crate::session::block_rules;
use crate::session::error::ConnectionError;
use crate::session::inventory_sync;
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
    session: &mut PlaySession,
    profile: &mut PlayerProfile,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    match interaction {
        BlockInteraction::UseItemOn {
            hand,
            pos,
            face,
            sequence,
        } => {
            let before = profile.inventory.clone();
            let target = to_world_pos(pos).offset(to_world_face(face));
            let result = if !session.dead && hand == 0 && can_reach_block(session, target) {
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
            send_block_update(writer, phase, result.pos, result.state).await?;
            send_inventory_delta(writer, phase, &before, profile, &result).await?;
            publish_observers(result, sessions, session.id).await;
        }
        BlockInteraction::PlayerAction {
            action,
            pos,
            face: _,
            sequence,
        } => {
            let before = profile.inventory.clone();
            let pos = to_world_pos(pos);
            let result = block_destroy::handle_player_action(
                action,
                region,
                session,
                pos,
                phase,
                profile.game_mode,
                &mut profile.inventory,
            )
            .await?;
            send_prediction_ack(writer, phase, sequence).await?;
            send_block_update(writer, phase, result.pos, result.state).await?;
            send_inventory_delta(writer, phase, &before, profile, &result).await?;
            publish_observers(result, sessions, session.id).await;
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

async fn publish_observers(
    result: InteractionResult,
    sessions: &SessionRegistry,
    initiator: crate::session::registry::SessionId,
) {
    if let Some(chunk) = result.broadcast {
        sessions
            .broadcast_block_update(chunk, result.pos, result.state, Some(initiator))
            .await;
    }
    if let Some(item) = result.spawned_item {
        sessions.broadcast_item_spawn(item.chunk, item).await;
    }
}

fn to_world_pos(pos: block_interaction::BlockPos) -> BlockPos {
    BlockPos::new(pos.x, pos.y, pos.z)
}

fn to_world_face(face: block_interaction::BlockFace) -> crate::world::BlockFace {
    match face {
        block_interaction::BlockFace::Down => crate::world::BlockFace::Down,
        block_interaction::BlockFace::Up => crate::world::BlockFace::Up,
        block_interaction::BlockFace::North => crate::world::BlockFace::North,
        block_interaction::BlockFace::South => crate::world::BlockFace::South,
        block_interaction::BlockFace::West => crate::world::BlockFace::West,
        block_interaction::BlockFace::East => crate::world::BlockFace::East,
    }
}
