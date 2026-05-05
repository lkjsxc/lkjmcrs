use crate::protocol::chat;
use crate::protocol::ids;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use tokio::io::AsyncWrite;

pub async fn send_system_chat<W>(
    writer: &mut W,
    phase: SessionState,
    message: &str,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::SYSTEM_CHAT,
        &chat::encode_system_chat(message),
    )
    .await
}

pub async fn send_kick<W>(
    writer: &mut W,
    phase: SessionState,
    reason: &str,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::KICK_DISCONNECT,
        &chat::encode_kick(reason),
    )
    .await
}
