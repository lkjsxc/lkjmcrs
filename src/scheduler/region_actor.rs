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
    inbox: mpsc::Receiver<RegionCommand>,
}

#[derive(Debug, Clone)]
pub struct RegionHandle {
    pub(super) id: RegionId,
    pub(super) outbox: mpsc::Sender<RegionCommand>,
}

#[derive(Debug)]
pub(super) enum RegionCommand {
    Apply {
        label: String,
        reply: oneshot::Sender<usize>,
    },
    SpawnChunks {
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
            inbox,
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
                RegionCommand::SpawnChunks { radius, reply } => {
                    let chunks = self.spawn_chunks(radius);
                    let _ = reply.send(chunks);
                }
                RegionCommand::ChunkSnapshot { pos, reply } => {
                    let _ = reply.send(self.chunks.get(&pos).cloned());
                }
                RegionCommand::GetBlock { pos, reply } => {
                    let block = self
                        .chunks
                        .get(&pos.chunk())
                        .map(|chunk| chunk.block_at_pos(pos));
                    let _ = reply.send(block);
                }
                RegionCommand::SetBlock { pos, state, reply } => {
                    let mutation = self.set_block(pos, state);
                    let _ = reply.send(mutation);
                }
                RegionCommand::Snapshot { reply } => {
                    let _ = reply.send(self.applied);
                }
            }
        }
    }

    fn set_block(
        &mut self,
        pos: BlockPos,
        requested: BlockState,
    ) -> Result<BlockMutation, RegionActorError> {
        let chunk_pos = pos.chunk();
        let before = self
            .chunks
            .get(&chunk_pos)
            .map(|chunk| chunk.block_at_pos(pos));
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
        if mutation.changed
            && mutation.accepted()
            && let (Some(storage), Some(chunk)) = (&self.storage, self.chunks.get(&chunk_pos))
        {
            storage.save_chunk(chunk)?;
        }
        Ok(mutation)
    }

    fn spawn_chunks(&mut self, radius: i32) -> Result<Vec<ChunkSnapshot>, RegionActorError> {
        for pos in self.world.spawn_chunk_positions(radius) {
            if self.chunks.contains_key(&pos) {
                continue;
            }
            let chunk = match &self.storage {
                Some(storage) => storage.load_chunk(pos)?,
                None => self.world.chunk_snapshot(pos),
            };
            self.chunks.insert(pos, chunk);
        }
        Ok(self
            .world
            .spawn_chunk_positions(radius)
            .into_iter()
            .filter_map(|pos| self.chunks.get(&pos).cloned())
            .collect())
    }
}
