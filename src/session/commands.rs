use crate::player::GameMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerCommand {
    Help,
    Spawn,
    Say(String),
    Gamemode {
        mode: GameMode,
        target: Option<String>,
    },
    Kick {
        target: String,
        reason: String,
    },
}

pub fn parse(input: &str) -> Result<ServerCommand, &'static str> {
    let trimmed = input.trim().trim_start_matches('/');
    let mut parts = trimmed.split_whitespace();
    match parts.next().unwrap_or("") {
        "help" => Ok(ServerCommand::Help),
        "spawn" => Ok(ServerCommand::Spawn),
        "say" => parse_say(trimmed),
        "gamemode" => parse_gamemode(parts),
        "kick" => parse_kick(trimmed),
        _ => Err("Unknown command"),
    }
}

impl ServerCommand {
    pub const fn requires_op(&self) -> bool {
        matches!(
            self,
            Self::Say(_) | Self::Gamemode { .. } | Self::Kick { .. }
        )
    }
}

fn parse_say(input: &str) -> Result<ServerCommand, &'static str> {
    let message = input.strip_prefix("say").unwrap_or("").trim();
    if message.is_empty() {
        Err("Usage: /say <message>")
    } else {
        Ok(ServerCommand::Say(message.to_string()))
    }
}

fn parse_gamemode<'a>(
    mut parts: impl Iterator<Item = &'a str>,
) -> Result<ServerCommand, &'static str> {
    let mode_text = parts.next().ok_or("Usage: /gamemode <mode> [player]")?;
    let mode = GameMode::parse(mode_text).ok_or("Unknown gamemode")?;
    let target = parts.next().map(ToOwned::to_owned);
    Ok(ServerCommand::Gamemode { mode, target })
}

fn parse_kick(input: &str) -> Result<ServerCommand, &'static str> {
    let rest = input.strip_prefix("kick").unwrap_or("").trim();
    let mut split = rest.splitn(2, char::is_whitespace);
    let target = split.next().filter(|value| !value.is_empty());
    let Some(target) = target else {
        return Err("Usage: /kick <player> [reason]");
    };
    let reason = split.next().unwrap_or("Kicked by an operator").trim();
    Ok(ServerCommand::Kick {
        target: target.to_string(),
        reason: if reason.is_empty() {
            "Kicked by an operator".to_string()
        } else {
            reason.to_string()
        },
    })
}

#[cfg(test)]
mod tests {
    use super::{ServerCommand, parse};
    use crate::player::GameMode;

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
    }
}
