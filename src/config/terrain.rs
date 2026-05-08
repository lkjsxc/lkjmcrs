use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerrainGeneratorName {
    Flat,
    Natural,
}

#[derive(Debug, Error)]
#[error("invalid terrain_generator: {0}")]
pub struct TerrainGeneratorError(pub String);

impl TerrainGeneratorName {
    pub fn parse(value: &str) -> Result<Self, TerrainGeneratorError> {
        match value {
            "flat" => Ok(Self::Flat),
            "natural" => Ok(Self::Natural),
            other => Err(TerrainGeneratorError(other.to_string())),
        }
    }
}

pub fn default_terrain_generator() -> String {
    "natural".to_string()
}

pub const fn default_world_seed() -> i64 {
    0
}
