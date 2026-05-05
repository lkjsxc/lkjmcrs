use std::{env, net::SocketAddr};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: SocketAddr,
    pub motd: String,
    pub max_players: usize,
    pub online_mode: bool,
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
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind = env_value("LKJMCRS_BIND", "0.0.0.0:25565").parse()?;
        let motd = env_value("LKJMCRS_MOTD", "lkjmcrs 1.21.11");
        let max_players = env_value("LKJMCRS_MAX_PLAYERS", "100").parse()?;
        let online_mode = parse_bool(&env_value("LKJMCRS_ONLINE_MODE", "false"))?;

        if online_mode {
            return Err(ConfigError::OnlineMode);
        }

        Ok(Self {
            bind,
            motd,
            max_players,
            online_mode,
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

#[cfg(test)]
mod tests {
    use super::parse_bool;

    #[test]
    fn parses_bool_values() {
        assert!(parse_bool("true").unwrap());
        assert!(!parse_bool("false").unwrap());
        assert!(parse_bool("wat").is_err());
    }
}
