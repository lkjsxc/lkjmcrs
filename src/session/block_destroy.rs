use crate::player::{GameMode, Inventory};
use crate::protocol::block_interaction::PlayerAction;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_actions::InteractionResult;
use crate::session::block_rules;
use crate::session::error::ConnectionError;
use crate::session::play_state::PlaySession;
use crate::session::reach::can_reach_block;
use crate::world::{BlockPos, BlockState};
use tokio::time::Instant;

pub(super) async fn handle_player_action(
    action: PlayerAction,
    region: &RegionHandle,
    session: &mut PlaySession,
    pos: BlockPos,
    phase: SessionState,
    game_mode: GameMode,
    inventory: &mut Inventory,
) -> Result<InteractionResult, ConnectionError> {
    if session.dead || !can_reach_block(session, pos) {
        session.abort_mining();
        return reconcile(region, pos, phase).await;
    }
    match action {
        PlayerAction::StartDestroyBlock => {
            start_destroy(region, session, pos, phase, game_mode, inventory).await
        }
        PlayerAction::StopDestroyBlock => {
            stop_destroy(region, session, pos, phase, game_mode, inventory).await
        }
        PlayerAction::AbortDestroyBlock | PlayerAction::Other(_) => {
            session.abort_mining();
            reconcile(region, pos, phase).await
        }
    }
}

async fn start_destroy(
    region: &RegionHandle,
    session: &mut PlaySession,
    pos: BlockPos,
    phase: SessionState,
    game_mode: GameMode,
    inventory: &mut Inventory,
) -> Result<InteractionResult, ConnectionError> {
    let state = block_rules::current_block(region, pos, phase)
        .await?
        .unwrap_or(BlockState::Air);
    if !crate::session::mining::can_start_mining(state) {
        session.abort_mining();
        return Ok(current_result(pos, state));
    }
    if crate::session::mining::required_break_time(state, game_mode).is_zero() {
        return block_rules::break_block(region, pos, phase, game_mode, inventory).await;
    }
    session.start_mining(pos, state, Instant::now());
    Ok(current_result(pos, state))
}

async fn stop_destroy(
    region: &RegionHandle,
    session: &mut PlaySession,
    pos: BlockPos,
    phase: SessionState,
    game_mode: GameMode,
    inventory: &mut Inventory,
) -> Result<InteractionResult, ConnectionError> {
    if session.stop_mining(pos, Instant::now()) {
        return block_rules::break_block(region, pos, phase, game_mode, inventory).await;
    }
    reconcile(region, pos, phase).await
}

async fn reconcile(
    region: &RegionHandle,
    pos: BlockPos,
    phase: SessionState,
) -> Result<InteractionResult, ConnectionError> {
    let state = block_rules::current_block(region, pos, phase).await?;
    Ok(InteractionResult {
        pos,
        state: state.unwrap_or(BlockState::Air),
        broadcast: None,
        spawned_item: None,
    })
}

fn current_result(pos: BlockPos, state: BlockState) -> InteractionResult {
    InteractionResult {
        pos,
        state,
        broadcast: None,
        spawned_item: None,
    }
}
