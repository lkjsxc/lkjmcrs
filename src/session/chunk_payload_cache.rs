use crate::protocol::chunk;
use crate::session::chunk_wire::WireChunk;
use crate::world::ChunkSnapshot;
use std::sync::OnceLock;

static FLAT_BODY: OnceLock<Vec<u8>> = OnceLock::new();

#[derive(Debug, Default)]
pub struct ChunkPayloadCache {
    stats: ChunkPayloadCacheStats,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPayloadCacheStats {
    pub flat_hits: usize,
    pub flat_misses: usize,
    pub override_bypasses: usize,
}

impl ChunkPayloadCache {
    pub fn encode(&mut self, snapshot: &ChunkSnapshot) -> Vec<u8> {
        if snapshot.is_shared_flat_base() {
            return self.encode_flat(snapshot);
        }
        self.stats.override_bypasses += 1;
        chunk::encode_level_chunk_with_light(&WireChunk(snapshot))
    }

    pub fn stats(&self) -> ChunkPayloadCacheStats {
        self.stats
    }

    fn encode_flat(&mut self, snapshot: &ChunkSnapshot) -> Vec<u8> {
        if FLAT_BODY.get().is_some() {
            self.stats.flat_hits += 1;
        } else {
            self.stats.flat_misses += 1;
        }
        let body = FLAT_BODY
            .get_or_init(|| chunk::encode_level_chunk_body_with_light(&WireChunk(snapshot)));
        with_position(snapshot, body)
    }
}

fn with_position(snapshot: &ChunkSnapshot, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + body.len());
    out.extend_from_slice(&snapshot.pos.x.to_be_bytes());
    out.extend_from_slice(&snapshot.pos.z.to_be_bytes());
    out.extend_from_slice(body);
    out
}

#[cfg(test)]
mod tests {
    use super::ChunkPayloadCache;
    use crate::protocol::chunk;
    use crate::session::chunk_wire::WireChunk;
    use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot};

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
    fn natural_chunks_bypass_shared_flat_cache() {
        let mut cache = ChunkPayloadCache::default();
        let chunk = crate::world::TerrainGenerator::natural(9).chunk_snapshot(ChunkPos::new(2, 0));

        cache.encode(&chunk);

        assert_eq!(cache.stats().flat_hits, 0);
        assert_eq!(cache.stats().flat_misses, 0);
        assert_eq!(cache.stats().override_bypasses, 1);
    }
}
