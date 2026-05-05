use crate::player::GameMode;
use crate::world::{BlockPos, BlockState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayOutbound {
    BlockUpdate { pos: BlockPos, state: BlockState },
    SystemChat { message: String },
    ApplyGameMode { game_mode: GameMode },
    Kick { reason: String },
}
