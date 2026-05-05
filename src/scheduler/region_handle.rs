use crate::scheduler::region_actor::{RegionCommand, RegionHandle};
use crate::scheduler::{BlockMutation, RegionActorError};
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, RegionId};
use tokio::sync::oneshot;

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
}
