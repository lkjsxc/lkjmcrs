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
