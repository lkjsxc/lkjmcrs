use crate::protocol::codec;
use crate::protocol::ids;
use crate::protocol::status;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::handler::ServerContext;
use crate::session::io::{is_connection_closed, protocol_error, read_packet, write_packet};
use std::sync::Arc;
use tokio::net::TcpStream;

pub async fn handle(
    mut stream: TcpStream,
    context: Arc<ServerContext>,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Status;
    let request = read_packet(&mut stream, phase).await?;
    if request.id != ids::status::REQUEST {
        return Err(protocol_error(phase, "expected status request"));
    }
    let json = status::response_json(
        &context.config.motd,
        context.player_count(),
        context.config.max_players,
    )
    .map_err(|source| ConnectionError::Json { phase, source })?;
    let mut payload = Vec::new();
    codec::write_string(&mut payload, &json);
    write_packet(&mut stream, phase, ids::status::RESPONSE, &payload).await?;
    handle_optional_ping(&mut stream).await
}

async fn handle_optional_ping(stream: &mut TcpStream) -> Result<(), ConnectionError> {
    let phase = SessionState::Status;
    match read_packet(stream, phase).await {
        Ok(ping) if ping.id == ids::status::PING => {
            write_packet(stream, phase, ids::status::PONG, &ping.data).await?;
        }
        Ok(_) => return Err(protocol_error(phase, "expected status ping")),
        Err(error) if is_connection_closed(&error) => {}
        Err(error) => return Err(error),
    }
    Ok(())
}
