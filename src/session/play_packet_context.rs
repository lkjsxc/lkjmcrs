use crate::player::PlayerStore;
use crate::scheduler::RegionHandle;
use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::registry::SessionRegistry;

pub struct PlayPacketContext<'a, W>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    pub region: &'a RegionHandle,
    pub sessions: &'a SessionRegistry,
    pub max_players: usize,
    pub player_store: &'a PlayerStore,
    pub writer: &'a mut W,
    pub chunk_cache: &'a mut ChunkPayloadCache,
}
