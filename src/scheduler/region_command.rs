use crate::scheduler::{BlockMutation, RegionActorError};
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot};
use tokio::sync::oneshot;

#[derive(Debug)]
pub(super) enum RegionCommand {
    Apply {
        label: String,
        reply: oneshot::Sender<usize>,
    },
    SpawnChunks {
        center: ChunkPos,
        radius: i32,
        reply: oneshot::Sender<Result<Vec<ChunkSnapshot>, RegionActorError>>,
    },
    ChunkSnapshot {
        pos: ChunkPos,
        reply: oneshot::Sender<Option<ChunkSnapshot>>,
    },
    GetBlock {
        pos: BlockPos,
        reply: oneshot::Sender<Option<BlockState>>,
    },
    SetBlock {
        pos: BlockPos,
        state: BlockState,
        reply: oneshot::Sender<Result<BlockMutation, RegionActorError>>,
    },
    Snapshot {
        reply: oneshot::Sender<usize>,
    },
    LoadComplete {
        id: u64,
        result: Result<Vec<ChunkSnapshot>, RegionActorError>,
    },
    SaveComplete {
        id: u64,
        result: Result<(), RegionActorError>,
    },
}

#[derive(Debug)]
pub(super) struct PendingLoad {
    pub positions: Vec<ChunkPos>,
    pub reply: oneshot::Sender<Result<Vec<ChunkSnapshot>, RegionActorError>>,
}

#[derive(Debug)]
pub(super) struct PendingSave {
    pub before: BlockState,
    pub mutation: BlockMutation,
    pub reply: oneshot::Sender<Result<BlockMutation, RegionActorError>>,
}
