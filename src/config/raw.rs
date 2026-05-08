use super::{Config, ConfigError, TerrainGeneratorName, terrain};
use crate::player::GameMode;
use serde::Deserialize;
use std::path::PathBuf;
use uuid::Uuid;

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
    #[serde(default = "terrain::default_terrain_generator")]
    terrain_generator: String,
    #[serde(default = "terrain::default_world_seed")]
    world_seed: i64,
    #[serde(default)]
    session_server_url: Option<String>,
    #[serde(default)]
    allow_insecure_session_server: bool,
    #[serde(default)]
    operator_uuids: Vec<Uuid>,
}

pub(super) fn parse(json: &str) -> Result<Config, ConfigError> {
    let raw: RawConfig = serde_json::from_str(json)?;
    raw.validate()
}

impl RawConfig {
    fn validate(self) -> Result<Config, ConfigError> {
        let session_server_url = self
            .session_server_url
            .unwrap_or_else(default_session_server_url);
        validate_session_server_url(&session_server_url)?;
        if session_server_url.starts_with("http://") && !self.allow_insecure_session_server {
            return Err(ConfigError::InsecureSessionServer);
        }
        let default_game_mode = GameMode::parse(&self.default_game_mode)
            .ok_or(ConfigError::DefaultGameMode(self.default_game_mode))?;
        validate_distance("view_distance", self.view_distance)?;
        let simulation_distance = self.simulation_distance.unwrap_or(self.view_distance);
        validate_distance("simulation_distance", simulation_distance)?;
        let terrain_generator = TerrainGeneratorName::parse(&self.terrain_generator)?;
        Ok(Config {
            bind: self.bind.parse()?,
            motd: self.motd,
            max_players: self.max_players,
            online_mode: self.online_mode,
            data_dir: self.data_dir,
            default_game_mode,
            view_distance: self.view_distance,
            simulation_distance,
            terrain_generator,
            world_seed: self.world_seed,
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

fn validate_session_server_url(url: &str) -> Result<(), ConfigError> {
    let parsed = reqwest::Url::parse(url).map_err(|_| ConfigError::InvalidSessionServerUrl)?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        _ => Err(ConfigError::InvalidSessionServerUrl),
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
