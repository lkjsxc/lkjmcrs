use crate::scheduler::{BlockMutation, RegionActorError};
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, DroppedItemEntity};
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
    LoadChunks {
        positions: Vec<ChunkPos>,
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
    SpawnItem {
        pos: BlockPos,
        item_id: String,
        count: u8,
        reply: oneshot::Sender<DroppedItemEntity>,
    },
    ItemsInChunks {
        chunks: Vec<ChunkPos>,
        reply: oneshot::Sender<Vec<DroppedItemEntity>>,
    },
    CollectNearby {
        x: f64,
        y: f64,
        z: f64,
        accepted_items: Vec<String>,
        reply: oneshot::Sender<Option<DroppedItemEntity>>,
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
    pub mutation: BlockMutation,
    pub attempts: u8,
}
