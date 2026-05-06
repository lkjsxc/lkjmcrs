use crate::world::{BlockPos, BlockState, ChunkPos, WorldStorageError};

#[derive(Debug)]
pub(super) struct StoredBlock {
    pub local_x: i32,
    pub y: i32,
    pub local_z: i32,
    pub state: String,
}

impl StoredBlock {
    pub fn global_pos(&self, chunk: ChunkPos) -> BlockPos {
        BlockPos::new(
            chunk.x * 16 + self.local_x,
            self.y,
            chunk.z * 16 + self.local_z,
        )
    }
}

pub(super) fn state_name(state: BlockState) -> &'static str {
    match state {
        BlockState::Air => "minecraft:air",
        BlockState::Bedrock => "minecraft:bedrock",
        BlockState::Stone => "minecraft:stone",
        BlockState::Dirt => "minecraft:dirt",
        BlockState::GrassBlock => "minecraft:grass_block",
    }
}

pub(super) fn block_state(value: &str) -> Result<BlockState, WorldStorageError> {
    match value {
        "minecraft:air" => Ok(BlockState::Air),
        "minecraft:bedrock" => Ok(BlockState::Bedrock),
        "minecraft:stone" => Ok(BlockState::Stone),
        "minecraft:dirt" => Ok(BlockState::Dirt),
        "minecraft:grass_block" => Ok(BlockState::GrassBlock),
        other => Err(WorldStorageError::InvalidState(other.to_string())),
    }
}
