use crate::world::storage_blocks::{StoredBlock, block_state, state_name};
use crate::world::storage_schema::ensure_schema;
use crate::world::{ChunkPos, ChunkSnapshot};
use rusqlite::{Connection, params};
use std::fs;
use std::path::PathBuf;
#[cfg(test)]
use std::time::Duration;
use thiserror::Error;

const BUSY_TIMEOUT_MS: u64 = 5_000;

#[derive(Debug, Clone)]
pub struct WorldStorage {
    root: PathBuf,
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
    #[error("storage SQLite failed: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("unsupported world schema version {0}")]
    UnsupportedSchema(u32),
    #[error("invalid block state {0}")]
    InvalidState(String),
    #[error("invalid stored block at {0},{1},{2}")]
    InvalidBlock(i32, i32, i32),
}

impl WorldStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            #[cfg(test)]
            test: TestStorage::default(),
        }
    }

    #[cfg(test)]
    pub fn with_delay_for_tests(root: impl Into<PathBuf>, delay: Duration) -> Self {
        Self {
            root: root.into(),
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
            test: TestStorage {
                delay: None,
                fail_saves: true,
            },
        }
    }

    pub fn schema_version(&self) -> Result<u32, WorldStorageError> {
        let connection = self.connection()?;
        Ok(connection.query_row("PRAGMA user_version", [], |row| row.get(0))?)
    }

    pub fn validate(&self) -> Result<(), WorldStorageError> {
        self.connection().map(|_| ())
    }

    pub fn load_chunk(&self, pos: ChunkPos) -> Result<ChunkSnapshot, WorldStorageError> {
        self.pause_for_test();
        let connection = self.connection()?;
        let mut chunk = ChunkSnapshot::flat(pos);
        let mut rows = connection.prepare(
            "SELECT local_x, y, local_z, state
             FROM chunk_overrides
             WHERE chunk_x = ?1 AND chunk_z = ?2",
        )?;
        let overrides = rows.query_map(params![pos.x, pos.z], |row| {
            Ok(StoredBlock {
                local_x: row.get(0)?,
                y: row.get(1)?,
                local_z: row.get(2)?,
                state: row.get(3)?,
            })
        })?;
        for row in overrides {
            let block = row?;
            let state = block_state(&block.state)?;
            let global = block.global_pos(pos);
            if chunk.set_block(global, state) != Some(state) {
                return Err(WorldStorageError::InvalidBlock(
                    block.local_x,
                    block.y,
                    block.local_z,
                ));
            }
        }
        Ok(chunk)
    }

    pub fn save_chunk(&self, chunk: &ChunkSnapshot) -> Result<(), WorldStorageError> {
        self.pause_for_test();
        self.fail_save_for_test()?;
        let mut connection = self.connection()?;
        let tx = connection.transaction()?;
        tx.execute(
            "DELETE FROM chunk_overrides WHERE chunk_x = ?1 AND chunk_z = ?2",
            params![chunk.pos.x, chunk.pos.z],
        )?;
        for (pos, state) in chunk.override_entries() {
            tx.execute(
                "INSERT INTO chunk_overrides
                 (chunk_x, chunk_z, local_x, y, local_z, state)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    chunk.pos.x,
                    chunk.pos.z,
                    pos.local_x() as i32,
                    pos.y,
                    pos.local_z() as i32,
                    state_name(state),
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn connection(&self) -> Result<Connection, WorldStorageError> {
        fs::create_dir_all(&self.root)?;
        let connection = Connection::open(self.root.join("world.sqlite3"))?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.busy_timeout(std::time::Duration::from_millis(BUSY_TIMEOUT_MS))?;
        ensure_schema(&connection)?;
        Ok(connection)
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
