use crate::world::storage::{WorldStore, redb_error};
use crate::world::storage_json::StoredChunk;
use crate::world::{ChunkPos, ChunkSnapshot, WorldStorageError};
use redb::{Database, ReadableDatabase, TableDefinition};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const WORLD_DB: &str = "world.redb";
const META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");
const CHUNKS: TableDefinition<&str, &[u8]> = TableDefinition::new("chunks");

#[derive(Debug)]
pub(super) struct RedbWorldStore {
    root: PathBuf,
    database: Arc<Mutex<Option<Arc<Database>>>>,
    write_lock: Arc<Mutex<()>>,
}

impl RedbWorldStore {
    pub(super) fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            database: Arc::new(Mutex::new(None)),
            write_lock: Arc::new(Mutex::new(())),
        }
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
}

impl WorldStore for RedbWorldStore {
    fn validate(&self) -> Result<(), WorldStorageError> {
        self.database().map(|_| ())
    }

    fn load_chunk(
        &self,
        pos: ChunkPos,
        base: ChunkSnapshot,
    ) -> Result<ChunkSnapshot, WorldStorageError> {
        let database = self.database()?;
        let read = database.begin_read().map_err(redb_error)?;
        let table = read.open_table(CHUNKS).map_err(redb_error)?;
        let Some(bytes) = table.get(chunk_key(pos).as_str()).map_err(redb_error)? else {
            return Ok(base);
        };
        let stored: StoredChunk = serde_json::from_slice(bytes.value())?;
        stored.apply_to(base)
    }

    fn save_chunk(&self, chunk: &ChunkSnapshot) -> Result<(), WorldStorageError> {
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
}

fn chunk_key(pos: ChunkPos) -> String {
    format!("overworld/{}/{}", pos.x, pos.z)
}
