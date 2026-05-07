use crate::player::GameMode;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerCommand {
    Help,
    Spawn,
    SetHome(String),
    Home(String),
    Homes,
    SetWarp(String),
    Warp(String),
    Warps,
    Say(String),
    Gamemode {
        mode: GameMode,
        target: Option<String>,
    },
    Damage {
        target: String,
        amount: f32,
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
        "sethome" => {
            parse_optional_location(parts, ServerCommand::SetHome, "Usage: /sethome [name]")
        }
        "home" => parse_optional_location(parts, ServerCommand::Home, "Usage: /home [name]"),
        "homes" => parse_no_args(parts, ServerCommand::Homes, "Usage: /homes"),
        "setwarp" => {
            parse_required_location(parts, ServerCommand::SetWarp, "Usage: /setwarp <name>")
        }
        "warp" => parse_required_location(parts, ServerCommand::Warp, "Usage: /warp <name>"),
        "warps" => parse_no_args(parts, ServerCommand::Warps, "Usage: /warps"),
        "say" => parse_say(trimmed),
        "gamemode" => parse_gamemode(parts),
        "damage" => parse_damage(parts),
        "kick" => parse_kick(trimmed),
        _ => Err("Unknown command"),
    }
}

impl ServerCommand {
    pub const fn requires_op(&self) -> bool {
        matches!(
            self,
            Self::SetWarp(_)
                | Self::Say(_)
                | Self::Gamemode { .. }
                | Self::Damage { .. }
                | Self::Kick { .. }
        )
    }
}

pub fn normalize_location_name(raw: &str) -> Result<String, &'static str> {
    let name = raw.trim().to_ascii_lowercase();
    if name.is_empty() || name.len() > 32 {
        return Err("Location names must be 1-32 characters");
    }
    if name.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
    }) {
        Ok(name)
    } else {
        Err("Location names may contain a-z, 0-9, _, and -")
    }
}

fn parse_no_args<'a>(
    mut parts: impl Iterator<Item = &'a str>,
    command: ServerCommand,
    usage: &'static str,
) -> Result<ServerCommand, &'static str> {
    if parts.next().is_some() {
        Err(usage)
    } else {
        Ok(command)
    }
}

fn parse_optional_location<'a>(
    mut parts: impl Iterator<Item = &'a str>,
    build: fn(String) -> ServerCommand,
    usage: &'static str,
) -> Result<ServerCommand, &'static str> {
    let name = normalize_location_name(parts.next().unwrap_or("home"))?;
    if parts.next().is_some() {
        return Err(usage);
    }
    Ok(build(name))
}

fn parse_required_location<'a>(
    mut parts: impl Iterator<Item = &'a str>,
    build: fn(String) -> ServerCommand,
    usage: &'static str,
) -> Result<ServerCommand, &'static str> {
    let Some(name) = parts.next() else {
        return Err(usage);
    };
    let name = normalize_location_name(name)?;
    if parts.next().is_some() {
        return Err(usage);
    }
    Ok(build(name))
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

fn parse_damage<'a>(
    mut parts: impl Iterator<Item = &'a str>,
) -> Result<ServerCommand, &'static str> {
    let target = parts.next().ok_or("Usage: /damage <player> <amount>")?;
    let amount_text = parts.next().ok_or("Usage: /damage <player> <amount>")?;
    if parts.next().is_some() {
        return Err("Usage: /damage <player> <amount>");
    }
    let amount = amount_text
        .parse::<f32>()
        .map_err(|_| "Invalid damage amount")?;
    if !amount.is_finite() || amount <= 0.0 || amount > 1000.0 {
        return Err("Damage amount must be positive and at most 1000");
    }
    Ok(ServerCommand::Damage {
        target: target.to_string(),
        amount,
    })
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
