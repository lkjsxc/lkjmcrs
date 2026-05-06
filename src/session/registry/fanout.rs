use super::{SessionId, SessionRegistry};
use crate::session::outbound::PlayOutbound;
use crate::world::ChunkPos;

impl SessionRegistry {
    pub(super) async fn broadcast(&self, message: PlayOutbound) -> usize {
        let mut sent = 0;
        let mut stale = Vec::new();
        let mut sessions = self.inner.sessions.lock().await;
        for (id, entry) in sessions.iter() {
            match entry.sender.try_send(message.clone()) {
                Ok(()) => sent += 1,
                Err(_) => stale.push(*id),
            }
        }
        for id in stale {
            sessions.remove(&id);
        }
        sent
    }

    pub(super) async fn broadcast_chunk(
        &self,
        chunk: ChunkPos,
        message: PlayOutbound,
        exclude: Option<SessionId>,
    ) -> usize {
        let mut sent = 0;
        let mut stale = Vec::new();
        let mut sessions = self.inner.sessions.lock().await;
        for (id, entry) in sessions.iter() {
            if Some(*id) == exclude || !entry.chunks.contains(&chunk) {
                continue;
            }
            match entry.sender.try_send(message.clone()) {
                Ok(()) => sent += 1,
                Err(_) => stale.push(*id),
            }
        }
        for id in stale {
            sessions.remove(&id);
        }
        sent
    }

    pub(super) async fn send_to_name(&self, name: &str, message: PlayOutbound) -> bool {
        let mut stale = None;
        let mut sent = false;
        let mut sessions = self.inner.sessions.lock().await;
        for (id, entry) in sessions.iter() {
            if !entry.name.eq_ignore_ascii_case(name) {
                continue;
            }
            match entry.sender.try_send(message.clone()) {
                Ok(()) => sent = true,
                Err(_) => stale = Some(*id),
            }
            break;
        }
        if let Some(id) = stale {
            sessions.remove(&id);
        }
        sent
    }
}
