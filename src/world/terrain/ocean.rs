use super::column::TerrainColumn;
use super::fields::TerrainFields;
use super::river::RIVER_LEVEL;

const OCEAN_START: f64 = -0.50;
const COAST_END: f64 = -0.05;

pub(in crate::world) fn apply_ocean(fields: TerrainFields, base_y: i32) -> TerrainColumn {
    if fields.land < OCEAN_START {
        return TerrainColumn::new(ocean_floor(fields), Some(RIVER_LEVEL));
    }
    if fields.land < COAST_END {
        return TerrainColumn::new(coast_bank(fields, base_y), None);
    }
    TerrainColumn::new(base_y, None)
}

fn ocean_floor(fields: TerrainFields) -> i32 {
    let depth = ((OCEAN_START - fields.land) * 72.0).clamp(2.0, 24.0);
    let shelf = if fields.land > OCEAN_START - 0.08 {
        2.5 + depth * 0.35
    } else {
        depth
    };
    (RIVER_LEVEL as f64 - shelf + fields.detail * 1.8)
        .round()
        .clamp(46.0, (RIVER_LEVEL - 2) as f64) as i32
}

fn coast_bank(fields: TerrainFields, base_y: i32) -> i32 {
    let t = ((fields.land - OCEAN_START) / (COAST_END - OCEAN_START)).clamp(0.0, 1.0);
    let rise = smooth(t) * 9.0 + fields.detail * 1.5;
    let target = (RIVER_LEVEL as f64 + 1.0 + rise).round() as i32;
    base_y.clamp(RIVER_LEVEL + 1, target.max(RIVER_LEVEL + 1))
}

fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}
