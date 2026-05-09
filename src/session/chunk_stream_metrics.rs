use crate::session::chunk_payload_cache::ChunkPayloadCacheStats;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ChunkStreamStats {
    pub followup_batches: usize,
    pub followup_chunks: usize,
    pub followup_payload_bytes: usize,
    pub max_pending_queue_len: usize,
}

impl ChunkStreamStats {
    pub fn record_pending(&mut self, pending: usize) {
        self.max_pending_queue_len = self.max_pending_queue_len.max(pending);
    }

    pub fn record_batch(&mut self, chunks: usize, payload_bytes: usize, pending: usize) {
        self.followup_batches += 1;
        self.followup_chunks += chunks;
        self.followup_payload_bytes += payload_bytes;
        self.record_pending(pending);
    }
}

pub fn emit_chunk_stream_stats(
    stream: ChunkStreamStats,
    cache: ChunkPayloadCacheStats,
    batch_chunks: usize,
    batch_payload_bytes: usize,
    pending_queue_len: usize,
    active_sessions: usize,
    region_mailbox_depth: usize,
) {
    if pending_queue_len == 0 {
        tracing::info!(
            target: "lkjmcrs::scale",
            followup_batches = stream.followup_batches,
            followup_chunks = stream.followup_chunks,
            followup_payload_bytes = stream.followup_payload_bytes,
            batch_chunks,
            batch_payload_bytes,
            pending_queue_len,
            max_pending_queue_len = stream.max_pending_queue_len,
            active_sessions,
            region_mailbox_depth,
            flat_cache_hits = cache.flat_hits,
            flat_cache_misses = cache.flat_misses,
            generated_cache_hits = cache.generated_hits,
            generated_cache_misses = cache.generated_misses,
            generated_cache_evictions = cache.generated_evictions,
            override_cache_bypasses = cache.override_bypasses,
            "chunk stream counters"
        );
        return;
    }
    tracing::debug!(
        target: "lkjmcrs::scale",
        followup_batches = stream.followup_batches,
        followup_chunks = stream.followup_chunks,
        followup_payload_bytes = stream.followup_payload_bytes,
        batch_chunks,
        batch_payload_bytes,
        pending_queue_len,
        max_pending_queue_len = stream.max_pending_queue_len,
        active_sessions,
        region_mailbox_depth,
        flat_cache_hits = cache.flat_hits,
        flat_cache_misses = cache.flat_misses,
        generated_cache_hits = cache.generated_hits,
        generated_cache_misses = cache.generated_misses,
        generated_cache_evictions = cache.generated_evictions,
        override_cache_bypasses = cache.override_bypasses,
        "chunk stream counters"
    );
}
