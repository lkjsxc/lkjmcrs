use crate::player::PlayerPosition;
use serde::{Deserialize, Serialize};

pub const OVERWORLD: &str = "minecraft:overworld";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedLocation {
    pub name: String,
    pub world: String,
    pub position: PlayerPosition,
}

impl NamedLocation {
    pub fn overworld(name: String, position: PlayerPosition) -> Self {
        Self {
            name,
            world: OVERWORLD.to_string(),
            position,
        }
    }
}
