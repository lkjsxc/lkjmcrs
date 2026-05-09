use super::{BlockState, ChunkPos, TerrainGenerator};

#[test]
fn worldgen_golden_samples_stay_stable() {
    let chunk = TerrainGenerator::natural(9001).chunk_snapshot(ChunkPos::new(0, 0));
    let samples = [
        column(&chunk, 0, 0),
        column(&chunk, 7, 7),
        column(&chunk, 15, 15),
    ];

    assert_eq!(
        samples,
        [
            (82, BlockState::GrassBlock),
            (80, BlockState::GrassBlock),
            (84, BlockState::GrassBlock),
        ]
    );
}

#[test]
fn adjacent_chunks_share_border_heights() {
    let world = TerrainGenerator::natural(42);
    let left = world.chunk_snapshot(ChunkPos::new(0, 0));
    let right = world.chunk_snapshot(ChunkPos::new(1, 0));

    for z in 0..16 {
        let west = left.heightmap_at_local(15, z);
        let east = right.heightmap_at_local(0, z);
        assert!(west.abs_diff(east) <= 8);
    }
}

#[test]
fn flat_generator_remains_selectable() {
    let chunk = TerrainGenerator::flat().chunk_snapshot(ChunkPos::new(0, 0));
    assert!(chunk.is_shared_flat_base());
    assert_eq!(column(&chunk, 0, 0), (80, BlockState::GrassBlock));
}

fn column(chunk: &super::ChunkSnapshot, x: usize, z: usize) -> (u16, BlockState) {
    let height = chunk.heightmap_at_local(x, z);
    (height, chunk.block_at_local(x, i32::from(height) - 1, z))
}
