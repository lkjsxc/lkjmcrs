use super::column::TerrainColumn;
use super::fields::TerrainFields;
use super::river::RIVER_LEVEL;

pub(in crate::world) fn apply_ocean(fields: TerrainFields, base_y: i32) -> TerrainColumn {
    let sea_floor = ocean_floor(fields, base_y);
    let water_y = (sea_floor < RIVER_LEVEL && fields.land < -0.48).then_some(RIVER_LEVEL);
    TerrainColumn::new(sea_floor, water_y)
}

fn ocean_floor(fields: TerrainFields, base_y: i32) -> i32 {
    if fields.land >= -0.48 {
        return base_y;
    }
    let depth = ((-0.48 - fields.land) * 56.0).round() as i32;
    let shelf = if fields.land > -0.58 { 2 } else { depth };
    (RIVER_LEVEL - 2 - shelf + (fields.detail * 2.0).round() as i32).clamp(42, base_y)
}
