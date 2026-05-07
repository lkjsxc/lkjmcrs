use crate::world::storage_blocks::{block_state, state_name};
use crate::world::{BlockPos, ChunkPos, ChunkSnapshot, WorldStorageError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct StoredChunk {
    chunk_x: i32,
    chunk_z: i32,
    overrides: Vec<StoredBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredBlock {
    local_x: u8,
    y: i32,
    local_z: u8,
    state: String,
}

impl StoredChunk {
    pub(super) fn from_snapshot(chunk: &ChunkSnapshot) -> Self {
        Self {
            chunk_x: chunk.pos.x,
            chunk_z: chunk.pos.z,
            overrides: chunk
                .override_entries()
                .into_iter()
                .map(|(pos, state)| StoredBlock {
                    local_x: pos.local_x() as u8,
                    y: pos.y,
                    local_z: pos.local_z() as u8,
                    state: state_name(state).to_string(),
                })
                .collect(),
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.overrides.is_empty()
    }

    pub(super) fn into_snapshot(self, pos: ChunkPos) -> Result<ChunkSnapshot, WorldStorageError> {
        if self.chunk_x != pos.x || self.chunk_z != pos.z {
            return Err(WorldStorageError::InvalidChunkKey);
        }
        let mut chunk = ChunkSnapshot::flat(pos);
        for block in self.overrides {
            let state = block_state(&block.state)?;
            let pos = BlockPos::new(
                pos.x * 16 + i32::from(block.local_x),
                block.y,
                pos.z * 16 + i32::from(block.local_z),
            );
            if chunk.set_block(pos, state) != Some(state) {
                return Err(WorldStorageError::InvalidBlock(
                    i32::from(block.local_x),
                    block.y,
                    i32::from(block.local_z),
                ));
            }
        }
        Ok(chunk)
    }
}
