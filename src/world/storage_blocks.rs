use crate::world::{BlockState, WorldStorageError};

pub(super) fn state_code(state: BlockState) -> u16 {
    match state {
        BlockState::Air => 0,
        BlockState::Bedrock => 1,
        BlockState::Stone => 2,
        BlockState::Dirt => 3,
        BlockState::GrassBlock => 4,
        BlockState::Water => 5,
        BlockState::SpruceLog => 6,
        BlockState::SpruceLeaves => 7,
    }
}

pub(super) fn block_state(code: u16) -> Result<BlockState, WorldStorageError> {
    match code {
        0 => Ok(BlockState::Air),
        1 => Ok(BlockState::Bedrock),
        2 => Ok(BlockState::Stone),
        3 => Ok(BlockState::Dirt),
        4 => Ok(BlockState::GrassBlock),
        5 => Ok(BlockState::Water),
        6 => Ok(BlockState::SpruceLog),
        7 => Ok(BlockState::SpruceLeaves),
        other => Err(WorldStorageError::InvalidState(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::{block_state, state_code};
    use crate::world::BlockState;

    #[test]
    fn wood_states_round_trip() {
        assert_eq!(state_code(BlockState::SpruceLog), 6);
        assert_eq!(state_code(BlockState::SpruceLeaves), 7);
        assert_eq!(block_state(6).unwrap(), BlockState::SpruceLog);
        assert_eq!(block_state(7).unwrap(), BlockState::SpruceLeaves);
    }
}
