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
pub(in crate::world) const FORMULA_MARKER: &str = "natural-level-water-dense-wood";

pub(in crate::world) use column::{TerrainColumn, terrain_column};
pub(in crate::world) use decorator::{
    TREE_MAX_HEIGHT, TREE_REACH, block_at as decorator_block_at, is_tree_root_at, nearby_wood,
    tree_state_at,
};
#[cfg(test)]
pub(in crate::world) use river::RIVER_LEVEL;
pub(in crate::world) use spawn::spawn_position;
