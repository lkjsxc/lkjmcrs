use crate::world::storage_redb::RedbWorldStore;
use crate::world::{ChunkPos, ChunkSnapshot, TerrainGenerator};
use std::path::PathBuf;
use std::sync::Arc;
#[cfg(test)]
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct WorldStorage {
    generator: TerrainGenerator,
    store: Arc<dyn WorldStore>,
    #[cfg(test)]
    test: TestStorage,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, Default)]
struct TestStorage {
    delay: Option<Duration>,
    fail_saves: bool,
}

#[derive(Debug, Error)]
pub enum WorldStorageError {
    #[error("storage I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("storage redb failed: {0}")]
    Redb(String),
    #[error("storage JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid stored chunk key")]
    InvalidChunkKey,
    #[error("invalid block state {0}")]
    InvalidState(String),
    #[error("invalid stored block at {0},{1},{2}")]
    InvalidBlock(i32, i32, i32),
}

pub trait WorldStore: Send + Sync + std::fmt::Debug {
    fn validate(&self) -> Result<(), WorldStorageError>;
    fn load_chunk(
        &self,
        pos: ChunkPos,
        base: ChunkSnapshot,
    ) -> Result<ChunkSnapshot, WorldStorageError>;
    fn save_chunk(&self, chunk: &ChunkSnapshot) -> Result<(), WorldStorageError>;
}

impl WorldStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self::with_backend(root, TerrainGenerator::flat(), test_storage())
    }

    pub fn with_generator(root: impl Into<PathBuf>, generator: TerrainGenerator) -> Self {
        Self::with_backend(root, generator, test_storage())
    }

    #[cfg(test)]
    pub fn with_delay_for_tests(root: impl Into<PathBuf>, delay: Duration) -> Self {
        Self::with_backend(
            root,
            TerrainGenerator::flat(),
            TestStorage {
                delay: Some(delay),
                fail_saves: false,
            },
        )
    }

    #[cfg(test)]
    pub fn with_save_failure_for_tests(root: impl Into<PathBuf>) -> Self {
        Self::with_backend(
            root,
            TerrainGenerator::flat(),
            TestStorage {
                delay: None,
                fail_saves: true,
            },
        )
    }

    pub fn validate(&self) -> Result<(), WorldStorageError> {
        self.store.validate()
    }

    pub fn load_chunk(&self, pos: ChunkPos) -> Result<ChunkSnapshot, WorldStorageError> {
        self.pause_for_test();
        let base = self.generator.chunk_snapshot(pos);
        self.store.load_chunk(pos, base)
    }

    pub fn save_chunk(&self, chunk: &ChunkSnapshot) -> Result<(), WorldStorageError> {
        self.pause_for_test();
        self.fail_save_for_test()?;
        self.store.save_chunk(chunk)
    }

    fn pause_for_test(&self) {
        #[cfg(test)]
        if let Some(delay) = self.test.delay {
            std::thread::sleep(delay);
        }
    }

    fn fail_save_for_test(&self) -> Result<(), WorldStorageError> {
        #[cfg(test)]
        if self.test.fail_saves {
            return Err(std::io::Error::other("forced save failure").into());
        }
        Ok(())
    }

    fn with_backend(
        root: impl Into<PathBuf>,
        generator: TerrainGenerator,
        test: TestStorage,
    ) -> Self {
        #[cfg(not(test))]
        let _ = test;
        Self {
            generator,
            store: Arc::new(RedbWorldStore::new(root)),
            #[cfg(test)]
            test,
        }
    }
}

pub(super) fn redb_error(error: impl std::fmt::Display) -> WorldStorageError {
    WorldStorageError::Redb(error.to_string())
}

#[cfg(not(test))]
type TestStorage = ();

#[cfg(not(test))]
fn test_storage() -> TestStorage {}

#[cfg(test)]
fn test_storage() -> TestStorage {
    TestStorage::default()
}
