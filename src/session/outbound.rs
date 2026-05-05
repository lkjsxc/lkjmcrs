use crate::world::{BlockPos, BlockState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayOutbound {
    BlockUpdate { pos: BlockPos, state: BlockState },
}
