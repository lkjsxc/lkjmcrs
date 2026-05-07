use crate::scheduler::region_command::{PendingLoad, PendingSave};
use crate::scheduler::region_state::RegionActor;
use crate::scheduler::storage_jobs;
use crate::scheduler::{BlockMutation, RegionActorError};
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot};
use tokio::sync::oneshot;

impl RegionActor {
    pub(super) fn set_block(
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
        let _ = reply.send(Ok(mutation));
        self.pending_saves.insert(
            id,
            PendingSave {
                mutation,
                attempts: 0,
            },
        );
        storage_jobs::save_chunk(
            id,
            self.storage.clone().unwrap(),
            chunk,
            self.outbox.clone(),
        );
    }

    pub(super) fn spawn_chunks(
        &mut self,
        center: ChunkPos,
        radius: i32,
        reply: oneshot::Sender<Result<Vec<ChunkSnapshot>, RegionActorError>>,
    ) {
        let positions = self.world.chunk_positions(center, radius);
        self.load_chunks(positions, reply);
    }

    pub(super) fn load_chunks(
        &mut self,
        positions: Vec<ChunkPos>,
        reply: oneshot::Sender<Result<Vec<ChunkSnapshot>, RegionActorError>>,
    ) {
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

    pub(super) fn complete_load(
        &mut self,
        id: u64,
        result: Result<Vec<ChunkSnapshot>, RegionActorError>,
    ) {
        let Some(pending) = self.pending_loads.remove(&id) else {
            return;
        };
        match result {
            Ok(chunks) => {
                chunks.into_iter().for_each(|c| {
                    self.chunks.insert(c.pos, c);
                });
                let _ = pending
                    .reply
                    .send(Ok(self.collect_chunks(&pending.positions)));
            }
            Err(error) => _ = pending.reply.send(Err(error)),
        }
    }

    pub(super) fn complete_save(&mut self, id: u64, result: Result<(), RegionActorError>) {
        let Some(pending) = self.pending_saves.remove(&id) else {
            return;
        };
        match result {
            Ok(()) => {
                tracing::trace!(region = self.id.0, "chunk save completed");
            }
            Err(error) => {
                tracing::warn!(region = self.id.0, %error, "chunk save failed");
                if pending.attempts >= 3 {
                    return;
                }
                if let (Some(storage), Some(chunk)) = (
                    self.storage.clone(),
                    self.chunks.get(&pending.mutation.chunk).cloned(),
                ) {
                    let retry = self.next_job();
                    self.pending_saves.insert(
                        retry,
                        PendingSave {
                            mutation: pending.mutation,
                            attempts: pending.attempts + 1,
                        },
                    );
                    storage_jobs::save_chunk(retry, storage, chunk, self.outbox.clone());
                }
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
