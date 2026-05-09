use super::surface::surface_height;

pub(in crate::world) fn spawn_position(seed: i64) -> (f64, f64, f64) {
    let (x, z, y) = best_column(seed);
    (x as f64 + 0.5, y as f64 + 1.0, z as f64 + 0.5)
}

fn best_column(seed: i64) -> (i32, i32, i32) {
    let mut best = (0, 0, surface_height(seed, 0, 0));
    let mut best_score = i32::MIN;
    for z in (-64..=64).step_by(4) {
        for x in (-64..=64).step_by(4) {
            let y = surface_height(seed, x, z);
            let slope = max_neighbor_delta(seed, x, z);
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
