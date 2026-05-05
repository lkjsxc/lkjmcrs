#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
    Closed,
}

impl SessionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Handshake => "handshake",
            Self::Status => "status",
            Self::Login => "login",
            Self::Configuration => "configuration",
            Self::Play => "play",
            Self::Closed => "closed",
        }
    }
}

impl std::fmt::Display for SessionState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
