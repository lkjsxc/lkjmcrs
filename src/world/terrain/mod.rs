mod biome;
mod cave;
mod column;
mod decorator;
mod fields;
mod noise;
mod ocean;
mod river;
mod spawn;
mod surface;

pub(in crate::world) use biome::SurfaceKind;
pub(in crate::world) use cave::carves_air;
pub(in crate::world) const FORMULA_MARKER: &str = "natural-surface-wood";

pub(in crate::world) use column::{TerrainColumn, terrain_column};
pub(in crate::world) use decorator::{block_at as decorator_block_at, nearby_wood};
pub(in crate::world) use spawn::spawn_position;
