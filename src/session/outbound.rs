use crate::player::GameMode;
use crate::world::{BlockPos, BlockState, DroppedItemEntity};

#[derive(Debug, Clone, PartialEq)]
pub enum PlayOutbound {
    BlockUpdate {
        pos: BlockPos,
        state: BlockState,
    },
    SystemChat {
        message: String,
    },
    ApplyGameMode {
        game_mode: GameMode,
    },
    Damage {
        amount: f32,
    },
    SetVitals {
        health: f32,
        hunger: u8,
        saturation: f32,
    },
    Kick {
        reason: String,
    },
    ItemSpawn {
        item: DroppedItemEntity,
    },
    ItemCollect {
        item: DroppedItemEntity,
        collector: i32,
    },
    ItemDestroy {
        entity_id: i32,
    },
}
