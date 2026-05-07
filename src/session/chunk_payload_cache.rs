use crate::protocol::chunk;
use crate::session::chunk_wire::WireChunk;
use crate::world::ChunkSnapshot;

#[derive(Debug, Default)]
pub struct ChunkPayloadCache {
    flat_body: Option<Vec<u8>>,
}

impl ChunkPayloadCache {
    pub fn encode(&mut self, snapshot: &ChunkSnapshot) -> Vec<u8> {
        if snapshot.override_count() == 0 {
            return self.encode_flat(snapshot);
        }
        chunk::encode_level_chunk_with_light(&WireChunk(snapshot))
    }

    fn encode_flat(&mut self, snapshot: &ChunkSnapshot) -> Vec<u8> {
        let body = self
            .flat_body
            .get_or_insert_with(|| chunk::encode_level_chunk_body_with_light(&WireChunk(snapshot)));
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
    use crate::world::{ChunkPos, ChunkSnapshot};

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
}
