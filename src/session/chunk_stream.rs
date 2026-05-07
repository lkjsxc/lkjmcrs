use crate::protocol::play;
use crate::protocol::{chunk, ids};
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::bootstrap::send_chunk_batch;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::item_visibility;
use crate::session::registry::{SessionId, SessionRegistry};
use crate::world::{ChunkPos, ChunkSnapshot};
use std::collections::HashSet;
use tokio::io::AsyncWrite;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkStream {
    center: ChunkPos,
    radius: i32,
    visible: HashSet<ChunkPos>,
}

impl ChunkStream {
    pub fn new(center: ChunkPos, radius: i32) -> Self {
        Self {
            center,
            radius,
            visible: visible_chunks(center, radius).into_iter().collect(),
        }
    }

    pub async fn stream_after_movement<W>(
        &mut self,
        x: f64,
        z: f64,
        phase: SessionState,
        context: StreamContext<'_>,
        writer: &mut W,
    ) -> Result<(), ConnectionError>
    where
        W: AsyncWrite + Unpin,
    {
        let next_center = chunk_center(x, z);
        let Some(diff) = self.advance(next_center) else {
            return Ok(());
        };
        write_packet(
            writer,
            phase,
            ids::play::CHUNK_CACHE_CENTER,
            &play::encode_chunk_cache_center(next_center.x, next_center.z),
        )
        .await?;
        for pos in &diff.leaving {
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
            .unsubscribe(context.session_id, diff.leaving.iter().copied())
            .await;
        let chunks = load_newly_visible(context.region, phase, &diff.entering).await?;
        if !chunks.is_empty() {
            send_chunk_batch(writer, &chunks).await?;
            item_visibility::send_items_in_chunks(
                writer,
                phase,
                context.region,
                chunks.iter().map(|chunk| chunk.pos).collect(),
            )
            .await?;
            context
                .sessions
                .subscribe(context.session_id, chunks.iter().map(|chunk| chunk.pos))
                .await;
        }
        Ok(())
    }

    pub(super) fn advance(&mut self, next_center: ChunkPos) -> Option<VisibleDiff> {
        if next_center == self.center {
            return None;
        }
        let next = visible_chunks(next_center, self.radius);
        let next_set: HashSet<_> = next.iter().copied().collect();
        let previous = std::mem::replace(&mut self.visible, next_set.clone());
        self.center = next_center;
        let mut entering = next
            .into_iter()
            .filter(|pos| !previous.contains(pos))
            .collect::<Vec<_>>();
        let mut leaving = previous
            .into_iter()
            .filter(|pos| !next_set.contains(pos))
            .collect::<Vec<_>>();
        entering.sort_by_key(|pos| (pos.x, pos.z));
        leaving.sort_by_key(|pos| (pos.x, pos.z));
        Some(VisibleDiff { entering, leaving })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VisibleDiff {
    pub(super) entering: Vec<ChunkPos>,
    pub(super) leaving: Vec<ChunkPos>,
}

#[derive(Debug, Clone, Copy)]
pub struct StreamContext<'a> {
    pub region: &'a RegionHandle,
    pub sessions: &'a SessionRegistry,
    pub session_id: SessionId,
}

pub fn chunk_center(x: f64, z: f64) -> ChunkPos {
    ChunkPos::new(block_coord(x).div_euclid(16), block_coord(z).div_euclid(16))
}

pub fn visible_chunks(center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
    assert!(radius >= 0, "chunk radius must be non-negative");
    let mut chunks = Vec::new();
    for z in center.z - radius..=center.z + radius {
        for x in center.x - radius..=center.x + radius {
            chunks.push(ChunkPos::new(x, z));
        }
    }
    chunks
}

async fn load_newly_visible(
    region: &RegionHandle,
    phase: SessionState,
    newly_visible: &[ChunkPos],
) -> Result<Vec<ChunkSnapshot>, ConnectionError> {
    region
        .load_chunks(newly_visible.to_vec())
        .await
        .map_err(|source| ConnectionError::Region { phase, source })
}

fn block_coord(value: f64) -> i32 {
    value.floor() as i32
}
