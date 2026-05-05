use crate::world::{BlockPos, BlockState, ChunkPos};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockMutation {
    pub pos: BlockPos,
    pub chunk: ChunkPos,
    pub requested: BlockState,
    pub state: BlockState,
    pub loaded: bool,
    pub changed: bool,
}

impl BlockMutation {
    pub fn accepted(self) -> bool {
        self.loaded && self.state == self.requested
    }
}
