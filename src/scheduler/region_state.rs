use crate::scheduler::region_command::{PendingLoad, PendingSave, RegionCommand};
use crate::world::{ChunkPos, ChunkSnapshot, DroppedItemEntity, FlatWorld, RegionId, WorldStorage};
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct RegionActor {
    pub(super) id: RegionId,
    pub(super) applied: usize,
    pub(super) chunks: HashMap<ChunkPos, ChunkSnapshot>,
    pub(super) item_entities: HashMap<i32, DroppedItemEntity>,
    pub(super) next_item_entity_id: i32,
    pub(super) world: FlatWorld,
    pub(super) storage: Option<WorldStorage>,
    pub(super) outbox: mpsc::Sender<RegionCommand>,
    pub(super) inbox: mpsc::Receiver<RegionCommand>,
    pub(super) pending_loads: HashMap<u64, PendingLoad>,
    pub(super) pending_saves: HashMap<u64, PendingSave>,
    pub(super) next_job: u64,
}
