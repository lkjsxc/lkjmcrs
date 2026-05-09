use crate::player::PlayerPosition;
use crate::session::outbound::PlayOutbound;
use crate::session::registry::SessionId;

#[derive(Debug, Clone, Copy)]
pub struct PlaySettings {
    pub max_players: usize,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub spawn_position: PlayerPosition,
}

pub(super) struct RegisteredSession {
    pub id: SessionId,
    pub outbound: tokio::sync::mpsc::Receiver<PlayOutbound>,
    pub is_op: bool,
}
