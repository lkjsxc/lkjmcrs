use crate::world::storage_json::StoredChunk;
use crate::world::{ChunkPos, ChunkSnapshot, TerrainGenerator};
use redb::{Database, ReadableDatabase, TableDefinition};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
#[cfg(test)]
use std::time::Duration;
use thiserror::Error;

const WORLD_DB: &str = "world.redb";
const META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");
const CHUNKS: TableDefinition<&str, &[u8]> = TableDefinition::new("chunks");

#[derive(Debug, Clone)]
pub struct WorldStorage {
    root: PathBuf,
    generator: TerrainGenerator,
    database: Arc<Mutex<Option<Arc<Database>>>>,
    write_lock: Arc<Mutex<()>>,
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

impl WorldStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            generator: TerrainGenerator::flat(),
            database: Arc::new(Mutex::new(None)),
            write_lock: Arc::new(Mutex::new(())),
            #[cfg(test)]
            test: TestStorage::default(),
        }
    }

    pub fn with_generator(root: impl Into<PathBuf>, generator: TerrainGenerator) -> Self {
        Self {
            root: root.into(),
            generator,
            database: Arc::new(Mutex::new(None)),
            write_lock: Arc::new(Mutex::new(())),
            #[cfg(test)]
            test: TestStorage::default(),
        }
    }

    #[cfg(test)]
    pub fn with_delay_for_tests(root: impl Into<PathBuf>, delay: Duration) -> Self {
        Self {
            root: root.into(),
            generator: TerrainGenerator::flat(),
            database: Arc::new(Mutex::new(None)),
            write_lock: Arc::new(Mutex::new(())),
            test: TestStorage {
                delay: Some(delay),
                fail_saves: false,
            },
        }
    }

    #[cfg(test)]
    pub fn with_save_failure_for_tests(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            generator: TerrainGenerator::flat(),
            database: Arc::new(Mutex::new(None)),
            write_lock: Arc::new(Mutex::new(())),
            test: TestStorage {
                delay: None,
                fail_saves: true,
            },
        }
    }

    pub fn validate(&self) -> Result<(), WorldStorageError> {
        self.database().map(|_| ())
    }

    pub fn load_chunk(&self, pos: ChunkPos) -> Result<ChunkSnapshot, WorldStorageError> {
        self.pause_for_test();
        let database = self.database()?;
        let read = database.begin_read().map_err(redb_error)?;
        let table = read.open_table(CHUNKS).map_err(redb_error)?;
        let base = self.generator.chunk_snapshot(pos);
        let Some(bytes) = table.get(chunk_key(pos).as_str()).map_err(redb_error)? else {
            return Ok(base);
        };
        let stored: StoredChunk = serde_json::from_slice(bytes.value())?;
        stored.apply_to(base)
    }

    pub fn save_chunk(&self, chunk: &ChunkSnapshot) -> Result<(), WorldStorageError> {
        self.pause_for_test();
        self.fail_save_for_test()?;
        let _guard = self
            .write_lock
            .lock()
            .map_err(|_| std::io::Error::other("world storage write lock poisoned"))?;
        let database = self.database()?;
        let key = chunk_key(chunk.pos);
        let stored = StoredChunk::from_snapshot(chunk);
        let write = database.begin_write().map_err(redb_error)?;
        {
            let mut table = write.open_table(CHUNKS).map_err(redb_error)?;
            if stored.is_empty() {
                table.remove(key.as_str()).map_err(redb_error)?;
            } else {
                let bytes = serde_json::to_vec(&stored)?;
                table
                    .insert(key.as_str(), bytes.as_slice())
                    .map_err(redb_error)?;
            }
        }
        write.commit().map_err(redb_error)?;
        Ok(())
    }

    fn database(&self) -> Result<Arc<Database>, WorldStorageError> {
        let mut cached = self
            .database
            .lock()
            .map_err(|_| WorldStorageError::Redb("database lock poisoned".to_string()))?;
        if let Some(database) = cached.as_ref() {
            return Ok(database.clone());
        }
        fs::create_dir_all(&self.root)?;
        let database = Database::create(self.root.join(WORLD_DB)).map_err(redb_error)?;
        let write = database.begin_write().map_err(redb_error)?;
        {
            write.open_table(META).map_err(redb_error)?;
            write.open_table(CHUNKS).map_err(redb_error)?;
        }
        write.commit().map_err(redb_error)?;
        let database = Arc::new(database);
        *cached = Some(database.clone());
        Ok(database)
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
}

fn chunk_key(pos: ChunkPos) -> String {
    format!("overworld/{}/{}", pos.x, pos.z)
}

fn redb_error(error: impl std::fmt::Display) -> WorldStorageError {
    WorldStorageError::Redb(error.to_string())
}
