use super::column::TerrainColumn;
use super::noise;

pub(in crate::world) const RIVER_LEVEL: i32 = 63;

const CHANNEL_WIDTH: f64 = 0.075;
const BANK_WIDTH: f64 = 0.155;

pub(in crate::world) fn apply_river(seed: i64, x: i32, z: i32, base_y: i32) -> TerrainColumn {
    let distance = river_distance(seed, x, z);
    if distance > BANK_WIDTH {
        return TerrainColumn {
            surface_y: base_y,
            water_y: None,
        };
    }

    let influence = ((BANK_WIDTH - distance) / BANK_WIDTH).clamp(0.0, 1.0);
    let bank_cut = (influence * 5.0).round() as i32;
    let mut surface_y = base_y - bank_cut;
    if distance <= CHANNEL_WIDTH {
        let channel_cut = ((CHANNEL_WIDTH - distance) / CHANNEL_WIDTH * 4.0).round() as i32;
        surface_y = surface_y.min(RIVER_LEVEL - 1 - channel_cut);
    }

    let water_y = (surface_y < RIVER_LEVEL && distance <= CHANNEL_WIDTH).then_some(RIVER_LEVEL);
    TerrainColumn { surface_y, water_y }
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
    use super::{RIVER_LEVEL, apply_river};

    #[test]
    fn river_columns_are_seed_stable() {
        let a = apply_river(424242, 18, -31, 78);
        let b = apply_river(424242, 18, -31, 78);

        assert_eq!(a, b);
    }

    #[test]
    fn water_never_sits_below_solid_surface() {
        for z in -64..64 {
            for x in -64..64 {
                let column = apply_river(424242, x, z, 80);
                if let Some(water_y) = column.water_y {
                    assert!(column.surface_y < water_y);
                    assert_eq!(water_y, RIVER_LEVEL);
                }
            }
        }
    }
}
