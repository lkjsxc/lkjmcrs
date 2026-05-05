use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct WorldStorage {
    root: PathBuf,
}

#[derive(Debug, Error)]
pub enum WorldStorageError {
    #[error("storage I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("storage JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported chunk schema version {0}")]
    UnsupportedSchema(u32),
    #[error("chunk file coordinate mismatch")]
    CoordinateMismatch,
    #[error("invalid block state {0}")]
    InvalidState(String),
    #[error("invalid stored block at {0},{1},{2}")]
    InvalidBlock(i32, i32, i32),
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredChunk {
    schema: u32,
    chunk_x: i32,
    chunk_z: i32,
    overrides: Vec<StoredBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredBlock {
    x: i32,
    y: i32,
    z: i32,
    state: String,
}

impl WorldStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn load_chunk(&self, pos: ChunkPos) -> Result<ChunkSnapshot, WorldStorageError> {
        let path = self.chunk_path(pos);
        let Some(bytes) = read_optional(&path)? else {
            return Ok(ChunkSnapshot::flat(pos));
        };
        let stored: StoredChunk = serde_json::from_slice(&bytes)?;
        validate_header(&stored, pos)?;
        let mut chunk = ChunkSnapshot::flat(pos);
        for block in stored.overrides {
            let state = block_state(&block.state)?;
            let pos = BlockPos::new(block.x, block.y, block.z);
            if chunk.set_block(pos, state) != Some(state) {
                return Err(WorldStorageError::InvalidBlock(block.x, block.y, block.z));
            }
        }
        Ok(chunk)
    }

    pub fn save_chunk(&self, chunk: &ChunkSnapshot) -> Result<(), WorldStorageError> {
        let path = self.chunk_path(chunk.pos);
        if chunk.override_count() == 0 {
            remove_optional(&path)?;
            return Ok(());
        }
        let stored = StoredChunk {
            schema: SCHEMA_VERSION,
            chunk_x: chunk.pos.x,
            chunk_z: chunk.pos.z,
            overrides: stored_blocks(chunk),
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serde_json::to_vec_pretty(&stored)?)?;
        Ok(())
    }

    fn chunk_path(&self, pos: ChunkPos) -> PathBuf {
        self.root
            .join("chunks")
            .join(format!("c.{}.{}.json", pos.x, pos.z))
    }
}

fn read_optional(path: &Path) -> Result<Option<Vec<u8>>, WorldStorageError> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn remove_optional(path: &Path) -> Result<(), WorldStorageError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn validate_header(stored: &StoredChunk, pos: ChunkPos) -> Result<(), WorldStorageError> {
    if stored.schema != SCHEMA_VERSION {
        return Err(WorldStorageError::UnsupportedSchema(stored.schema));
    }
    if stored.chunk_x != pos.x || stored.chunk_z != pos.z {
        return Err(WorldStorageError::CoordinateMismatch);
    }
    Ok(())
}

fn stored_blocks(chunk: &ChunkSnapshot) -> Vec<StoredBlock> {
    chunk
        .override_entries()
        .into_iter()
        .map(|(pos, state)| StoredBlock {
            x: pos.x,
            y: pos.y,
            z: pos.z,
            state: state_name(state).to_string(),
        })
        .collect()
}

fn state_name(state: BlockState) -> &'static str {
    match state {
        BlockState::Air => "minecraft:air",
        BlockState::Bedrock => "minecraft:bedrock",
        BlockState::Stone => "minecraft:stone",
        BlockState::Dirt => "minecraft:dirt",
        BlockState::GrassBlock => "minecraft:grass_block",
    }
}

fn block_state(value: &str) -> Result<BlockState, WorldStorageError> {
    match value {
        "minecraft:air" => Ok(BlockState::Air),
        "minecraft:bedrock" => Ok(BlockState::Bedrock),
        "minecraft:stone" => Ok(BlockState::Stone),
        "minecraft:dirt" => Ok(BlockState::Dirt),
        "minecraft:grass_block" => Ok(BlockState::GrassBlock),
        other => Err(WorldStorageError::InvalidState(other.to_string())),
    }
}
