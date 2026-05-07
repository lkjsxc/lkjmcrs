use crate::scheduler::region_command::RegionCommand;
use crate::scheduler::{BlockMutation, RegionActorError};
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, DroppedItemEntity, RegionId};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug, Clone)]
pub struct RegionHandle {
    pub(super) id: RegionId,
    pub(super) outbox: mpsc::Sender<RegionCommand>,
}

impl RegionHandle {
    pub const fn id(&self) -> RegionId {
        self.id
    }

    pub async fn apply(&self, label: impl Into<String>) -> Result<usize, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::Apply {
                label: label.into(),
                reply,
            })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }

    pub async fn applied_count(&self) -> Result<usize, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::Snapshot { reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }

    pub async fn spawn_chunks(&self, radius: i32) -> Result<Vec<ChunkSnapshot>, RegionActorError> {
        self.spawn_chunks_around(ChunkPos::new(0, 0), radius).await
    }

    pub async fn spawn_chunks_around(
        &self,
        center: ChunkPos,
        radius: i32,
    ) -> Result<Vec<ChunkSnapshot>, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::SpawnChunks {
                center,
                radius,
                reply,
            })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)?
    }

    pub async fn load_chunks(
        &self,
        positions: Vec<ChunkPos>,
    ) -> Result<Vec<ChunkSnapshot>, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::LoadChunks { positions, reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)?
    }

    pub async fn chunk_snapshot(
        &self,
        pos: ChunkPos,
    ) -> Result<Option<ChunkSnapshot>, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::ChunkSnapshot { pos, reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }

    pub async fn get_block(&self, pos: BlockPos) -> Result<Option<BlockState>, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::GetBlock { pos, reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }

    pub async fn set_block(
        &self,
        pos: BlockPos,
        state: BlockState,
    ) -> Result<BlockMutation, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::SetBlock { pos, state, reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)?
    }

    pub async fn spawn_item(
        &self,
        pos: BlockPos,
        item_id: impl Into<String>,
        count: u8,
    ) -> Result<DroppedItemEntity, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::SpawnItem {
                pos,
                item_id: item_id.into(),
                count,
                reply,
            })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }

    pub async fn items_in_chunks(
        &self,
        chunks: Vec<ChunkPos>,
    ) -> Result<Vec<DroppedItemEntity>, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::ItemsInChunks { chunks, reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }

    pub async fn collect_nearby(
        &self,
        x: f64,
        y: f64,
        z: f64,
        accepted_items: Vec<String>,
    ) -> Result<Option<DroppedItemEntity>, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::CollectNearby {
                x,
                y,
                z,
                accepted_items,
                reply,
            })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }
}
