use super::column::terrain_column;

const SEARCH_RADIUS: i32 = 16;
const SEARCH_STEP: usize = 4;
const MAX_SPAWN_SLOPE: i32 = 4;

pub(in crate::world) fn spawn_position(seed: i64) -> (f64, f64, f64) {
    let (x, z, y) = best_column(seed);
    (x as f64 + 0.5, y as f64 + 1.0, z as f64 + 0.5)
}

fn best_column(seed: i64) -> (i32, i32, i32) {
    let mut best = (0, 0, terrain_column(seed, 0, 0).surface_y);
    let mut best_score = i32::MIN;
    for z in (-SEARCH_RADIUS..=SEARCH_RADIUS).step_by(SEARCH_STEP) {
        for x in (-SEARCH_RADIUS..=SEARCH_RADIUS).step_by(SEARCH_STEP) {
            let column = terrain_column(seed, x, z);
            if column.is_water() {
                continue;
            }
            let y = column.surface_y;
            let slope = max_neighbor_delta(seed, x, z);
            if slope > MAX_SPAWN_SLOPE {
                continue;
            }
            let distance = x.abs() + z.abs();
            let score = 120 - slope * 8 - distance / 4 - (y - 80).abs();
            if score > best_score {
                best_score = score;
                best = (x, z, y);
            }
        }
    }
    best
}

fn max_neighbor_delta(seed: i64, x: i32, z: i32) -> i32 {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dx, dz)| {
            (terrain_column(seed, x + dx, z + dz).surface_y - terrain_column(seed, x, z).surface_y)
                .abs()
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{MAX_SPAWN_SLOPE, max_neighbor_delta, spawn_position, terrain_column};

    #[test]
    fn spawn_position_is_seed_stable_and_safe() {
        let first = spawn_position(9001);
        let second = spawn_position(9001);
        assert_eq!(first, second);

        let x = first.0.floor() as i32;
        let y = first.1.floor() as i32 - 1;
        let z = first.2.floor() as i32;
        let column = terrain_column(9001, x, z);
        assert_eq!(column.surface_y, y);
        assert!(!column.is_water());
        assert!(max_neighbor_delta(9001, x, z) <= MAX_SPAWN_SLOPE);
    }
}
