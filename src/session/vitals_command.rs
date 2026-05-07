use crate::session::commands::ServerCommand;

pub fn parse_vitals<'a>(
    mut parts: impl Iterator<Item = &'a str>,
) -> Result<ServerCommand, &'static str> {
    let target = parts.next().ok_or(USAGE)?;
    let health = parse_f32(parts.next().ok_or(USAGE)?, "Health")?;
    let hunger = parse_hunger(parts.next().ok_or(USAGE)?)?;
    let saturation = parse_f32(parts.next().ok_or(USAGE)?, "Saturation")?;
    if parts.next().is_some() {
        return Err(USAGE);
    }
    Ok(ServerCommand::Vitals {
        target: target.to_string(),
        health,
        hunger,
        saturation,
    })
}

const USAGE: &str = "Usage: /vitals <player> <health> <hunger> <saturation>";

fn parse_f32(value: &str, name: &'static str) -> Result<f32, &'static str> {
    let parsed = value.parse::<f32>().map_err(|_| USAGE)?;
    if parsed.is_finite() && (0.0..=20.0).contains(&parsed) {
        Ok(parsed)
    } else if name == "Health" {
        Err("Health must be finite from 0 through 20")
    } else {
        Err("Saturation must be finite from 0 through 20")
    }
}

fn parse_hunger(value: &str) -> Result<u8, &'static str> {
    let parsed = value
        .parse::<u8>()
        .map_err(|_| "Hunger must be 0 through 20")?;
    if parsed <= 20 {
        Ok(parsed)
    } else {
        Err("Hunger must be 0 through 20")
    }
}
