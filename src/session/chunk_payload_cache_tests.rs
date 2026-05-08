use crate::protocol::chunk;
use crate::session::chunk_payload_cache::{ChunkPayloadCache, GeneratedPayloadCache};
use crate::session::chunk_wire::WireChunk;
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, TerrainGenerator};

#[test]
fn cached_flat_payload_matches_direct_encoding() {
    let mut cache = ChunkPayloadCache::default();
    for pos in [ChunkPos::new(0, 0), ChunkPos::new(3, -2)] {
        let chunk = ChunkSnapshot::flat(pos);
        assert_eq!(
            cache.encode(&chunk),
            chunk::encode_level_chunk_with_light(&WireChunk(&chunk))
        );
    }
}

#[test]
fn cache_stats_distinguish_flat_hits_from_override_bypasses() {
    let mut cache = ChunkPayloadCache::default();
    cache.encode(&ChunkSnapshot::flat(ChunkPos::new(0, 0)));
    cache.encode(&ChunkSnapshot::flat(ChunkPos::new(1, 0)));
    let mut overridden = ChunkSnapshot::flat(ChunkPos::new(2, 0));
    overridden.set_block(BlockPos::new(32, 80, 0), BlockState::Stone);
    cache.encode(&overridden);
    assert_eq!(cache.stats().flat_misses + cache.stats().flat_hits, 2);
    assert_eq!(cache.stats().override_bypasses, 1);
}

#[test]
fn flat_payload_body_is_shared_across_cache_instances() {
    let mut first = ChunkPayloadCache::default();
    let mut second = ChunkPayloadCache::default();

    first.encode(&ChunkSnapshot::flat(ChunkPos::new(10, 0)));
    second.encode(&ChunkSnapshot::flat(ChunkPos::new(11, 0)));

    assert_eq!(second.stats().flat_hits, 1);
    assert_eq!(second.stats().flat_misses, 0);
}

#[test]
fn override_chunks_bypass_shared_flat_cache() {
    let mut cache = ChunkPayloadCache::default();
    let mut overridden = ChunkSnapshot::flat(ChunkPos::new(12, 0));
    overridden.set_block(BlockPos::new(192, 80, 0), BlockState::Stone);

    cache.encode(&overridden);

    assert_eq!(cache.stats().flat_hits, 0);
    assert_eq!(cache.stats().flat_misses, 0);
    assert_eq!(cache.stats().override_bypasses, 1);
}

#[test]
fn natural_chunks_use_generated_cache_instead_of_flat_cache() {
    let mut cache = ChunkPayloadCache::default();
    let chunk = TerrainGenerator::natural(9).chunk_snapshot(ChunkPos::new(2, 0));

    cache.encode(&chunk);

    assert_eq!(cache.stats().flat_hits, 0);
    assert_eq!(cache.stats().flat_misses, 0);
    assert_eq!(cache.stats().generated_misses, 1);
    assert_eq!(cache.stats().override_bypasses, 0);
}

#[test]
fn natural_generated_payload_cache_hits_across_cache_instances() {
    let chunk = TerrainGenerator::natural(9001).chunk_snapshot(ChunkPos::new(8, -3));
    let mut first = ChunkPayloadCache::default();
    let mut second = ChunkPayloadCache::default();

    first.encode(&chunk);
    second.encode(&chunk);

    assert_eq!(second.stats().generated_hits, 1);
    assert_eq!(second.stats().generated_misses, 0);
}

#[test]
fn natural_generated_payload_cache_differs_by_seed_and_position() {
    let mut cache = ChunkPayloadCache::default();
    let first = TerrainGenerator::natural(9002).chunk_snapshot(ChunkPos::new(9, -3));
    let different_seed = TerrainGenerator::natural(9003).chunk_snapshot(ChunkPos::new(9, -3));
    let different_pos = TerrainGenerator::natural(9002).chunk_snapshot(ChunkPos::new(10, -3));

    let first_payload = cache.encode(&first);
    assert_ne!(first_payload, cache.encode(&different_seed));
    assert_ne!(first_payload, cache.encode(&different_pos));
}

#[test]
fn natural_chunks_with_overrides_bypass_generated_cache() {
    let mut cache = ChunkPayloadCache::default();
    let mut chunk = TerrainGenerator::natural(9004).chunk_snapshot(ChunkPos::new(11, -3));
    chunk.set_block(BlockPos::new(176, 80, -48), BlockState::Stone);

    cache.encode(&chunk);

    assert_eq!(cache.stats().generated_hits, 0);
    assert_eq!(cache.stats().generated_misses, 0);
    assert_eq!(cache.stats().override_bypasses, 1);
}

#[test]
fn generated_payload_cache_evicts_fifo() {
    let mut cache = GeneratedPayloadCache::new(2);
    let first = generated_key(12);
    let second = generated_key(13);
    let third = generated_key(14);

    assert!(!cache.insert(first, vec![1]));
    assert!(!cache.insert(second, vec![2]));
    assert!(cache.insert(third, vec![3]));

    assert!(!cache.contains(first));
    assert!(cache.contains(second));
    assert!(cache.contains(third));
}

fn generated_key(x: i32) -> crate::world::GeneratedChunkKey {
    TerrainGenerator::natural(9005)
        .chunk_snapshot(ChunkPos::new(x, -3))
        .generated_cache_key()
        .unwrap()
}
