use crate::scheduler::region_command::RegionCommand;
use crate::scheduler::region_handle::RegionHandle;
use crate::scheduler::region_state::RegionActor;
use crate::world::{RegionId, TerrainGenerator, WorldStorage};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub(super) const REGION_MAILBOX_CAPACITY: usize = 64;

impl RegionActor {
    pub fn spawn(id: RegionId) -> RegionHandle {
        Self::spawn_with_storage(id, None, TerrainGenerator::flat())
    }

    pub fn spawn_persistent(id: RegionId, storage: WorldStorage) -> RegionHandle {
        Self::spawn_with_generator(id, storage, TerrainGenerator::flat())
    }

    pub fn spawn_with_generator(
        id: RegionId,
        storage: WorldStorage,
        world: TerrainGenerator,
    ) -> RegionHandle {
        Self::spawn_with_storage(id, Some(storage), world)
    }

    fn spawn_with_storage(
        id: RegionId,
        storage: Option<WorldStorage>,
        world: TerrainGenerator,
    ) -> RegionHandle {
        let (outbox, inbox) = mpsc::channel(REGION_MAILBOX_CAPACITY);
        let actor = Self {
            id,
            applied: 0,
            chunks: HashMap::new(),
            item_entities: HashMap::new(),
            next_item_entity_id: Self::first_item_entity_id(),
            world,
            storage,
            outbox: outbox.clone(),
            inbox,
            pending_loads: HashMap::new(),
            pending_saves: HashMap::new(),
            next_job: 1,
        };
        tokio::spawn(actor.run());
        RegionHandle { id, outbox }
    }

    async fn run(mut self) {
        while let Some(command) = self.inbox.recv().await {
            match command {
                RegionCommand::Apply { label, reply } => {
                    tracing::trace!(region = self.id.0, %label, "region task applied");
                    self.applied += 1;
                    let _ = reply.send(self.applied);
                }
                RegionCommand::SpawnChunks {
                    center,
                    radius,
                    reply,
                } => self.spawn_chunks(center, radius, reply),
                RegionCommand::LoadChunks { positions, reply } => {
                    self.load_chunks(positions, reply)
                }
                RegionCommand::ChunkSnapshot { pos, reply } => {
                    let _ = reply.send(self.chunks.get(&pos).cloned());
                }
                RegionCommand::GetBlock { pos, reply } => {
                    let block = self.chunks.get(&pos.chunk()).map(|c| c.block_at_pos(pos));
                    let _ = reply.send(block);
                }
                RegionCommand::SetBlock { pos, state, reply } => self.set_block(pos, state, reply),
                RegionCommand::SpawnItem {
                    pos,
                    item_id,
                    count,
                    reply,
                } => self.spawn_item(pos, item_id, count, reply),
                RegionCommand::ItemsInChunks { chunks, reply } => {
                    self.items_in_chunks(chunks, reply)
                }
                RegionCommand::CollectNearby {
                    x,
                    y,
                    z,
                    accepted_items,
                    reply,
                } => self.collect_nearby(x, y, z, accepted_items, reply),
                RegionCommand::Snapshot { reply } => {
                    let _ = reply.send(self.applied);
                }
                RegionCommand::LoadComplete { id, result } => self.complete_load(id, result),
                RegionCommand::SaveComplete { id, result } => self.complete_save(id, result),
            }
        }
    }
}
