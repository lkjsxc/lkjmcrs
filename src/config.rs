use crate::player::GameMode;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use thiserror::Error;

const SCHEMA: &str = "lkjmcrs.config";

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub bind: SocketAddr,
    pub motd: String,
    pub max_players: usize,
    pub online_mode: bool,
    pub data_dir: PathBuf,
    pub default_game_mode: GameMode,
    pub survival_starter_stone: u8,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub ops: Vec<String>,
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
    #[error("unsupported config schema: {0}")]
    Schema(String),
    #[error("invalid bind: {0}")]
    Bind(#[from] std::net::AddrParseError),
    #[error("online_mode=true is not implemented in this milestone")]
    OnlineMode,
    #[error("invalid default_game_mode: {0}")]
    DefaultGameMode(String),
    #[error("survival_starter_stone must be between 0 and 64")]
    StarterStoneRange,
    #[error("{0} must be between 2 and 8")]
    DistanceRange(&'static str),
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    #[serde(default = "default_schema")]
    schema: String,
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
    #[serde(default)]
    survival_starter_stone: u8,
    #[serde(default = "default_distance")]
    view_distance: i32,
    simulation_distance: Option<i32>,
    #[serde(default)]
    ops: Vec<String>,
}

impl Config {
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

    pub fn is_op(&self, name: &str) -> bool {
        self.ops.iter().any(|op| op.eq_ignore_ascii_case(name))
    }
}

impl RawConfig {
    fn validate(self) -> Result<Config, ConfigError> {
        if self.schema != SCHEMA {
            return Err(ConfigError::Schema(self.schema));
        }
        if self.online_mode {
            return Err(ConfigError::OnlineMode);
        }
        let default_game_mode = GameMode::parse(&self.default_game_mode)
            .ok_or(ConfigError::DefaultGameMode(self.default_game_mode))?;
        if self.survival_starter_stone > 64 {
            return Err(ConfigError::StarterStoneRange);
        }
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
            survival_starter_stone: self.survival_starter_stone,
            view_distance: self.view_distance,
            simulation_distance,
            ops: self.ops,
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

fn default_schema() -> String {
    SCHEMA.to_string()
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
    "creative".to_string()
}

fn default_distance() -> i32 {
    2
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod config_tests;
