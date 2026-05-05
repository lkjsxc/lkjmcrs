use crate::protocol::codec::{self, Packet};
use crate::protocol::ids;
use crate::protocol::movement::Movement;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::codec_error;
use crate::session::play_state::PlaySession;
use std::io::Cursor;

pub fn handle_play_packet(
    packet: Packet,
    phase: SessionState,
    session: &mut PlaySession,
) -> Result<(), ConnectionError> {
    if let Some(movement) = Movement::decode(packet.id, packet.data.clone())
        .map_err(|error| codec_error(phase, error))?
    {
        session.apply_movement(movement);
        tracing::debug!(phase = %phase, packet_id = packet.id, "movement packet applied");
        return Ok(());
    }

    match packet.id {
        ids::play::SERVERBOUND_TELEPORT_CONFIRM
        | ids::play::SERVERBOUND_CHUNK_BATCH_RECEIVED
        | ids::play::SERVERBOUND_SETTINGS
        | ids::play::SERVERBOUND_PLAYER_LOADED
        | ids::play::SERVERBOUND_PONG => {
            tracing::debug!(phase = %phase, packet_id = packet.id, "play packet accepted");
        }
        ids::play::SERVERBOUND_KEEPALIVE => {
            let id = decode_keepalive(packet.data).map_err(|error| codec_error(phase, error))?;
            if session.keepalive_matches(id) {
                tracing::debug!(phase = %phase, id, "keepalive response accepted");
            } else {
                tracing::warn!(
                    phase = %phase,
                    id,
                    expected = session.last_keepalive_id,
                    "keepalive response id mismatch"
                );
            }
        }
        _ => {
            tracing::debug!(phase = %phase, packet_id = packet.id, "play packet ignored");
        }
    }
    Ok(())
}

fn decode_keepalive(data: Vec<u8>) -> Result<i64, codec::CodecError> {
    let mut cursor = Cursor::new(data);
    let id = codec::read_i64(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(codec::CodecError::Eof);
    }
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::decode_keepalive;
    use crate::protocol::codec;

    #[test]
    fn keepalive_decode_rejects_trailing_bytes() {
        let mut data = Vec::new();
        codec::write_i64(&mut data, 7);
        data.push(0);
        assert!(decode_keepalive(data).is_err());
    }
}
