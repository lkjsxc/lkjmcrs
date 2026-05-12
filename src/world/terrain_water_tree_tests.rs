use super::{BlockState, ChunkPos, TerrainGenerator, terrain};

#[test]
fn flat_generator_remains_selectable() {
    let chunk = TerrainGenerator::flat().chunk_snapshot(ChunkPos::new(0, 0));

    assert!(chunk.is_shared_flat_base());
    assert_eq!(chunk.block_at_local(0, 79, 0), BlockState::GrassBlock);
    assert_eq!(chunk.block_at_local(0, 80, 0), BlockState::Air);
}

#[test]
fn fixed_seed_water_uses_documented_level() {
    let world = TerrainGenerator::natural(8675309);
    let mut water_at_level = 0;

    for chunk_z in -2..=2 {
        for chunk_x in -2..=2 {
            let chunk = world.chunk_snapshot(ChunkPos::new(chunk_x, chunk_z));
            water_at_level += count_state_at_y(&chunk, BlockState::Water, terrain::RIVER_LEVEL);
        }
    }

    assert!(water_at_level > 0);
}

#[test]
fn fixed_seed_water_has_nearby_low_bank() {
    let sample = find_water_with_dry_bank().expect("water should meet a dry low bank");
    let (water, bank) = sample;

    assert_eq!(water.water_y, Some(terrain::RIVER_LEVEL));
    assert!(bank.surface_y <= terrain::RIVER_LEVEL + 8);
    assert!(bank.surface_y > terrain::RIVER_LEVEL);
}

#[test]
fn fixed_seed_generates_dense_spruce_trunks() {
    let world = TerrainGenerator::natural(8675309);
    let center = spawn_chunk(&world);
    let trunks = count_trunk_columns(
        &world,
        center.x - 2..=center.x + 2,
        center.z - 2..=center.z + 2,
    );

    assert!(
        trunks >= 16,
        "spruce trunk columns near {:?}: {trunks}",
        center
    );
}

fn spawn_chunk(world: &TerrainGenerator) -> ChunkPos {
    let spawn = world.spawn();
    ChunkPos::new(
        (spawn.0.floor() as i32).div_euclid(16),
        (spawn.2.floor() as i32).div_euclid(16),
    )
}

fn count_state_at_y(chunk: &super::ChunkSnapshot, state: BlockState, y: i32) -> usize {
    (0..16)
        .flat_map(|z| (0..16).map(move |x| (x, z)))
        .filter(|(x, z)| chunk.block_at_local(*x, y, *z) == state)
        .count()
}

fn find_water_with_dry_bank() -> Option<(terrain::TerrainColumn, terrain::TerrainColumn)> {
    for z in -64..64 {
        for x in -64..64 {
            let water = terrain::terrain_column(8675309, x, z);
            if water.water_y != Some(terrain::RIVER_LEVEL) {
                continue;
            }
            if let Some(bank) = dry_neighbor_bank(x, z) {
                return Some((water, bank));
            }
        }
    }
    None
}

fn dry_neighbor_bank(x: i32, z: i32) -> Option<terrain::TerrainColumn> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dx, dz)| terrain::terrain_column(8675309, x + dx, z + dz))
        .find(|column| column.water_y.is_none() && column.surface_y > terrain::RIVER_LEVEL)
}

fn count_trunk_columns(
    world: &TerrainGenerator,
    chunk_xs: std::ops::RangeInclusive<i32>,
    chunk_zs: std::ops::RangeInclusive<i32>,
) -> usize {
    chunk_zs
        .flat_map(|chunk_z| {
            chunk_xs
                .clone()
                .map(move |chunk_x| ChunkPos::new(chunk_x, chunk_z))
        })
        .map(|pos| world.chunk_snapshot(pos))
        .map(|chunk| trunk_columns_in_chunk(&chunk))
        .sum()
}

fn trunk_columns_in_chunk(chunk: &super::ChunkSnapshot) -> usize {
    (0..16)
        .flat_map(|z| (0..16).map(move |x| (x, z)))
        .filter(|(x, z)| (0..160).any(|y| chunk.block_at_local(*x, y, *z) == BlockState::SpruceLog))
        .count()
}
