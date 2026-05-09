use super::surface::surface_height;

const SEARCH_RADIUS: i32 = 16;
const SEARCH_STEP: usize = 4;
const MAX_SPAWN_SLOPE: i32 = 4;

pub(in crate::world) fn spawn_position(seed: i64) -> (f64, f64, f64) {
    let (x, z, y) = best_column(seed);
    (x as f64 + 0.5, y as f64 + 1.0, z as f64 + 0.5)
}

fn best_column(seed: i64) -> (i32, i32, i32) {
    let mut best = (0, 0, surface_height(seed, 0, 0));
    let mut best_score = i32::MIN;
    for z in (-SEARCH_RADIUS..=SEARCH_RADIUS).step_by(SEARCH_STEP) {
        for x in (-SEARCH_RADIUS..=SEARCH_RADIUS).step_by(SEARCH_STEP) {
            let y = surface_height(seed, x, z);
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
        .map(|(dx, dz)| (surface_height(seed, x + dx, z + dz) - surface_height(seed, x, z)).abs())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{MAX_SPAWN_SLOPE, max_neighbor_delta, spawn_position, surface_height};

    #[test]
    fn spawn_position_is_seed_stable_and_safe() {
        let first = spawn_position(9001);
        let second = spawn_position(9001);
        assert_eq!(first, second);

        let x = first.0.floor() as i32;
        let y = first.1.floor() as i32 - 1;
        let z = first.2.floor() as i32;
        assert_eq!(surface_height(9001, x, z), y);
        assert!(max_neighbor_delta(9001, x, z) <= MAX_SPAWN_SLOPE);
    }
}
