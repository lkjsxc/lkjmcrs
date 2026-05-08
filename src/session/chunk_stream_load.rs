use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::chunk_stream_send::{ChunkSendBudget, EncodedChunk};
use crate::world::{ChunkPos, ChunkSnapshot};

pub(super) fn encode_loaded_with_budget(
    positions: Vec<ChunkPos>,
    snapshots: Vec<ChunkSnapshot>,
    cache: &mut ChunkPayloadCache,
    budget: ChunkSendBudget,
) -> (Vec<EncodedChunk>, Vec<ChunkPos>) {
    let mut chunks = Vec::new();
    let mut bytes = 0;
    let mut unsent = Vec::new();
    for (index, snapshot) in snapshots.into_iter().enumerate() {
        let encoded = EncodedChunk::from_snapshot(&snapshot, cache);
        let would_exceed = bytes + encoded.len() > budget.max_payload_bytes;
        if would_exceed && !chunks.is_empty() {
            unsent.extend(positions[index..].iter().copied());
            break;
        }
        bytes += encoded.len();
        chunks.push(encoded);
    }
    (chunks, unsent)
}
