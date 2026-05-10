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
    if water_y.is_some() && fields.land < -0.48 {
        return BiomeKind::Ocean;
    }
    if water_y.is_some() {
        return BiomeKind::River;
    }
    if (-0.58..=-0.34).contains(&fields.land) {
        return BiomeKind::Coast;
    }
    if surface_y >= 102 || fields.ridge > 0.72 {
        return BiomeKind::Highlands;
    }
    if fields.moisture > -0.05 && fields.temperature > -0.45 {
        return BiomeKind::Forest;
    }
    BiomeKind::Plains
}

pub(in crate::world) fn surface_for(biome: BiomeKind, surface_y: i32) -> SurfaceKind {
    match biome {
        BiomeKind::Ocean | BiomeKind::River | BiomeKind::Coast => SurfaceKind::Dirt,
        BiomeKind::Highlands if surface_y >= 108 => SurfaceKind::Stone,
        BiomeKind::Highlands => SurfaceKind::Grass,
        BiomeKind::Forest | BiomeKind::Plains => SurfaceKind::Grass,
    }
}
