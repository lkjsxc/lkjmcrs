use crate::player::PlayerProfile;
use crate::player::store_rows::{load_profile, save_profile};
use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

const SCHEMA_VERSION: i32 = 1;

#[derive(Debug, Clone)]
pub struct PlayerStore {
    path: PathBuf,
}

#[derive(Debug, Error)]
pub enum PlayerStoreError {
    #[error("player storage I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("player storage SQLite failed: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("player storage blocking task failed: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("unsupported player schema version {0}")]
    UnsupportedSchema(i32),
    #[error("invalid stored game mode {0}")]
    InvalidGameMode(String),
    #[error("invalid stored inventory slot")]
    InvalidInventorySlot,
}

impl PlayerStore {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, PlayerStoreError> {
        fs::create_dir_all(root.as_ref())?;
        let path = root.as_ref().join("players.sqlite3");
        let connection = Connection::open(&path)?;
        initialize_schema(&connection)?;
        Ok(Self { path })
    }

    pub async fn load_or_create(
        &self,
        uuid: Uuid,
        name: String,
    ) -> Result<PlayerProfile, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_checked(&path)?;
            let mut profile = load_profile(&connection, uuid)?
                .unwrap_or_else(|| PlayerProfile::new(uuid, name.clone()));
            profile.name = name;
            Ok(profile)
        })
        .await?
    }

    pub async fn save(&self, profile: PlayerProfile) -> Result<(), PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = open_checked(&path)?;
            save_profile(&mut connection, &profile)
        })
        .await?
    }
}

fn open_checked(path: &Path) -> Result<Connection, PlayerStoreError> {
    let connection = Connection::open(path)?;
    initialize_schema(&connection)?;
    Ok(connection)
}

fn initialize_schema(connection: &Connection) -> Result<(), PlayerStoreError> {
    let version: i32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version != 0 && version != SCHEMA_VERSION {
        return Err(PlayerStoreError::UnsupportedSchema(version));
    }
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS player_profiles (
          uuid TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          game_mode TEXT NOT NULL,
          x REAL NOT NULL,
          y REAL NOT NULL,
          z REAL NOT NULL,
          yaw REAL NOT NULL,
          pitch REAL NOT NULL,
          health REAL NOT NULL,
          hunger INTEGER NOT NULL,
          saturation REAL NOT NULL
        );
        CREATE TABLE IF NOT EXISTS player_inventory_slots (
          uuid TEXT NOT NULL,
          slot INTEGER NOT NULL,
          item_id TEXT NOT NULL,
          count INTEGER NOT NULL,
          data TEXT,
          PRIMARY KEY (uuid, slot)
        );
        PRAGMA user_version = 1;
        ",
    )?;
    Ok(())
}
