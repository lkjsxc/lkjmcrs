use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::entity_packets;
use crate::session::error::ConnectionError;
use crate::world::ChunkPos;
use tokio::io::AsyncWrite;

pub async fn send_items_in_chunks<W>(
    writer: &mut W,
    phase: SessionState,
    region: &RegionHandle,
    chunks: Vec<ChunkPos>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let items = region
        .items_in_chunks(chunks)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    for item in items {
        entity_packets::send_item_spawn(writer, phase, &item).await?;
    }
    Ok(())
}
