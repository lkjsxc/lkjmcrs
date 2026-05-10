use super::biome::{BiomeKind, SurfaceKind};
use super::column::{TerrainColumn, terrain_column};
use super::noise;
use crate::world::BlockState;

const TREE_CELL: i32 = 11;
const TREE_SCAN_RADIUS: i32 = 2;
const MAX_TREE_SLOPE: i32 = 4;

pub(in crate::world) fn block_at(seed: i64, x: i32, y: i32, z: i32) -> Option<BlockState> {
    for rz in z - TREE_SCAN_RADIUS..=z + TREE_SCAN_RADIUS {
        for rx in x - TREE_SCAN_RADIUS..=x + TREE_SCAN_RADIUS {
            let root = terrain_column(seed, rx, rz);
            if !is_tree_root(seed, rx, rz, root) {
                continue;
            }
            if let Some(state) = tree_state(seed, rx, rz, root.surface_y, x, y, z) {
                return Some(state);
            }
        }
    }
    None
}

pub(in crate::world) fn nearby_wood(seed: i64, x: i32, z: i32) -> bool {
    for dz in (-32..=32).step_by(4) {
        for dx in (-32..=32).step_by(4) {
            let column = terrain_column(seed, x + dx, z + dz);
            if is_tree_root(seed, x + dx, z + dz, column) {
                return true;
            }
        }
    }
    false
}

fn is_tree_root(seed: i64, x: i32, z: i32, column: TerrainColumn) -> bool {
    column.water_y.is_none()
        && column.surface == SurfaceKind::Grass
        && column.biome == BiomeKind::Forest
        && column.surface_y < 104
        && root_position(seed, x, z) == (x, z)
        && max_neighbor_delta(seed, x, z, column.surface_y) <= MAX_TREE_SLOPE
}

fn root_position(seed: i64, x: i32, z: i32) -> (i32, i32) {
    let cell_x = x.div_euclid(TREE_CELL);
    let cell_z = z.div_euclid(TREE_CELL);
    let offset_x = cell_offset(seed ^ 0x7191, cell_x, cell_z);
    let offset_z = cell_offset(seed ^ 0x1917, cell_x, cell_z);
    (cell_x * TREE_CELL + offset_x, cell_z * TREE_CELL + offset_z)
}

fn cell_offset(seed: i64, cell_x: i32, cell_z: i32) -> i32 {
    let value = ((noise::unit(seed, cell_x, cell_z) + 1.0) * 0.5 * TREE_CELL as f64) as i32;
    value.clamp(1, TREE_CELL - 2)
}

fn max_neighbor_delta(seed: i64, x: i32, z: i32, surface_y: i32) -> i32 {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dx, dz)| (terrain_column(seed, x + dx, z + dz).surface_y - surface_y).abs())
        .max()
        .unwrap_or(0)
}

fn tree_state(
    seed: i64,
    rx: i32,
    rz: i32,
    root_y: i32,
    x: i32,
    y: i32,
    z: i32,
) -> Option<BlockState> {
    let trunk_top = root_y + 4 + (noise::unit(seed ^ 0x5eed, rx, rz).abs() * 2.0) as i32;
    if x == rx && z == rz && (root_y + 1..=trunk_top).contains(&y) {
        return Some(BlockState::SpruceLog);
    }
    let radius = (x - rx).abs().max((z - rz).abs());
    let leaf_base = trunk_top - 2;
    let leaf_top = trunk_top + 1;
    (radius <= leaf_radius(y, leaf_base, leaf_top) && (leaf_base..=leaf_top).contains(&y))
        .then_some(BlockState::SpruceLeaves)
}

fn leaf_radius(y: i32, leaf_base: i32, leaf_top: i32) -> i32 {
    if y == leaf_top {
        1
    } else if y == leaf_base {
        2
    } else {
        1
    }
}
