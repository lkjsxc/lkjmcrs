use crate::player::location_rows;
use crate::player::schema::{initialize_schema, validate_schema};
use crate::player::store_rows::{load_profile, save_profile};
use crate::player::{NamedLocation, PlayerDefaults, PlayerProfile};
use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

const MAX_HOMES: usize = 16;
const BUSY_TIMEOUT_MS: u64 = 5_000;

#[derive(Debug, Clone)]
pub struct PlayerStore {
    path: PathBuf,
    write_lock: Arc<Mutex<()>>,
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
    #[error("invalid stored selected hotbar slot")]
    InvalidSelectedHotbarSlot,
    #[error("invalid stored location")]
    InvalidLocation,
    #[error("home limit exceeded")]
    HomeLimitExceeded,
    #[error("player storage write lock poisoned")]
    WriteLock,
}

impl PlayerStore {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, PlayerStoreError> {
        fs::create_dir_all(root.as_ref())?;
        let path = root.as_ref().join("players.sqlite3");
        let connection = open_configured(&path)?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        initialize_schema(&connection)?;
        Ok(Self {
            path,
            write_lock: Arc::new(Mutex::new(())),
        })
    }

    pub async fn load_or_create(
        &self,
        uuid: Uuid,
        name: String,
        defaults: PlayerDefaults,
    ) -> Result<PlayerProfile, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_checked(&path)?;
            let mut profile = load_profile(&connection, uuid)?
                .unwrap_or_else(|| PlayerProfile::new_with_defaults(uuid, name.clone(), defaults));
            profile.name = name;
            Ok(profile)
        })
        .await?
    }

    pub async fn save(&self, profile: PlayerProfile) -> Result<(), PlayerStoreError> {
        let path = self.path.clone();
        let write_lock = self.write_lock.clone();
        tokio::task::spawn_blocking(move || {
            let _guard = write_lock.lock().map_err(|_| PlayerStoreError::WriteLock)?;
            let mut connection = open_checked(&path)?;
            save_profile(&mut connection, &profile)
        })
        .await?
    }

    pub async fn set_home(
        &self,
        uuid: Uuid,
        location: NamedLocation,
    ) -> Result<(), PlayerStoreError> {
        let path = self.path.clone();
        let write_lock = self.write_lock.clone();
        tokio::task::spawn_blocking(move || {
            let _guard = write_lock.lock().map_err(|_| PlayerStoreError::WriteLock)?;
            let connection = open_checked(&path)?;
            let exists = location_rows::get_home(&connection, uuid, &location.name)?.is_some();
            if !exists && location_rows::count_homes(&connection, uuid)? >= MAX_HOMES {
                return Err(PlayerStoreError::HomeLimitExceeded);
            }
            location_rows::upsert_home(&connection, uuid, &location)
        })
        .await?
    }

    pub async fn home(
        &self,
        uuid: Uuid,
        name: String,
    ) -> Result<Option<NamedLocation>, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_checked(&path)?;
            location_rows::get_home(&connection, uuid, &name)
        })
        .await?
    }

    pub async fn home_names(&self, uuid: Uuid) -> Result<Vec<String>, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_checked(&path)?;
            location_rows::list_home_names(&connection, uuid)
        })
        .await?
    }

    pub async fn set_warp(
        &self,
        created_by_uuid: Uuid,
        location: NamedLocation,
    ) -> Result<(), PlayerStoreError> {
        let path = self.path.clone();
        let write_lock = self.write_lock.clone();
        tokio::task::spawn_blocking(move || {
            let _guard = write_lock.lock().map_err(|_| PlayerStoreError::WriteLock)?;
            let connection = open_checked(&path)?;
            location_rows::upsert_warp(&connection, created_by_uuid, &location)
        })
        .await?
    }

    pub async fn warp(&self, name: String) -> Result<Option<NamedLocation>, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_checked(&path)?;
            location_rows::get_warp(&connection, &name)
        })
        .await?
    }

    pub async fn warp_names(&self) -> Result<Vec<String>, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_checked(&path)?;
            location_rows::list_warp_names(&connection)
        })
        .await?
    }
}

fn open_checked(path: &Path) -> Result<Connection, PlayerStoreError> {
    let connection = open_configured(path)?;
    validate_schema(&connection)?;
    Ok(connection)
}

fn open_configured(path: &Path) -> Result<Connection, PlayerStoreError> {
    let connection = Connection::open(path)?;
    connection.busy_timeout(std::time::Duration::from_millis(BUSY_TIMEOUT_MS))?;
    Ok(connection)
}
