mod raw;
mod terrain;

pub use terrain::TerrainGeneratorName;

use crate::player::GameMode;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

const DEFAULT_PATH: &str = "config/server.json";

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub bind: SocketAddr,
    pub motd: String,
    pub max_players: usize,
    pub online_mode: bool,
    pub data_dir: PathBuf,
    pub default_game_mode: GameMode,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub terrain_generator: TerrainGeneratorName,
    pub world_seed: i64,
    pub session_server_url: String,
    pub allow_insecure_session_server: bool,
    pub operator_uuids: Vec<Uuid>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config {path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("invalid config JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid bind: {0}")]
    Bind(#[from] std::net::AddrParseError),
    #[error("insecure session_server_url requires allow_insecure_session_server=true")]
    InsecureSessionServer,
    #[error("session_server_url must be a valid http or https URL")]
    InvalidSessionServerUrl,
    #[error("invalid default_game_mode: {0}")]
    DefaultGameMode(String),
    #[error(transparent)]
    TerrainGenerator(#[from] terrain::TerrainGeneratorError),
    #[error("{0} is outside its supported distance range")]
    DistanceRange(&'static str),
}

impl Config {
    pub fn from_default_path() -> Result<Self, ConfigError> {
        let path = Path::new(DEFAULT_PATH);
        if path.exists() {
            return Self::from_path(path);
        }
        Self::from_json("{}")
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let json = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_json(&json)
    }

    pub(crate) fn from_json(json: &str) -> Result<Self, ConfigError> {
        raw::parse(json)
    }

    pub fn is_op(&self, uuid: Uuid) -> bool {
        self.operator_uuids.contains(&uuid)
    }
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod config_tests;
