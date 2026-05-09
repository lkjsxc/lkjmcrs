use crate::world::{BlockState, WorldStorageError};

pub(super) fn state_code(state: BlockState) -> u16 {
    match state {
        BlockState::Air => 0,
        BlockState::Bedrock => 1,
        BlockState::Stone => 2,
        BlockState::Dirt => 3,
        BlockState::GrassBlock => 4,
    }
}

pub(super) fn block_state(code: u16) -> Result<BlockState, WorldStorageError> {
    match code {
        0 => Ok(BlockState::Air),
        1 => Ok(BlockState::Bedrock),
        2 => Ok(BlockState::Stone),
        3 => Ok(BlockState::Dirt),
        4 => Ok(BlockState::GrassBlock),
        other => Err(WorldStorageError::InvalidState(other.to_string())),
    }
}
