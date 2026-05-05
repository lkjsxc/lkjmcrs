use crate::protocol::codec;
use crate::session::SessionState;
use crate::session::profile::ProfileError;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionLogLevel {
    Debug,
    Info,
    Warn,
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("{phase}: {source}")]
    Codec {
        phase: SessionState,
        #[source]
        source: codec::CodecError,
    },
    #[error("{phase}: {message}")]
    Protocol {
        phase: SessionState,
        message: &'static str,
    },
    #[error("{phase}: {source}")]
    Json {
        phase: SessionState,
        #[source]
        source: serde_json::Error,
    },
    #[error("{phase}: {source}")]
    Profile {
        phase: SessionState,
        #[source]
        source: ProfileError,
    },
}

impl ConnectionError {
    pub fn codec(phase: SessionState, source: codec::CodecError) -> Self {
        Self::Codec { phase, source }
    }

    pub const fn phase(&self) -> SessionState {
        match self {
            Self::Codec { phase, .. }
            | Self::Protocol { phase, .. }
            | Self::Json { phase, .. }
            | Self::Profile { phase, .. } => *phase,
        }
    }

    pub const fn log_level(&self) -> ConnectionLogLevel {
        match self {
            Self::Codec {
                phase: SessionState::Play,
                source: codec::CodecError::ConnectionClosed,
            } => ConnectionLogLevel::Info,
            Self::Codec {
                source: codec::CodecError::ConnectionClosed,
                ..
            } => ConnectionLogLevel::Debug,
            _ => ConnectionLogLevel::Warn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConnectionError, ConnectionLogLevel};
    use crate::protocol::codec::CodecError;
    use crate::session::SessionState;

    #[test]
    fn normal_pre_play_close_is_not_warning() {
        let error =
            ConnectionError::codec(SessionState::Configuration, CodecError::ConnectionClosed);
        assert_eq!(error.log_level(), ConnectionLogLevel::Debug);
    }

    #[test]
    fn normal_play_close_is_info() {
        let error = ConnectionError::codec(SessionState::Play, CodecError::ConnectionClosed);
        assert_eq!(error.log_level(), ConnectionLogLevel::Info);
    }

    #[test]
    fn malformed_packets_are_warnings() {
        let error = ConnectionError::codec(SessionState::Login, CodecError::Eof);
        assert_eq!(error.log_level(), ConnectionLogLevel::Warn);
    }
}
