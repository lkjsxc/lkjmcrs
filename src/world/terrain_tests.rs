use super::{BlockState, ChunkPos, TerrainGenerator, terrain};

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
            (64, BlockState::Water),
            (64, BlockState::Water),
            (64, BlockState::Water),
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
fn fixed_seed_generates_underground_cave_air() {
    let world = TerrainGenerator::natural(8675309);
    let first = first_cave_air(&world).expect("fixed seed should generate cave air");
    let (pos, x, y, z) = first;
    let chunk = world.chunk_snapshot(pos);
    let repeated = world.chunk_snapshot(pos);

    assert_eq!(chunk.block_at_local(x, y, z), BlockState::Air);
    assert_eq!(repeated.block_at_local(x, y, z), BlockState::Air);
    assert!(y >= 8);
    assert!(i32::from(chunk.heightmap_at_local(x, z)) - y > 8);
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
fn cave_carving_keeps_bedrock_and_surface_intact() {
    let chunk = TerrainGenerator::natural(8675309).chunk_snapshot(ChunkPos::new(0, 0));

    for z in 0..16 {
        for x in 0..16 {
            let surface_y = i32::from(chunk.heightmap_at_local(x, z)) - 1;
            assert_eq!(chunk.block_at_local(x, 0, z), BlockState::Bedrock);
            assert_ne!(chunk.block_at_local(x, surface_y, z), BlockState::Air);
            for y in surface_y.saturating_sub(7)..=surface_y {
                assert_ne!(chunk.block_at_local(x, y, z), BlockState::Air);
            }
        }
    }
}

#[test]
fn river_water_columns_are_not_carved() {
    let world = TerrainGenerator::natural(8675309);
    let mut checked = 0;

    for chunk_z in -2..=2 {
        for chunk_x in -2..=2 {
            let chunk = world.chunk_snapshot(ChunkPos::new(chunk_x, chunk_z));
            for z in 0..16 {
                for x in 0..16 {
                    let height = chunk.heightmap_at_local(x, z);
                    if chunk.block_at_local(x, i32::from(height) - 1, z) != BlockState::Water {
                        continue;
                    }
                    checked += 1;
                    for y in 8..i32::from(height) {
                        assert_ne!(chunk.block_at_local(x, y, z), BlockState::Air);
                    }
                }
            }
        }
    }

    assert!(checked > 0);
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
fn cave_decisions_are_stable_at_chunk_borders() {
    let seed = 8675309;
    let world = TerrainGenerator::natural(seed);
    let left = world.chunk_snapshot(ChunkPos::new(0, 0));
    let right = world.chunk_snapshot(ChunkPos::new(1, 0));

    for z in 0..16 {
        assert_column_matches_cave_field(seed, &left, 15, 15, z);
        assert_column_matches_cave_field(seed, &right, 16, 0, z);
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

fn first_cave_air(world: &TerrainGenerator) -> Option<(ChunkPos, usize, i32, usize)> {
    for chunk_z in -2..=2 {
        for chunk_x in -2..=2 {
            let pos = ChunkPos::new(chunk_x, chunk_z);
            let chunk = world.chunk_snapshot(pos);
            for z in 0..16 {
                for x in 0..16 {
                    let surface_y = i32::from(chunk.heightmap_at_local(x, z)) - 1;
                    for y in 8..=surface_y - 8 {
                        if chunk.block_at_local(x, y, z) == BlockState::Air {
                            return Some((pos, x, y, z));
                        }
                    }
                }
            }
        }
    }
    None
}

fn assert_column_matches_cave_field(
    seed: i64,
    chunk: &super::ChunkSnapshot,
    global_x: i32,
    local_x: usize,
    z: usize,
) {
    let global_z = chunk.pos.z * 16 + z as i32;
    let column = terrain::terrain_column(seed, global_x, global_z);
    if column.water_y.is_some() {
        return;
    }
    for y in 8..=column.surface_y - 8 {
        let expected_air = terrain::carves_air(seed, global_x, y, global_z, column);
        assert_eq!(
            chunk.block_at_local(local_x, y, z) == BlockState::Air,
            expected_air
        );
    }
}
