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
            (81, BlockState::GrassBlock),
            (80, BlockState::GrassBlock),
            (84, BlockState::GrassBlock),
        ]
    );
}

#[test]
fn fixed_seed_generates_static_water_near_spawn() {
    let world = TerrainGenerator::natural(8675309);
    let mut water_columns = 0;

    for chunk_z in -2..=2 {
        for chunk_x in -2..=2 {
            let chunk = world.chunk_snapshot(ChunkPos::new(chunk_x, chunk_z));
            water_columns += water_column_count(&chunk);
        }
    }

    assert!(water_columns > 0);
}

#[test]
fn natural_spawn_resolves_to_dry_column() {
    let world = TerrainGenerator::natural(8675309);
    let spawn = world.spawn();
    let chunk = world.chunk_snapshot(ChunkPos::new(
        (spawn.0.floor() as i32).div_euclid(16),
        (spawn.2.floor() as i32).div_euclid(16),
    ));
    let x = (spawn.0.floor() as i32).rem_euclid(16) as usize;
    let z = (spawn.2.floor() as i32).rem_euclid(16) as usize;
    let ground_y = spawn.1.floor() as i32 - 1;

    assert_ne!(chunk.block_at_local(x, ground_y, z), BlockState::Water);
    assert_eq!(chunk.block_at_local(x, ground_y + 1, z), BlockState::Air);
    assert_eq!(chunk.block_at_local(x, ground_y + 2, z), BlockState::Air);
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

fn water_column_count(chunk: &super::ChunkSnapshot) -> usize {
    (0..16)
        .flat_map(|z| (0..16).map(move |x| (x, z)))
        .filter(|(x, z)| {
            let height = chunk.heightmap_at_local(*x, *z);
            chunk.block_at_local(*x, i32::from(height) - 1, *z) == BlockState::Water
        })
        .count()
}
