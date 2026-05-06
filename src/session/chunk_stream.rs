use crate::protocol::play;
use crate::protocol::{chunk, ids};
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::bootstrap::send_chunk_batch;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
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
                &chunk::encode_unload_chunk(*pos),
            )
            .await?;
        }
        context
            .sessions
            .unsubscribe(context.session_id, diff.leaving.iter().copied())
            .await;
        let chunks = load_newly_visible(
            context.region,
            phase,
            next_center,
            self.radius,
            &diff.entering,
        )
        .await?;
        if !chunks.is_empty() {
            send_chunk_batch(writer, &chunks).await?;
            context
                .sessions
                .subscribe(context.session_id, chunks.iter().map(|chunk| chunk.pos))
                .await;
        }
        Ok(())
    }

    fn advance(&mut self, next_center: ChunkPos) -> Option<VisibleDiff> {
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
struct VisibleDiff {
    entering: Vec<ChunkPos>,
    leaving: Vec<ChunkPos>,
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
    center: ChunkPos,
    radius: i32,
    newly_visible: &[ChunkPos],
) -> Result<Vec<ChunkSnapshot>, ConnectionError> {
    let newly_visible: HashSet<_> = newly_visible.iter().copied().collect();
    region
        .spawn_chunks_around(center, radius)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })
        .map(|chunks| {
            chunks
                .into_iter()
                .filter(|chunk| newly_visible.contains(&chunk.pos))
                .collect()
        })
}

fn block_coord(value: f64) -> i32 {
    value.floor() as i32
}

#[cfg(test)]
mod tests {
    use super::{ChunkStream, chunk_center, visible_chunks};
    use crate::world::ChunkPos;
    use std::collections::HashSet;

    #[test]
    fn chunk_center_uses_floored_euclidean_coordinates() {
        assert_eq!(chunk_center(0.0, 15.999), ChunkPos::new(0, 0));
        assert_eq!(chunk_center(16.0, 31.0), ChunkPos::new(1, 1));
        assert_eq!(chunk_center(-0.1, -1.0), ChunkPos::new(-1, -1));
        assert_eq!(chunk_center(-16.0, -16.1), ChunkPos::new(-1, -2));
    }

    #[test]
    fn visible_diff_from_origin_to_east_is_new_column() {
        let mut stream = ChunkStream::new(ChunkPos::new(0, 0), 2);
        let diff = stream.advance(ChunkPos::new(1, 0)).unwrap();
        let entering: HashSet<_> = diff.entering.into_iter().collect();
        let leaving: HashSet<_> = diff.leaving.into_iter().collect();
        assert_eq!(entering, (-2..=2).map(|z| ChunkPos::new(3, z)).collect());
        assert_eq!(leaving, (-2..=2).map(|z| ChunkPos::new(-2, z)).collect());
    }

    #[test]
    fn same_center_produces_no_delta() {
        let mut stream = ChunkStream::new(ChunkPos::new(0, 0), 2);
        assert_eq!(stream.advance(ChunkPos::new(0, 0)), None);
    }

    #[test]
    fn visible_chunks_are_square() {
        assert_eq!(visible_chunks(ChunkPos::new(0, 0), 2).len(), 25);
    }
}
