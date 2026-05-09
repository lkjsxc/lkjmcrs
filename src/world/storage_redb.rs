use crate::world::storage::{WorldStore, redb_error};
use crate::world::storage_section_codec::{StoredSection, section_range, section_y};
use crate::world::{ChunkPos, ChunkSnapshot, WorldStorageError};
use redb::{Database, ReadableDatabase, TableDefinition};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const WORLD_DB: &str = "world.redb";
const FORMAT_KEY: &str = "world_storage_schema";
const FORMAT_VALUE: &[u8] = b"lkjmcrs.section_overrides.current";
const META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");
const CHUNK_META: TableDefinition<&str, &[u8]> = TableDefinition::new("chunk_meta");
const CHUNK_SECTIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("chunk_sections");

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
            {
                let mut meta = write.open_table(META).map_err(redb_error)?;
                meta.insert(FORMAT_KEY, FORMAT_VALUE).map_err(redb_error)?;
            }
            write.open_table(CHUNK_META).map_err(redb_error)?;
            write.open_table(CHUNK_SECTIONS).map_err(redb_error)?;
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
        let table = read.open_table(CHUNK_SECTIONS).map_err(redb_error)?;
        let mut chunk = base;
        for y in section_range() {
            let key = section_key(pos, y);
            if let Some(bytes) = table.get(key.as_str()).map_err(redb_error)? {
                StoredSection::decode(bytes.value())?.apply_to(&mut chunk)?;
            }
        }
        Ok(chunk)
    }

    fn save_chunk(&self, chunk: &ChunkSnapshot) -> Result<(), WorldStorageError> {
        let _guard = self
            .write_lock
            .lock()
            .map_err(|_| std::io::Error::other("world storage write lock poisoned"))?;
        let database = self.database()?;
        let sections = sections_from_snapshot(chunk);
        let write = database.begin_write().map_err(redb_error)?;
        {
            let mut table = write.open_table(CHUNK_SECTIONS).map_err(redb_error)?;
            for y in section_range() {
                table
                    .remove(section_key(chunk.pos, y).as_str())
                    .map_err(redb_error)?;
            }
            for (y, stored) in sections {
                if !stored.is_empty() {
                    let bytes = stored.encode()?;
                    table
                        .insert(section_key(chunk.pos, y).as_str(), bytes.as_slice())
                        .map_err(redb_error)?;
                }
            }
        }
        write.commit().map_err(redb_error)?;
        Ok(())
    }
}

fn sections_from_snapshot(chunk: &ChunkSnapshot) -> BTreeMap<i32, StoredSection> {
    let mut grouped = BTreeMap::<i32, Vec<_>>::new();
    for (pos, state) in chunk.override_entries() {
        grouped
            .entry(section_y(pos.y))
            .or_default()
            .push((pos, state));
    }
    grouped
        .into_iter()
        .map(|(y, entries)| (y, StoredSection::from_entries(chunk.pos, y, entries)))
        .collect()
}

fn section_key(pos: ChunkPos, section_y: i32) -> String {
    format!("overworld/{}/{}/{}", pos.x, pos.z, section_y)
}
