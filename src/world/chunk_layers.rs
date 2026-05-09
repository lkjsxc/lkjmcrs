use crate::world::blocks::{BlockState, CHUNK_HEIGHT, CHUNK_WIDTH, MIN_Y};

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

pub(super) fn terrain_layers(heights: &[i32; 256]) -> Vec<u8> {
    let mut layers = vec![0; CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT];
    for z in 0..CHUNK_WIDTH {
        for x in 0..CHUNK_WIDTH {
            write_column(&mut layers, x, z, heights[z * CHUNK_WIDTH + x]);
        }
    }
    layers
}

fn write_column(layers: &mut [u8], x: usize, z: usize, surface_y: i32) {
    for y in 0..=surface_y {
        let state = match y {
            0 => 1,
            y if y == surface_y => 4,
            y if y >= surface_y - 3 => 3,
            _ => 2,
        };
        layers[layer_index(x, y, z)] = state;
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
