use crate::player::GameMode;
use serde::Deserialize;
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
    #[error("invalid default_game_mode: {0}")]
    DefaultGameMode(String),
    #[error("{0} must be between 2 and 8")]
    DistanceRange(&'static str),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    #[serde(default = "default_bind")]
    bind: String,
    #[serde(default = "default_motd")]
    motd: String,
    #[serde(default = "default_max_players")]
    max_players: usize,
    #[serde(default)]
    online_mode: bool,
    #[serde(default = "default_data_dir")]
    data_dir: PathBuf,
    #[serde(default = "default_game_mode")]
    default_game_mode: String,
    #[serde(default = "default_distance")]
    view_distance: i32,
    simulation_distance: Option<i32>,
    #[serde(default)]
    session_server_url: Option<String>,
    #[serde(default)]
    allow_insecure_session_server: bool,
    #[serde(default)]
    operator_uuids: Vec<Uuid>,
}

impl Config {
    pub fn from_default_path() -> Result<Self, ConfigError> {
        let path = Path::new(DEFAULT_PATH);
        if path.exists() {
            return Self::from_path(path);
        }
        RawConfig::default().validate()
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
        let raw: RawConfig = serde_json::from_str(json)?;
        raw.validate()
    }

    pub fn is_op(&self, uuid: Uuid) -> bool {
        self.operator_uuids.contains(&uuid)
    }
}

impl RawConfig {
    fn validate(self) -> Result<Config, ConfigError> {
        let session_server_url = self
            .session_server_url
            .unwrap_or_else(default_session_server_url);
        if session_server_url.starts_with("http://") && !self.allow_insecure_session_server {
            return Err(ConfigError::InsecureSessionServer);
        }
        let default_game_mode = GameMode::parse(&self.default_game_mode)
            .ok_or(ConfigError::DefaultGameMode(self.default_game_mode))?;
        validate_distance("view_distance", self.view_distance)?;
        let simulation_distance = self.simulation_distance.unwrap_or(self.view_distance);
        validate_distance("simulation_distance", simulation_distance)?;
        Ok(Config {
            bind: self.bind.parse()?,
            motd: self.motd,
            max_players: self.max_players,
            online_mode: self.online_mode,
            data_dir: self.data_dir,
            default_game_mode,
            view_distance: self.view_distance,
            simulation_distance,
            session_server_url,
            allow_insecure_session_server: self.allow_insecure_session_server,
            operator_uuids: self.operator_uuids,
        })
    }
}

fn validate_distance(name: &'static str, value: i32) -> Result<(), ConfigError> {
    if (2..=8).contains(&value) {
        Ok(())
    } else {
        Err(ConfigError::DistanceRange(name))
    }
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            bind: default_bind(),
            motd: default_motd(),
            max_players: default_max_players(),
            online_mode: false,
            data_dir: default_data_dir(),
            default_game_mode: default_game_mode(),
            view_distance: default_distance(),
            simulation_distance: None,
            session_server_url: None,
            allow_insecure_session_server: false,
            operator_uuids: Vec::new(),
        }
    }
}

fn default_bind() -> String {
    "0.0.0.0:25565".to_string()
}

fn default_motd() -> String {
    "lkjmcrs 1.21.11".to_string()
}

fn default_max_players() -> usize {
    100
}

fn default_data_dir() -> PathBuf {
    PathBuf::from("data")
}

fn default_game_mode() -> String {
    "survival".to_string()
}

fn default_distance() -> i32 {
    2
}

fn default_session_server_url() -> String {
    "https://sessionserver.mojang.com".to_string()
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod config_tests;
