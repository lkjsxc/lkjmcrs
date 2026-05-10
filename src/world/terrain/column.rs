use super::biome::{BiomeKind, SurfaceKind, biome_for, surface_for};
use super::fields::sample_fields;
use super::surface::surface_height;
use super::{ocean, river};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::world) struct TerrainColumn {
    pub surface_y: i32,
    pub water_y: Option<i32>,
    pub biome: BiomeKind,
    pub surface: SurfaceKind,
}

impl TerrainColumn {
    pub(in crate::world) const fn new(surface_y: i32, water_y: Option<i32>) -> Self {
        Self {
            surface_y,
            water_y,
            biome: BiomeKind::Plains,
            surface: SurfaceKind::Grass,
        }
    }

    pub(in crate::world) fn is_water(self) -> bool {
        self.water_y.is_some()
    }
}

pub(in crate::world) fn terrain_column(seed: i64, x: i32, z: i32) -> TerrainColumn {
    let fields = sample_fields(seed, x, z);
    let base = ocean::apply_ocean(fields, surface_height(fields));
    let mut column = river::apply_river(seed, x, z, base);
    column.biome = biome_for(fields, column.surface_y, column.water_y);
    column.surface = surface_for(column.biome, column.surface_y);
    column
}
