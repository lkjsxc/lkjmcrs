use crate::scheduler::RegionHandle;
use crate::session::registry::{SessionId, SessionRegistry};

#[derive(Debug, Clone, Copy)]
pub struct StreamContext<'a> {
    pub region: &'a RegionHandle,
    pub sessions: &'a SessionRegistry,
    pub session_id: SessionId,
}
