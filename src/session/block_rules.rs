use crate::player::{GameMode, Inventory};
use crate::protocol::block_interaction::PlayerAction;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_actions::InteractionResult;
use crate::session::error::ConnectionError;
use crate::world::{BlockPos, BlockState};

const STONE_ITEM: &str = "minecraft:stone";

pub(super) async fn place_block(
    region: &RegionHandle,
    pos: BlockPos,
    phase: SessionState,
    game_mode: GameMode,
    inventory: &mut Inventory,
) -> Result<InteractionResult, ConnectionError> {
    let current = region
        .get_block(pos)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    if current != Some(BlockState::Air) {
        return Ok(reconcile(pos, current));
    }
    if game_mode == GameMode::Survival && inventory.selected_item_count(STONE_ITEM) == 0 {
        return Ok(reconcile(pos, current));
    }
    let result = set_block(region, pos, BlockState::Stone, phase).await?;
    if game_mode == GameMode::Survival && result.broadcast.is_some() {
        inventory.consume_selected(STONE_ITEM);
    }
    Ok(result)
}

pub(super) async fn apply_player_action(
    region: &RegionHandle,
    action: PlayerAction,
    pos: BlockPos,
    phase: SessionState,
    game_mode: GameMode,
    inventory: &mut Inventory,
) -> Result<InteractionResult, ConnectionError> {
    match action {
        PlayerAction::StartDestroyBlock | PlayerAction::StopDestroyBlock => {
            break_block(region, pos, phase, game_mode, inventory).await
        }
        PlayerAction::AbortDestroyBlock | PlayerAction::Other(_) => region
            .get_block(pos)
            .await
            .map(|state| reconcile(pos, state))
            .map_err(|source| ConnectionError::Region { phase, source }),
    }
}

async fn break_block(
    region: &RegionHandle,
    pos: BlockPos,
    phase: SessionState,
    game_mode: GameMode,
    inventory: &mut Inventory,
) -> Result<InteractionResult, ConnectionError> {
    let before = region
        .get_block(pos)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    let result = set_block(region, pos, BlockState::Air, phase).await?;
    if game_mode == GameMode::Survival
        && result.broadcast.is_some()
        && let Some(item_id) = simple_drop(before)
    {
        inventory.add_simple_item(item_id, 1);
    }
    Ok(result)
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

fn reconcile(pos: BlockPos, state: Option<BlockState>) -> InteractionResult {
    InteractionResult {
        pos,
        state: state.unwrap_or(BlockState::Air),
        broadcast: None,
    }
}

fn simple_drop(state: Option<BlockState>) -> Option<&'static str> {
    match state {
        Some(BlockState::Stone) => Some("minecraft:stone"),
        Some(BlockState::Dirt | BlockState::GrassBlock) => Some("minecraft:dirt"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::simple_drop;
    use crate::world::BlockState;

    #[test]
    fn simple_drops_match_contract() {
        assert_eq!(
            simple_drop(Some(BlockState::Stone)),
            Some("minecraft:stone")
        );
        assert_eq!(simple_drop(Some(BlockState::Dirt)), Some("minecraft:dirt"));
        assert_eq!(
            simple_drop(Some(BlockState::GrassBlock)),
            Some("minecraft:dirt")
        );
        assert_eq!(simple_drop(Some(BlockState::Air)), None);
        assert_eq!(simple_drop(Some(BlockState::Bedrock)), None);
    }
}
