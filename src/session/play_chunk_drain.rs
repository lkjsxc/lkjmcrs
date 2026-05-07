use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::chunk_stream::{ChunkStream, StreamContext};
use crate::session::chunk_stream_send::ChunkSendBudget;
use crate::session::error::ConnectionError;
use crate::session::registry::{SessionId, SessionRegistry};
use tokio::io::AsyncWrite;

pub async fn flush_pending<W>(
    writer: &mut W,
    phase: SessionState,
    region: &RegionHandle,
    sessions: &SessionRegistry,
    session_id: SessionId,
    chunk_stream: &mut ChunkStream,
    cache: &mut ChunkPayloadCache,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    chunk_stream
        .flush_pending(
            StreamContext {
                region,
                sessions,
                session_id,
            },
            phase,
            writer,
            cache,
            ChunkSendBudget::progressive(),
        )
        .await
}
