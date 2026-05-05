use crate::session::outbound::PlayOutbound;
use crate::world::{BlockPos, BlockState, ChunkPos};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Mutex;
use tokio::sync::mpsc;

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
    pub async fn register(&self) -> (SessionId, mpsc::Receiver<PlayOutbound>) {
        let id = SessionId(self.inner.next_id.fetch_add(1, Ordering::Relaxed));
        let (sender, receiver) = mpsc::channel(OUTBOUND_CAPACITY);
        let entry = SessionEntry {
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

    pub async fn unregister(&self, id: SessionId) {
        self.inner.sessions.lock().await.remove(&id);
    }

    pub async fn broadcast_block_update(
        &self,
        chunk: ChunkPos,
        pos: BlockPos,
        state: BlockState,
    ) -> usize {
        let message = PlayOutbound::BlockUpdate { pos, state };
        let mut sent = 0;
        let mut stale = Vec::new();
        let mut sessions = self.inner.sessions.lock().await;
        for (id, entry) in sessions.iter() {
            if !entry.chunks.contains(&chunk) {
                continue;
            }
            match entry.sender.try_send(message) {
                Ok(()) => sent += 1,
                Err(_) => stale.push(*id),
            }
        }
        for id in stale {
            sessions.remove(&id);
        }
        sent
    }
}
