use crate::player::GameMode;
use crate::session::commands::{ServerCommand, parse};

#[test]
fn parses_gamemode_target() {
    assert_eq!(
        parse("gamemode survival Guest").unwrap(),
        ServerCommand::Gamemode {
            mode: GameMode::Survival,
            target: Some("Guest".to_string())
        }
    );
}

#[test]
fn identifies_operator_commands() {
    assert!(!parse("help").unwrap().requires_op());
    assert!(parse("say hello").unwrap().requires_op());
    assert!(parse("damage Guest 1").unwrap().requires_op());
    assert!(parse("setwarp base").unwrap().requires_op());
}

#[test]
fn parses_location_names() {
    assert_eq!(
        parse("sethome Base_1").unwrap(),
        ServerCommand::SetHome("base_1".to_string())
    );
    assert_eq!(
        parse("home").unwrap(),
        ServerCommand::Home("home".to_string())
    );
    assert!(parse("warp bad/name").is_err());
}

#[test]
fn parses_damage_amount() {
    assert_eq!(
        parse("damage Guest 7.5").unwrap(),
        ServerCommand::Damage {
            target: "Guest".to_string(),
            amount: 7.5,
        }
    );
    assert!(parse("damage Guest nan").is_err());
    assert!(parse("damage Guest 0").is_err());
}
