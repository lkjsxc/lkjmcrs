use crate::protocol::chunk;
use crate::session::chunk_wire::WireChunk;
use crate::world::ChunkSnapshot;

#[derive(Debug, Default)]
pub struct ChunkPayloadCache {
    flat_body: Option<Vec<u8>>,
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
        if snapshot.override_count() == 0 {
            return self.encode_flat(snapshot);
        }
        self.stats.override_bypasses += 1;
        chunk::encode_level_chunk_with_light(&WireChunk(snapshot))
    }

    pub fn stats(&self) -> ChunkPayloadCacheStats {
        self.stats
    }

    fn encode_flat(&mut self, snapshot: &ChunkSnapshot) -> Vec<u8> {
        if self.flat_body.is_some() {
            self.stats.flat_hits += 1;
        } else {
            self.stats.flat_misses += 1;
            self.flat_body = Some(chunk::encode_level_chunk_body_with_light(&WireChunk(
                snapshot,
            )));
        }
        let body = self.flat_body.as_ref().expect("flat payload body");
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
        assert_eq!(cache.stats().flat_misses, 1);
        assert_eq!(cache.stats().flat_hits, 1);
        assert_eq!(cache.stats().override_bypasses, 1);
    }
}
