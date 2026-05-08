use crate::player::{GameMode, PlayerProfile};
use crate::session::outbound::PlayOutbound;
use crate::world::{BlockPos, BlockState, ChunkPos, DroppedItemEntity};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Mutex;
use tokio::sync::mpsc;

mod fanout;

const OUTBOUND_CAPACITY: usize = 64;

#[derive(Debug, Clone)]
pub struct SessionRegistry {
    inner: Arc<RegistryInner>,
}

#[derive(Debug)]
struct RegistryInner {
    next_id: AtomicUsize,
    sessions: Mutex<HashMap<SessionId, SessionEntry>>,
}

#[derive(Debug)]
struct SessionEntry {
    name: String,
    chunks: HashSet<ChunkPos>,
    sender: mpsc::Sender<PlayOutbound>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub usize);

impl Default for SessionRegistry {
    fn default() -> Self {
        Self {
            inner: Arc::new(RegistryInner {
                next_id: AtomicUsize::new(1),
                sessions: Mutex::new(HashMap::new()),
            }),
        }
    }
}

impl SessionRegistry {
    pub async fn register(
        &self,
        profile: &PlayerProfile,
    ) -> (SessionId, mpsc::Receiver<PlayOutbound>) {
        let id = SessionId(self.inner.next_id.fetch_add(1, Ordering::Relaxed));
        let (sender, receiver) = mpsc::channel(OUTBOUND_CAPACITY);
        let entry = SessionEntry {
            name: profile.name.clone(),
            chunks: HashSet::new(),
            sender,
        };
        self.inner.sessions.lock().await.insert(id, entry);
        (id, receiver)
    }

    pub async fn subscribe<I>(&self, id: SessionId, chunks: I)
    where
        I: IntoIterator<Item = ChunkPos>,
    {
        if let Some(entry) = self.inner.sessions.lock().await.get_mut(&id) {
            entry.chunks.extend(chunks);
        }
    }

    pub async fn unsubscribe<I>(&self, id: SessionId, chunks: I)
    where
        I: IntoIterator<Item = ChunkPos>,
    {
        if let Some(entry) = self.inner.sessions.lock().await.get_mut(&id) {
            for chunk in chunks {
                entry.chunks.remove(&chunk);
            }
        }
    }

    pub async fn unregister(&self, id: SessionId) {
        self.inner.sessions.lock().await.remove(&id);
    }

    pub async fn active_count(&self) -> usize {
        self.inner.sessions.lock().await.len()
    }

    pub async fn broadcast_block_update(
        &self,
        chunk: ChunkPos,
        pos: BlockPos,
        state: BlockState,
        exclude: Option<SessionId>,
    ) -> usize {
        self.broadcast_chunk(chunk, PlayOutbound::BlockUpdate { pos, state }, exclude)
            .await
    }

    pub async fn broadcast_system_chat(&self, message: String) -> usize {
        self.broadcast(PlayOutbound::SystemChat { message }).await
    }

    pub async fn broadcast_item_spawn(&self, chunk: ChunkPos, item: DroppedItemEntity) -> usize {
        self.broadcast_chunk(chunk, PlayOutbound::ItemSpawn { item }, None)
            .await
    }

    pub async fn broadcast_item_collect(
        &self,
        chunk: ChunkPos,
        item: DroppedItemEntity,
        collector: i32,
        exclude: SessionId,
    ) -> usize {
        self.broadcast_chunk(
            chunk,
            PlayOutbound::ItemCollect { item, collector },
            Some(exclude),
        )
        .await
    }

    pub async fn broadcast_item_destroy(
        &self,
        chunk: ChunkPos,
        entity_id: i32,
        exclude: SessionId,
    ) -> usize {
        self.broadcast_chunk(
            chunk,
            PlayOutbound::ItemDestroy { entity_id },
            Some(exclude),
        )
        .await
    }

    pub async fn apply_gamemode(&self, name: &str, game_mode: GameMode) -> bool {
        self.send_to_name(name, PlayOutbound::ApplyGameMode { game_mode })
            .await
    }

    pub async fn damage(&self, name: &str, amount: f32) -> bool {
        self.send_to_name(name, PlayOutbound::Damage { amount })
            .await
    }

    pub async fn set_vitals(&self, name: &str, health: f32, hunger: u8, saturation: f32) -> bool {
        self.send_to_name(
            name,
            PlayOutbound::SetVitals {
                health,
                hunger,
                saturation,
            },
        )
        .await
    }

    pub async fn kick(&self, name: &str, reason: String) -> bool {
        self.send_to_name(name, PlayOutbound::Kick { reason }).await
    }
}
