use super::{decorator_block_at, nearby_wood, terrain_column};

const COARSE_RADIUS: i32 = 256;
const COARSE_STEP: usize = 8;
const REFINE_RADIUS: i32 = 8;
const MAX_SPAWN_SLOPE: i32 = 5;

pub(in crate::world) fn spawn_position(seed: i64) -> (f64, f64, f64) {
    let (x, z, y) = best_column(seed);
    (x as f64 + 0.5, y as f64 + 1.0, z as f64 + 0.5)
}

fn best_column(seed: i64) -> (i32, i32, i32) {
    let mut best = coarse_best_column(seed);
    let mut best_score = i32::MIN;
    for z in best.1 - REFINE_RADIUS..=best.1 + REFINE_RADIUS {
        for x in best.0 - REFINE_RADIUS..=best.0 + REFINE_RADIUS {
            let Some(score) = spawn_score(seed, x, z) else {
                continue;
            };
            if score <= best_score {
                continue;
            }
            best_score = score;
            best = (x, z, terrain_column(seed, x, z).surface_y);
        }
    }
    best
}

fn coarse_best_column(seed: i64) -> (i32, i32, i32) {
    let mut best = (0, 0, terrain_column(seed, 0, 0).surface_y);
    let mut best_score = i32::MIN;
    for z in (-COARSE_RADIUS..=COARSE_RADIUS).step_by(COARSE_STEP) {
        for x in (-COARSE_RADIUS..=COARSE_RADIUS).step_by(COARSE_STEP) {
            let Some(score) = spawn_score(seed, x, z) else {
                continue;
            };
            if score <= best_score {
                continue;
            }
            best_score = score;
            best = (x, z, terrain_column(seed, x, z).surface_y);
        }
    }
    best
}

fn spawn_score(seed: i64, x: i32, z: i32) -> Option<i32> {
    let column = terrain_column(seed, x, z);
    if column.is_water() {
        return None;
    }
    let slope = max_neighbor_delta(seed, x, z);
    if slope > MAX_SPAWN_SLOPE {
        return None;
    }
    let y = column.surface_y;
    if decorator_block_at(seed, x, y + 1, z).is_some()
        || decorator_block_at(seed, x, y + 2, z).is_some()
    {
        return None;
    }
    let distance = x.abs() + z.abs();
    Some(
        180 - slope * 12 - distance / 16 - (y - 76).abs() * 2
            + nearby_water_bonus(seed, x, z)
            + nearby_wood_bonus(seed, x, z)
            + openness_bonus(seed, x, z),
    )
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

fn nearby_water_bonus(seed: i64, x: i32, z: i32) -> i32 {
    for dz in (-24..=24).step_by(4) {
        for dx in (-24..=24).step_by(4) {
            if terrain_column(seed, x + dx, z + dz).is_water() {
                return 28;
            }
        }
    }
    0
}

fn nearby_wood_bonus(seed: i64, x: i32, z: i32) -> i32 {
    if nearby_wood(seed, x, z) { 32 } else { 0 }
}

fn openness_bonus(seed: i64, x: i32, z: i32) -> i32 {
    let mut safe = 0;
    for dz in (-8..=8).step_by(4) {
        for dx in (-8..=8).step_by(4) {
            let column = terrain_column(seed, x + dx, z + dz);
            if !column.is_water() && max_neighbor_delta(seed, x + dx, z + dz) <= MAX_SPAWN_SLOPE {
                safe += 1;
            }
        }
    }
    safe * 2
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
