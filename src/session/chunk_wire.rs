use crate::protocol::chunk;
use crate::world::{BlockState, ChunkSnapshot};

pub struct WireChunk<'a>(pub &'a ChunkSnapshot);

impl chunk::ChunkColumn for WireChunk<'_> {
    fn position(&self) -> chunk::ChunkPosition {
        chunk::ChunkPosition {
            x: self.0.pos.x,
            z: self.0.pos.z,
        }
    }

    fn block_state_id_at_local(&self, x: usize, y: i32, z: usize) -> i32 {
        block_state_id(self.0.block_at_local(x, y, z))
    }

    fn heightmap_at_local(&self, x: usize, z: usize) -> u16 {
        self.0.heightmap_at_local(x, z)
    }
}

pub(crate) fn block_state_id(state: BlockState) -> i32 {
    match state {
        BlockState::Air => chunk::AIR_ID,
        BlockState::Bedrock => chunk::BEDROCK_ID,
        BlockState::Stone => chunk::STONE_ID,
        BlockState::Dirt => chunk::DIRT_ID,
        BlockState::GrassBlock => chunk::GRASS_BLOCK_ID,
        BlockState::Water => chunk::WATER_ID,
        BlockState::SpruceLog => chunk::SPRUCE_LOG_ID,
        BlockState::SpruceLeaves => chunk::SPRUCE_LEAVES_ID,
    }
}
