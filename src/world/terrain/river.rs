use super::column::TerrainColumn;
use super::noise;

pub(in crate::world) const RIVER_LEVEL: i32 = 72;

const CHANNEL_WIDTH: f64 = 0.070;
const BANK_WIDTH: f64 = 0.190;

pub(in crate::world) fn apply_river(
    seed: i64,
    x: i32,
    z: i32,
    base: TerrainColumn,
) -> TerrainColumn {
    if base.water_y.is_some() {
        return base;
    }
    let distance = river_distance(seed, x, z);
    if distance > BANK_WIDTH {
        return base;
    }
    if distance <= CHANNEL_WIDTH {
        return TerrainColumn::new(riverbed_y(distance, base.surface_y), Some(RIVER_LEVEL));
    }
    TerrainColumn::new(bank_y(distance, base.surface_y), None)
}

fn riverbed_y(distance: f64, base_y: i32) -> i32 {
    let channel = ((CHANNEL_WIDTH - distance) / CHANNEL_WIDTH).clamp(0.0, 1.0);
    let target = RIVER_LEVEL - 2 - (channel * 2.0).round() as i32;
    base_y.min(target).max(RIVER_LEVEL - 4)
}

fn bank_y(distance: f64, base_y: i32) -> i32 {
    let influence = ((BANK_WIDTH - distance) / BANK_WIDTH).clamp(0.0, 1.0);
    let lowered = base_y - (influence * 22.0).round() as i32;
    let dry_floor = RIVER_LEVEL + 1;
    let terrace_cap = dry_floor
        + ((distance - CHANNEL_WIDTH) / (BANK_WIDTH - CHANNEL_WIDTH) * 10.0).round() as i32;
    lowered
        .max(dry_floor)
        .min(base_y)
        .min(terrace_cap.max(dry_floor))
}

fn river_distance(seed: i64, x: i32, z: i32) -> f64 {
    let warp_x = (noise::fbm(seed ^ 0x5511, x, z, 96, 2) * 12.0).round() as i32;
    let warp_z = (noise::fbm(seed ^ 0x1155, x, z, 96, 2) * 12.0).round() as i32;
    let valley = noise::fbm(seed ^ 0x6688, x + warp_x, z + warp_z, 112, 4).abs();
    let gate = noise::fbm(seed ^ 0x8866, x, z, 288, 2);
    if gate < -0.55 { 1.0 } else { valley }
}

#[cfg(test)]
mod tests {
    use super::{RIVER_LEVEL, TerrainColumn, apply_river};

    #[test]
    fn river_columns_are_seed_stable() {
        let a = apply_river(424242, 18, -31, TerrainColumn::new(78, None));
        let b = apply_river(424242, 18, -31, TerrainColumn::new(78, None));

        assert_eq!(a, b);
    }

    #[test]
    fn water_never_sits_below_solid_surface() {
        for z in -64..64 {
            for x in -64..64 {
                let column = apply_river(424242, x, z, TerrainColumn::new(80, None));
                if let Some(water_y) = column.water_y {
                    assert!(column.surface_y < water_y);
                    assert_eq!(water_y, RIVER_LEVEL);
                }
            }
        }
    }
}
