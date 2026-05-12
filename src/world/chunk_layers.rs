use crate::world::blocks::{BlockState, CHUNK_HEIGHT, CHUNK_WIDTH, MIN_Y};
use crate::world::terrain::SurfaceKind;
pub(super) use crate::world::terrain::TerrainColumn;
use crate::world::{ChunkPos, terrain};

const PALETTE: [BlockState; 8] = [
    BlockState::Air,
    BlockState::Bedrock,
    BlockState::Stone,
    BlockState::Dirt,
    BlockState::GrassBlock,
    BlockState::Water,
    BlockState::SpruceLog,
    BlockState::SpruceLeaves,
];

pub(super) fn palette() -> [BlockState; 8] {
    PALETTE
}

pub(super) fn flat_layers() -> Vec<u8> {
    let mut layers = vec![0; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT];
    for z in 0..CHUNK_WIDTH {
        for x in 0..CHUNK_WIDTH {
            write_flat_column(&mut layers, x, z);
        }
    }
    layers
}

pub(super) fn terrain_layers(
    pos: ChunkPos,
    world_seed: i64,
    columns: &[TerrainColumn; 256],
) -> Vec<u8> {
    let mut layers = vec![0; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT];
    for z in 0..CHUNK_WIDTH {
        for x in 0..CHUNK_WIDTH {
            write_column(
                &mut layers,
                pos,
                world_seed,
                x,
                z,
                columns[z * CHUNK_WIDTH + x],
            );
        }
    }
    write_tree_decorators(&mut layers, pos, world_seed);
    layers
}

fn write_column(
    layers: &mut [u8],
    pos: ChunkPos,
    world_seed: i64,
    x: usize,
    z: usize,
    column: TerrainColumn,
) {
    let global_x = pos.x * CHUNK_WIDTH as i32 + x as i32;
    let global_z = pos.z * CHUNK_WIDTH as i32 + z as i32;
    for y in 0..=column.surface_y {
        let mut state = match y {
            0 => 1,
            y if y == column.surface_y && column.water_y.is_none() => surface_state(column.surface),
            y if y >= column.surface_y - 3 => 3,
            _ => 2,
        };
        if (state == 2 || state == 3)
            && terrain::carves_air(world_seed, global_x, y, global_z, column)
        {
            state = 0;
        }
        layers[layer_index(x, y, z)] = state;
    }
    if let Some(water_y) = column.water_y {
        for y in column.surface_y + 1..=water_y {
            layers[layer_index(x, y, z)] = 5;
        }
    }
}

fn surface_state(surface: SurfaceKind) -> u8 {
    match surface {
        SurfaceKind::Grass => 4,
        SurfaceKind::Dirt => 3,
        SurfaceKind::Stone => 2,
    }
}

fn write_tree_decorators(layers: &mut [u8], pos: ChunkPos, world_seed: i64) {
    let min_x = pos.x * CHUNK_WIDTH as i32 - terrain::TREE_REACH;
    let max_x = pos.x * CHUNK_WIDTH as i32 + CHUNK_WIDTH as i32 - 1 + terrain::TREE_REACH;
    let min_z = pos.z * CHUNK_WIDTH as i32 - terrain::TREE_REACH;
    let max_z = pos.z * CHUNK_WIDTH as i32 + CHUNK_WIDTH as i32 - 1 + terrain::TREE_REACH;
    for rz in min_z..=max_z {
        for rx in min_x..=max_x {
            let root = terrain::terrain_column(world_seed, rx, rz);
            if terrain::is_tree_root_at(world_seed, rx, rz, root) {
                write_tree(layers, pos, world_seed, rx, rz, root.surface_y);
            }
        }
    }
}

fn write_tree(layers: &mut [u8], pos: ChunkPos, seed: i64, rx: i32, rz: i32, root_y: i32) {
    for y in root_y + 1..=root_y + terrain::TREE_MAX_HEIGHT {
        for z in rz - terrain::TREE_REACH..=rz + terrain::TREE_REACH {
            for x in rx - terrain::TREE_REACH..=rx + terrain::TREE_REACH {
                if let Some(state) = terrain::tree_state_at(seed, rx, rz, root_y, x, y, z) {
                    write_tree_block(layers, pos, x, y, z, state);
                }
            }
        }
    }
}

fn write_tree_block(
    layers: &mut [u8],
    pos: ChunkPos,
    global_x: i32,
    y: i32,
    global_z: i32,
    state: BlockState,
) {
    if global_x.div_euclid(16) != pos.x || global_z.div_euclid(16) != pos.z {
        return;
    }
    let x = global_x.rem_euclid(16) as usize;
    let z = global_z.rem_euclid(16) as usize;
    let index = layer_index(x, y, z);
    let state = state_index(state);
    if layers[index] == 0 || (layers[index] == 7 && state == 6) {
        layers[index] = state;
    }
}

fn state_index(state: BlockState) -> u8 {
    match state {
        BlockState::Air => 0,
        BlockState::Bedrock => 1,
        BlockState::Stone => 2,
        BlockState::Dirt => 3,
        BlockState::GrassBlock => 4,
        BlockState::Water => 5,
        BlockState::SpruceLog => 6,
        BlockState::SpruceLeaves => 7,
    }
}

fn write_flat_column(layers: &mut [u8], x: usize, z: usize) {
    for y in 0..=79 {
        let state = match y {
            0 => 1,
            1..=62 => 2,
            63..=78 => 3,
            79 => 4,
            _ => 0,
        };
        layers[layer_index(x, y, z)] = state;
    }
}

pub(super) fn layer_index(x: usize, y: i32, z: usize) -> usize {
    let y_index = (y - MIN_Y) as usize;
    (y_index * CHUNK_WIDTH * CHUNK_WIDTH) + (z * CHUNK_WIDTH) + x
}
