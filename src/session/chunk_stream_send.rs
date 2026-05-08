use crate::protocol::{chunk, ids};
use crate::session::SessionState;
use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::stream_budget::{MAX_FOLLOWUP_CHUNKS, MAX_FOLLOWUP_PAYLOAD_BYTES};
use crate::world::{ChunkPos, ChunkSnapshot};
use tokio::io::AsyncWrite;

#[derive(Debug, Clone, Copy)]
pub struct ChunkSendBudget {
    pub max_chunks: usize,
    pub max_payload_bytes: usize,
}

impl ChunkSendBudget {
    pub const fn progressive() -> Self {
        Self {
            max_chunks: MAX_FOLLOWUP_CHUNKS,
            max_payload_bytes: MAX_FOLLOWUP_PAYLOAD_BYTES,
        }
    }

    pub const fn unlimited() -> Self {
        Self {
            max_chunks: usize::MAX,
            max_payload_bytes: usize::MAX,
        }
    }
}

#[derive(Debug)]
pub struct EncodedChunk {
    pub pos: ChunkPos,
    payload: Vec<u8>,
}

impl EncodedChunk {
    pub fn from_snapshot(snapshot: &ChunkSnapshot, cache: &mut ChunkPayloadCache) -> Self {
        Self {
            pos: snapshot.pos,
            payload: cache.encode(snapshot),
        }
    }

    pub fn len(&self) -> usize {
        self.payload.len()
    }

    #[cfg(test)]
    pub fn from_payload_for_tests(pos: ChunkPos, payload: Vec<u8>) -> Self {
        Self { pos, payload }
    }
}

pub async fn send_encoded_chunk_batch<W>(
    writer: &mut W,
    chunks: &[EncodedChunk],
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let phase = SessionState::Play;
    write_packet(writer, phase, ids::play::CHUNK_BATCH_START, &[]).await?;
    for chunk in chunks {
        write_packet(
            writer,
            phase,
            ids::play::LEVEL_CHUNK_WITH_LIGHT,
            &chunk.payload,
        )
        .await?;
    }
    write_packet(
        writer,
        phase,
        ids::play::CHUNK_BATCH_FINISHED,
        &chunk::encode_chunk_batch_finished(chunks.len()),
    )
    .await
}
