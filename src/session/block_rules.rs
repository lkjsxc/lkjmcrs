use crate::player::{GameMode, Inventory};
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_actions::InteractionResult;
use crate::session::error::ConnectionError;
use crate::world::{BlockPos, BlockState};

const STONE_ITEM: &str = "minecraft:stone";
const DIRT_ITEM: &str = "minecraft:dirt";

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
    let Some((state, item_id)) = placement_state(inventory) else {
        return Ok(reconcile(pos, current));
    };
    let result = set_block(region, pos, state, phase).await?;
    if game_mode == GameMode::Survival && result.broadcast.is_some() {
        inventory.consume_selected(item_id);
    }
    Ok(result)
}

pub(super) async fn current_block(
    region: &RegionHandle,
    pos: BlockPos,
    phase: SessionState,
) -> Result<Option<BlockState>, ConnectionError> {
    region
        .get_block(pos)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })
}

pub(super) async fn break_block(
    region: &RegionHandle,
    pos: BlockPos,
    phase: SessionState,
    game_mode: GameMode,
    _inventory: &mut Inventory,
) -> Result<InteractionResult, ConnectionError> {
    let before = region
        .get_block(pos)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    let mut result = set_block(region, pos, BlockState::Air, phase).await?;
    if game_mode == GameMode::Survival
        && result.broadcast.is_some()
        && let Some(item_id) = simple_drop(before)
    {
        result.spawned_item = Some(
            region
                .spawn_item(pos, item_id, 1)
                .await
                .map_err(|source| ConnectionError::Region { phase, source })?,
        );
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
        spawned_item: None,
    })
}

fn reconcile(pos: BlockPos, state: Option<BlockState>) -> InteractionResult {
    InteractionResult {
        pos,
        state: state.unwrap_or(BlockState::Air),
        broadcast: None,
        spawned_item: None,
    }
}

pub(super) fn simple_drop(state: Option<BlockState>) -> Option<&'static str> {
    match state {
        Some(BlockState::Stone) => Some("minecraft:stone"),
        Some(BlockState::Dirt | BlockState::GrassBlock) => Some("minecraft:dirt"),
        _ => None,
    }
}

pub(super) fn placement_state(inventory: &Inventory) -> Option<(BlockState, &'static str)> {
    match inventory.selected_item_id()? {
        STONE_ITEM => Some((BlockState::Stone, STONE_ITEM)),
        DIRT_ITEM => Some((BlockState::Dirt, DIRT_ITEM)),
        _ => None,
    }
}
