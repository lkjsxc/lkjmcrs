use crate::protocol::{chunk, ids, play};
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::chunk_stream_load::encode_loaded_with_budget;
use crate::session::chunk_stream_metrics::{ChunkStreamStats, emit_chunk_stream_stats};
use crate::session::chunk_stream_send::{ChunkSendBudget, EncodedChunk, send_encoded_chunk_batch};
use crate::session::chunk_stream_window::{EAGER_RADIUS, eager_chunks, ordered_pending};
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::item_visibility;
use crate::session::registry::{SessionId, SessionRegistry};
use crate::world::ChunkPos;
use std::collections::{HashSet, VecDeque};
use tokio::io::AsyncWrite;

pub use crate::session::chunk_stream_window::{chunk_center, visible_chunks};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkStream {
    center: ChunkPos,
    radius: i32,
    sent: HashSet<ChunkPos>,
    pending: VecDeque<ChunkPos>,
    stats: ChunkStreamStats,
}

impl ChunkStream {
    pub fn new(center: ChunkPos, radius: i32) -> Self {
        let initial = eager_chunks(center, radius);
        let sent = initial.iter().copied().collect::<HashSet<_>>();
        let pending = ordered_pending(center, radius, &sent);
        Self {
            center,
            radius,
            sent,
            pending,
            stats: ChunkStreamStats::default(),
        }
    }

    pub fn initial_chunks(&self) -> Vec<ChunkPos> {
        let mut chunks = self.sent.iter().copied().collect::<Vec<_>>();
        chunks.sort_by_key(|pos| (pos.x, pos.z));
        chunks
    }

    pub async fn stream_after_movement<W>(
        &mut self,
        x: f64,
        z: f64,
        phase: SessionState,
        context: StreamContext<'_>,
        writer: &mut W,
        cache: &mut ChunkPayloadCache,
    ) -> Result<(), ConnectionError>
    where
        W: AsyncWrite + Unpin,
    {
        let next_center = chunk_center(x, z);
        let Some(leaving) = self.advance(next_center) else {
            return Ok(());
        };
        write_packet(
            writer,
            phase,
            ids::play::CHUNK_CACHE_CENTER,
            &play::encode_chunk_cache_center(next_center.x, next_center.z),
        )
        .await?;
        for pos in &leaving {
            write_packet(
                writer,
                phase,
                ids::play::UNLOAD_CHUNK,
                &chunk::encode_unload_chunk(chunk::ChunkPosition { x: pos.x, z: pos.z }),
            )
            .await?;
        }
        context
            .sessions
            .unsubscribe(context.session_id, leaving.iter().copied())
            .await;
        if self.radius <= EAGER_RADIUS {
            self.flush_pending(context, phase, writer, cache, ChunkSendBudget::unlimited())
                .await?;
        }
        Ok(())
    }

    pub(super) fn advance(&mut self, next_center: ChunkPos) -> Option<Vec<ChunkPos>> {
        if next_center == self.center {
            return None;
        }
        let next_set = HashSet::<_>::from_iter(visible_chunks(next_center, self.radius));
        let mut leaving = self
            .sent
            .iter()
            .copied()
            .filter(|pos| !next_set.contains(pos))
            .collect::<Vec<_>>();
        self.sent.retain(|pos| next_set.contains(pos));
        self.center = next_center;
        leaving.sort_by_key(|pos| (pos.x, pos.z));
        self.pending = ordered_pending(next_center, self.radius, &self.sent);
        Some(leaving)
    }

    pub async fn flush_pending<W>(
        &mut self,
        context: StreamContext<'_>,
        phase: SessionState,
        writer: &mut W,
        cache: &mut ChunkPayloadCache,
        budget: ChunkSendBudget,
    ) -> Result<(), ConnectionError>
    where
        W: AsyncWrite + Unpin,
    {
        let chunks = self
            .next_encoded(context.region, phase, cache, budget)
            .await?;
        if chunks.is_empty() {
            return Ok(());
        }
        let batch_chunks = chunks.len();
        let batch_payload_bytes = chunks.iter().map(EncodedChunk::len).sum::<usize>();
        send_encoded_chunk_batch(writer, &chunks).await?;
        let sent = chunks.iter().map(|chunk| chunk.pos).collect::<Vec<_>>();
        item_visibility::send_items_in_chunks(writer, phase, context.region, sent.clone()).await?;
        context
            .sessions
            .subscribe(context.session_id, sent.iter().copied())
            .await;
        self.sent.extend(sent);
        self.stats
            .record_batch(batch_chunks, batch_payload_bytes, self.pending.len());
        let active_sessions = context.sessions.active_count().await;
        emit_chunk_stream_stats(
            self.stats,
            cache.stats(),
            batch_chunks,
            batch_payload_bytes,
            self.pending.len(),
            active_sessions,
            context.region.mailbox_depth(),
        );
        Ok(())
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    async fn next_encoded(
        &mut self,
        region: &RegionHandle,
        phase: SessionState,
        cache: &mut ChunkPayloadCache,
        budget: ChunkSendBudget,
    ) -> Result<Vec<EncodedChunk>, ConnectionError> {
        self.stats.record_pending(self.pending.len());
        let positions = self.drain_pending_positions(budget.max_chunks);
        if positions.is_empty() {
            return Ok(Vec::new());
        }
        let snapshots = region
            .load_chunks(positions.clone())
            .await
            .map_err(|source| ConnectionError::Region { phase, source })?;
        let (chunks, unsent) = encode_loaded_with_budget(positions, snapshots, cache, budget);
        self.restore_unsent_front(unsent);
        Ok(chunks)
    }

    pub(super) fn drain_pending_positions(&mut self, limit: usize) -> Vec<ChunkPos> {
        let mut positions = Vec::new();
        while positions.len() < limit {
            let Some(pos) = self.pending.pop_front() else {
                break;
            };
            positions.push(pos);
        }
        positions
    }

    fn restore_unsent_front(&mut self, positions: Vec<ChunkPos>) {
        for pos in positions.into_iter().rev() {
            self.pending.push_front(pos);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StreamContext<'a> {
    pub region: &'a RegionHandle,
    pub sessions: &'a SessionRegistry,
    pub session_id: SessionId,
}
