use crate::player::storage_redb;
use crate::player::{NamedLocation, PlayerDefaults, PlayerProfile};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

const MAX_HOMES: usize = 16;
const PLAYER_DB: &str = "players.redb";

#[derive(Debug, Clone)]
pub struct PlayerStore {
    path: PathBuf,
    write_lock: Arc<Mutex<()>>,
}

#[derive(Debug, Error)]
pub enum PlayerStoreError {
    #[error("player storage I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("player storage redb failed: {0}")]
    Redb(String),
    #[error("player storage JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("player storage blocking task failed: {0}")]
    Join(#[from] tokio::task::JoinError),
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
        let store = Self {
            path: root.as_ref().join(PLAYER_DB),
            write_lock: Arc::new(Mutex::new(())),
        };
        storage_redb::open_database(&store.path)?;
        Ok(store)
    }

    pub async fn load_or_create(
        &self,
        uuid: Uuid,
        name: String,
        defaults: PlayerDefaults,
    ) -> Result<PlayerProfile, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let database = storage_redb::open_database(&path)?;
            let mut profile = storage_redb::load_profile(&database, uuid)?
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
            let database = storage_redb::open_database(&path)?;
            storage_redb::save_profile(&database, &profile)
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
            let database = storage_redb::open_database(&path)?;
            let exists = storage_redb::home(&database, uuid, &location.name)?.is_some();
            if !exists && storage_redb::home_names(&database, uuid)?.len() >= MAX_HOMES {
                return Err(PlayerStoreError::HomeLimitExceeded);
            }
            storage_redb::set_home(&database, uuid, &location)
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
            let database = storage_redb::open_database(&path)?;
            storage_redb::home(&database, uuid, &name)
        })
        .await?
    }

    pub async fn home_names(&self, uuid: Uuid) -> Result<Vec<String>, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let database = storage_redb::open_database(&path)?;
            storage_redb::home_names(&database, uuid)
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
            let database = storage_redb::open_database(&path)?;
            storage_redb::set_warp(&database, created_by_uuid, &location)
        })
        .await?
    }

    pub async fn warp(&self, name: String) -> Result<Option<NamedLocation>, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let database = storage_redb::open_database(&path)?;
            storage_redb::warp(&database, &name)
        })
        .await?
    }

    pub async fn warp_names(&self) -> Result<Vec<String>, PlayerStoreError> {
        let path = self.path.clone();
        tokio::task::spawn_blocking(move || {
            let database = storage_redb::open_database(&path)?;
            storage_redb::warp_names(&database)
        })
        .await?
    }
}
