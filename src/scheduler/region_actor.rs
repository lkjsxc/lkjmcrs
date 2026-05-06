use crate::scheduler::storage_jobs;
use crate::scheduler::region_command::{PendingLoad, PendingSave, RegionCommand};
use crate::scheduler::region_handle::RegionHandle;
use crate::scheduler::{BlockMutation, RegionActorError};
use crate::world::{
    BlockPos, BlockState, ChunkPos, ChunkSnapshot, FlatWorld, RegionId, WorldStorage,
};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct RegionActor {
    id: RegionId,
    applied: usize,
    chunks: HashMap<ChunkPos, ChunkSnapshot>,
    world: FlatWorld,
    storage: Option<WorldStorage>,
    outbox: mpsc::Sender<RegionCommand>,
    inbox: mpsc::Receiver<RegionCommand>,
    pending_loads: HashMap<u64, PendingLoad>,
    pending_saves: HashMap<u64, PendingSave>,
    next_job: u64,
}

impl RegionActor {
    pub fn spawn(id: RegionId) -> RegionHandle {
        Self::spawn_with_storage(id, None)
    }

    pub fn spawn_persistent(id: RegionId, storage: WorldStorage) -> RegionHandle {
        Self::spawn_with_storage(id, Some(storage))
    }

    fn spawn_with_storage(id: RegionId, storage: Option<WorldStorage>) -> RegionHandle {
        let (outbox, inbox) = mpsc::channel(64);
        let actor = Self {
            id,
            applied: 0,
            chunks: HashMap::new(),
            world: FlatWorld::default(),
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
                RegionCommand::SpawnChunks { center, radius, reply } => {
                    self.spawn_chunks(center, radius, reply)
                }
                RegionCommand::ChunkSnapshot { pos, reply } => {
                    let _ = reply.send(self.chunks.get(&pos).cloned());
                }
                RegionCommand::GetBlock { pos, reply } => {
                    let block = self.chunks.get(&pos.chunk()).map(|c| c.block_at_pos(pos));
                    let _ = reply.send(block);
                }
                RegionCommand::SetBlock { pos, state, reply } => {
                    self.set_block(pos, state, reply)
                }
                RegionCommand::Snapshot { reply } => {
                    let _ = reply.send(self.applied);
                }
                RegionCommand::LoadComplete { id, result } => self.complete_load(id, result),
                RegionCommand::SaveComplete { id, result } => self.complete_save(id, result),
            }
        }
    }

    fn set_block(
        &mut self,
        pos: BlockPos,
        requested: BlockState,
        reply: oneshot::Sender<Result<BlockMutation, RegionActorError>>,
    ) {
        let chunk_pos = pos.chunk();
        let before = self.chunks.get(&chunk_pos).map(|c| c.block_at_pos(pos));
        let after = self
            .chunks
            .get_mut(&chunk_pos)
            .and_then(|chunk| chunk.set_block(pos, requested));
        let state = after.or(before).unwrap_or(BlockState::Air);
        let mutation = BlockMutation {
            pos,
            chunk: chunk_pos,
            requested,
            state,
            loaded: before.is_some(),
            changed: after.is_some() && before != Some(state),
        };
        if !mutation.changed || !mutation.accepted() || self.storage.is_none() {
            let _ = reply.send(Ok(mutation));
            return;
        }
        let Some(chunk) = self.chunks.get(&chunk_pos).cloned() else {
            let _ = reply.send(Ok(mutation));
            return;
        };
        let id = self.next_job();
        self.pending_saves.insert(
            id,
            PendingSave {
                before: before.unwrap_or(BlockState::Air),
                mutation,
                reply,
            },
        );
        storage_jobs::save_chunk(id, self.storage.clone().unwrap(), chunk, self.outbox.clone());
    }

    fn spawn_chunks(
        &mut self,
        center: ChunkPos,
        radius: i32,
        reply: oneshot::Sender<Result<Vec<ChunkSnapshot>, RegionActorError>>,
    ) {
        let positions = self.world.chunk_positions(center, radius);
        let missing = positions
            .iter()
            .copied()
            .filter(|pos| !self.chunks.contains_key(pos))
            .collect::<Vec<_>>();
        if missing.is_empty() {
            let _ = reply.send(Ok(self.collect_chunks(&positions)));
            return;
        }
        let Some(storage) = self.storage.clone() else {
            for pos in missing {
                self.chunks.insert(pos, self.world.chunk_snapshot(pos));
            }
            let _ = reply.send(Ok(self.collect_chunks(&positions)));
            return;
        };
        let id = self.next_job();
        self.pending_loads
            .insert(id, PendingLoad { positions, reply });
        storage_jobs::load_chunks(id, storage, missing, self.outbox.clone());
    }

    fn complete_load(&mut self, id: u64, result: Result<Vec<ChunkSnapshot>, RegionActorError>) {
        let Some(pending) = self.pending_loads.remove(&id) else {
            return;
        };
        match result {
            Ok(chunks) => {
                chunks.into_iter().for_each(|c| {
                    self.chunks.insert(c.pos, c);
                });
                let _ = pending.reply.send(Ok(self.collect_chunks(&pending.positions)));
            }
            Err(error) => _ = pending.reply.send(Err(error)),
        }
    }

    fn complete_save(&mut self, id: u64, result: Result<(), RegionActorError>) {
        let Some(pending) = self.pending_saves.remove(&id) else {
            return;
        };
        match result {
            Ok(()) => {
                let _ = pending.reply.send(Ok(pending.mutation));
            }
            Err(error) => {
                tracing::warn!(region = self.id.0, %error, "chunk save failed");
                if let Some(chunk) = self.chunks.get_mut(&pending.mutation.chunk) {
                    chunk.set_block(pending.mutation.pos, pending.before);
                }
                let _ = pending.reply.send(Ok(BlockMutation {
                    state: pending.before,
                    ..pending.mutation
                }));
            }
        }
    }

    fn collect_chunks(&self, positions: &[ChunkPos]) -> Vec<ChunkSnapshot> {
        positions
            .iter()
            .filter_map(|pos| self.chunks.get(pos).cloned())
            .collect()
    }

    fn next_job(&mut self) -> u64 {
        let id = self.next_job;
        self.next_job += 1;
        id
    }
}
