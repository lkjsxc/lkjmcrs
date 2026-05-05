use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    Survival,
    Creative,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Inventory {
    pub slots: Vec<InventorySlot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventorySlot {
    pub slot: i32,
    pub item_id: String,
    pub count: u8,
    pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vitals {
    pub health: f32,
    pub hunger: u8,
    pub saturation: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub uuid: Uuid,
    pub name: String,
    pub game_mode: GameMode,
    pub position: PlayerPosition,
    pub inventory: Inventory,
    pub vitals: Vitals,
}

impl GameMode {
    pub const fn default_new_player() -> Self {
        Self::Creative
    }

    pub const fn vanilla_id(self) -> i8 {
        match self {
            Self::Survival => 0,
            Self::Creative => 1,
        }
    }

    pub const fn ability_flags(self) -> i8 {
        match self {
            Self::Survival => 0x00,
            Self::Creative => 0x0d,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Survival => "survival",
            Self::Creative => "creative",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "survival" => Some(Self::Survival),
            "creative" => Some(Self::Creative),
            _ => None,
        }
    }
}

impl Default for PlayerPosition {
    fn default() -> Self {
        Self {
            x: 0.5,
            y: 80.0,
            z: 0.5,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Default for Vitals {
    fn default() -> Self {
        Self {
            health: 20.0,
            hunger: 20,
            saturation: 5.0,
        }
    }
}

impl PlayerProfile {
    pub fn new(uuid: Uuid, name: impl Into<String>) -> Self {
        Self {
            uuid,
            name: name.into(),
            game_mode: GameMode::default_new_player(),
            position: PlayerPosition::default(),
            inventory: Inventory::default(),
            vitals: Vitals::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GameMode, PlayerProfile};
    use uuid::Uuid;

    #[test]
    fn new_profile_uses_documented_defaults() {
        let profile = PlayerProfile::new(Uuid::from_u128(7), "Probe");

        assert_eq!(profile.game_mode, GameMode::Creative);
        assert_eq!((profile.position.x, profile.position.y), (0.5, 80.0));
        assert_eq!(profile.vitals.health, 20.0);
        assert!(profile.inventory.slots.is_empty());
    }

    #[test]
    fn game_modes_map_to_vanilla_ids() {
        assert_eq!(GameMode::Survival.vanilla_id(), 0);
        assert_eq!(GameMode::Creative.vanilla_id(), 1);
    }
}
