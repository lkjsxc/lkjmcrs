use crate::scheduler::{BlockMutation, RegionActorError};
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, FlatWorld, RegionId};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct RegionActor {
    id: RegionId,
    applied: usize,
    chunks: HashMap<ChunkPos, ChunkSnapshot>,
    world: FlatWorld,
    inbox: mpsc::Receiver<RegionCommand>,
}

#[derive(Debug, Clone)]
pub struct RegionHandle {
    id: RegionId,
    outbox: mpsc::Sender<RegionCommand>,
}

#[derive(Debug)]
enum RegionCommand {
    Apply {
        label: String,
        reply: oneshot::Sender<usize>,
    },
    SpawnChunks {
        radius: i32,
        reply: oneshot::Sender<Vec<ChunkSnapshot>>,
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
        reply: oneshot::Sender<BlockMutation>,
    },
    Snapshot {
        reply: oneshot::Sender<usize>,
    },
}

impl RegionActor {
    pub fn spawn(id: RegionId) -> RegionHandle {
        let (outbox, inbox) = mpsc::channel(64);
        let actor = Self {
            id,
            applied: 0,
            chunks: HashMap::new(),
            world: FlatWorld::default(),
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

    fn set_block(&mut self, pos: BlockPos, requested: BlockState) -> BlockMutation {
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
        BlockMutation {
            pos,
            chunk: chunk_pos,
            requested,
            state,
            loaded: before.is_some(),
            changed: after.is_some() && before != Some(state),
        }
    }

    fn spawn_chunks(&mut self, radius: i32) -> Vec<ChunkSnapshot> {
        let chunks = self.world.spawn_chunks(radius);
        for chunk in chunks {
            self.chunks.entry(chunk.pos).or_insert(chunk);
        }
        self.world
            .spawn_chunk_positions(radius)
            .into_iter()
            .filter_map(|pos| self.chunks.get(&pos).cloned())
            .collect()
    }
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
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::SpawnChunks { radius, reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
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
        receive.await.map_err(|_| RegionActorError::Closed)
    }
}
