use crate::world::blocks::{BlockState, CHUNK_HEIGHT, CHUNK_WIDTH, MIN_Y};
pub(super) use crate::world::terrain::TerrainColumn;
use crate::world::{ChunkPos, terrain};

const PALETTE: [BlockState; 6] = [
    BlockState::Air,
    BlockState::Bedrock,
    BlockState::Stone,
    BlockState::Dirt,
    BlockState::GrassBlock,
    BlockState::Water,
];

pub(super) fn palette() -> [BlockState; 6] {
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
            y if y == column.surface_y && column.water_y.is_none() => 4,
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
