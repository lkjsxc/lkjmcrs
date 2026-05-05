use crate::player::GameMode;
use std::{env, net::SocketAddr, path::PathBuf};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: SocketAddr,
    pub motd: String,
    pub max_players: usize,
    pub online_mode: bool,
    pub data_dir: PathBuf,
    pub default_game_mode: GameMode,
    pub survival_starter_stone: u8,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid LKJMCRS_BIND: {0}")]
    Bind(#[from] std::net::AddrParseError),
    #[error("invalid LKJMCRS_MAX_PLAYERS: {0}")]
    MaxPlayers(#[from] std::num::ParseIntError),
    #[error("LKJMCRS_ONLINE_MODE=true is not implemented in this milestone")]
    OnlineMode,
    #[error("invalid LKJMCRS_ONLINE_MODE: {0}")]
    OnlineModeValue(String),
    #[error("invalid LKJMCRS_DEFAULT_GAME_MODE: {0}")]
    DefaultGameMode(String),
    #[error("invalid LKJMCRS_SURVIVAL_STARTER_STONE: {0}")]
    StarterStone(String),
    #[error("LKJMCRS_SURVIVAL_STARTER_STONE must be between 0 and 64")]
    StarterStoneRange,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind = env_value("LKJMCRS_BIND", "0.0.0.0:25565").parse()?;
        let motd = env_value("LKJMCRS_MOTD", "lkjmcrs 1.21.11");
        let max_players = env_value("LKJMCRS_MAX_PLAYERS", "100").parse()?;
        let online_mode = parse_bool(&env_value("LKJMCRS_ONLINE_MODE", "false"))?;
        let data_dir = PathBuf::from(env_value("LKJMCRS_DATA_DIR", "data"));
        let default_game_mode =
            parse_game_mode(&env_value("LKJMCRS_DEFAULT_GAME_MODE", "creative"))?;
        let survival_starter_stone =
            parse_starter_stone(&env_value("LKJMCRS_SURVIVAL_STARTER_STONE", "0"))?;

        if online_mode {
            return Err(ConfigError::OnlineMode);
        }

        Ok(Self {
            bind,
            motd,
            max_players,
            online_mode,
            data_dir,
            default_game_mode,
            survival_starter_stone,
        })
    }
}

fn env_value(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse_bool(value: &str) -> Result<bool, ConfigError> {
    match value {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        other => Err(ConfigError::OnlineModeValue(other.to_string())),
    }
}

fn parse_game_mode(value: &str) -> Result<GameMode, ConfigError> {
    GameMode::parse(value).ok_or_else(|| ConfigError::DefaultGameMode(value.to_string()))
}

fn parse_starter_stone(value: &str) -> Result<u8, ConfigError> {
    let parsed: u8 = value
        .parse()
        .map_err(|_| ConfigError::StarterStone(value.to_string()))?;
    if parsed <= 64 {
        Ok(parsed)
    } else {
        Err(ConfigError::StarterStoneRange)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_bool, parse_game_mode, parse_starter_stone};
    use crate::player::GameMode;

    #[test]
    fn parses_bool_values() {
        assert!(parse_bool("true").unwrap());
        assert!(!parse_bool("false").unwrap());
        assert!(parse_bool("wat").is_err());
    }

    #[test]
    fn parses_profile_defaults() {
        assert_eq!(parse_game_mode("survival").unwrap(), GameMode::Survival);
        assert!(parse_game_mode("adventure").is_err());
        assert_eq!(parse_starter_stone("64").unwrap(), 64);
        assert!(parse_starter_stone("65").is_err());
    }
}
