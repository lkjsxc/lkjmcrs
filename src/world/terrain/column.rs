use super::river;
use super::surface::surface_height;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::world) struct TerrainColumn {
    pub surface_y: i32,
    pub water_y: Option<i32>,
}

impl TerrainColumn {
    pub(in crate::world) fn is_water(self) -> bool {
        self.water_y.is_some()
    }
}

pub(in crate::world) fn terrain_column(seed: i64, x: i32, z: i32) -> TerrainColumn {
    river::apply_river(seed, x, z, surface_height(seed, x, z))
}
