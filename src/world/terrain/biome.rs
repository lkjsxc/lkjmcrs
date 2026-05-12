use super::fields::TerrainFields;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::world) enum BiomeKind {
    Ocean,
    Coast,
    River,
    Plains,
    Forest,
    Highlands,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::world) enum SurfaceKind {
    Grass,
    Dirt,
    Stone,
}

pub(in crate::world) fn biome_for(
    fields: TerrainFields,
    surface_y: i32,
    water_y: Option<i32>,
) -> BiomeKind {
    if water_y.is_some() && fields.land < -0.50 {
        return BiomeKind::Ocean;
    }
    if water_y.is_some() {
        return BiomeKind::River;
    }
    if (-0.50..=-0.10).contains(&fields.land) {
        return BiomeKind::Coast;
    }
    if surface_y >= 106 || fields.ridge > 0.74 {
        return BiomeKind::Highlands;
    }
    if fields.moisture > -0.12 && fields.temperature > -0.45 {
        return BiomeKind::Forest;
    }
    BiomeKind::Plains
}

pub(in crate::world) fn surface_for(biome: BiomeKind, surface_y: i32) -> SurfaceKind {
    match biome {
        BiomeKind::Ocean | BiomeKind::River => SurfaceKind::Dirt,
        BiomeKind::Coast => SurfaceKind::Grass,
        BiomeKind::Highlands if surface_y >= 108 => SurfaceKind::Stone,
        BiomeKind::Highlands => SurfaceKind::Grass,
        BiomeKind::Forest | BiomeKind::Plains => SurfaceKind::Grass,
    }
}
