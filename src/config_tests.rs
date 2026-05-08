use super::{Config, ConfigError, TerrainGeneratorName};
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
    assert_eq!(config.default_game_mode, GameMode::Survival);
    assert_eq!(config.view_distance, 32);
    assert_eq!(config.simulation_distance, 8);
    assert_eq!(config.terrain_generator, TerrainGeneratorName::Natural);
    assert_eq!(config.world_seed, 0);
    assert_eq!(
        config.session_server_url,
        "https://sessionserver.mojang.com"
    );
    assert!(!config.allow_insecure_session_server);
    assert!(config.operator_uuids.is_empty());
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
          "view_distance": 3,
          "simulation_distance": 4,
          "terrain_generator": "flat",
          "world_seed": -42,
          "online_mode": true,
          "operator_uuids": ["00000000-0000-0000-0000-000000000007"]
        }"#,
    )
    .unwrap();
    assert_eq!(config.default_game_mode, GameMode::Survival);
    assert_eq!(config.simulation_distance, 4);
    assert_eq!(config.terrain_generator, TerrainGeneratorName::Flat);
    assert_eq!(config.world_seed, -42);
    assert!(config.online_mode);
    assert!(config.is_op(uuid::Uuid::from_u128(7)));
}

#[test]
fn rejects_unknown_terrain_generator() {
    assert!(matches!(
        Config::from_json(r#"{"terrain_generator":"terra"}"#),
        Err(ConfigError::TerrainGenerator(_))
    ));
}

#[test]
fn rejects_insecure_session_server_without_fixture_allowance() {
    assert!(matches!(
        Config::from_json(r#"{"session_server_url":"http://fixture:25566"}"#),
        Err(ConfigError::InsecureSessionServer)
    ));
    assert!(
        Config::from_json(
            r#"{
              "session_server_url": "http://fixture:25566",
              "allow_insecure_session_server": true
            }"#
        )
        .is_ok()
    );
}

#[test]
fn rejects_invalid_session_server_urls() {
    assert!(matches!(
        Config::from_json(r#"{"session_server_url":"sessionserver"}"#),
        Err(ConfigError::InvalidSessionServerUrl)
    ));
    assert!(matches!(
        Config::from_json(r#"{"session_server_url":"ftp://sessionserver"}"#),
        Err(ConfigError::InvalidSessionServerUrl)
    ));
}

#[test]
fn rejects_range_failures() {
    assert!(Config::from_json(r#"{"starter_items":[]}"#).is_err());
    assert!(Config::from_json(r#"{"view_distance":32}"#).is_ok());
    assert!(matches!(
        Config::from_json(r#"{"view_distance":1}"#),
        Err(ConfigError::DistanceRange("view_distance"))
    ));
    assert!(matches!(
        Config::from_json(r#"{"view_distance":33}"#),
        Err(ConfigError::DistanceRange("view_distance"))
    ));
    assert!(Config::from_json(r#"{"simulation_distance":8}"#).is_ok());
    assert!(matches!(
        Config::from_json(r#"{"simulation_distance":9}"#),
        Err(ConfigError::DistanceRange("simulation_distance"))
    ));
}
