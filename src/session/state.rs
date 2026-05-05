#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
    Closed,
}
