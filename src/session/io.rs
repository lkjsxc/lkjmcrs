use crate::protocol::codec::{self, Packet};
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use tokio::net::TcpStream;

pub async fn expect_packet(
    stream: &mut TcpStream,
    phase: SessionState,
    expected: i32,
) -> Result<Packet, ConnectionError> {
    let packet = read_packet(stream, phase).await?;
    if packet.id == expected {
        Ok(packet)
    } else {
        Err(protocol_error(phase, "unexpected packet id"))
    }
}

pub async fn read_until_packet(
    stream: &mut TcpStream,
    phase: SessionState,
    expected: i32,
    ignored: &[i32],
) -> Result<Packet, ConnectionError> {
    loop {
        let packet = read_packet(stream, phase).await?;
        if packet.id == expected {
            return Ok(packet);
        }
        if ignored.contains(&packet.id) {
            tracing::debug!(phase = %phase, packet_id = packet.id, "packet ignored");
            continue;
        }
        return Err(protocol_error(phase, "unexpected packet id"));
    }
}

pub async fn read_packet(
    stream: &mut TcpStream,
    phase: SessionState,
) -> Result<Packet, ConnectionError> {
    codec::read_packet(stream)
        .await
        .map_err(|error| codec_error(phase, error))
}

pub async fn write_packet(
    stream: &mut TcpStream,
    phase: SessionState,
    id: i32,
    payload: &[u8],
) -> Result<(), ConnectionError> {
    codec::write_packet(stream, id, payload)
        .await
        .map_err(|error| codec_error(phase, error))
}

pub fn codec_error(phase: SessionState, source: codec::CodecError) -> ConnectionError {
    ConnectionError::codec(phase, source)
}

pub fn protocol_error(phase: SessionState, message: &'static str) -> ConnectionError {
    ConnectionError::Protocol { phase, message }
}

pub fn is_connection_closed(error: &ConnectionError) -> bool {
    matches!(
        error,
        ConnectionError::Codec {
            source: codec::CodecError::ConnectionClosed,
            ..
        }
    )
}
