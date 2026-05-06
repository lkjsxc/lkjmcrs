use super::{Config, ConfigError};
use crate::player::GameMode;
use std::path::PathBuf;

#[test]
fn json_defaults_match_canon() {
    let config = Config::from_json("{}").unwrap();
    assert_eq!(config.bind.to_string(), "0.0.0.0:25565");
    assert_eq!(config.motd, "lkjmcrs 1.21.11");
    assert_eq!(config.max_players, 100);
    assert!(!config.online_mode);
    assert_eq!(config.data_dir, PathBuf::from("data"));
    assert_eq!(config.default_game_mode, GameMode::Creative);
    assert_eq!(config.survival_starter_stone, 0);
    assert_eq!(config.view_distance, 2);
    assert_eq!(config.simulation_distance, 2);
    assert!(config.ops.is_empty());
}

#[test]
fn parses_explicit_json_config() {
    let config = Config::from_json(
        r#"{
          "bind": "127.0.0.1:25566",
          "motd": "survival",
          "max_players": 20,
          "data_dir": "/data",
          "default_game_mode": "survival",
          "survival_starter_stone": 3,
          "view_distance": 3,
          "simulation_distance": 4,
          "ops": ["Admin"]
        }"#,
    )
    .unwrap();
    assert_eq!(config.default_game_mode, GameMode::Survival);
    assert_eq!(config.survival_starter_stone, 3);
    assert_eq!(config.simulation_distance, 4);
    assert!(config.is_op("admin"));
}

#[test]
fn rejects_range_failures() {
    assert!(matches!(
        Config::from_json(r#"{"survival_starter_stone":65}"#),
        Err(ConfigError::StarterStoneRange)
    ));
    assert!(matches!(
        Config::from_json(r#"{"view_distance":1}"#),
        Err(ConfigError::DistanceRange("view_distance"))
    ));
}
