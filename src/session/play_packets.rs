use crate::protocol::block_interaction::BlockInteraction;
use crate::protocol::codec::{self, Packet};
use crate::protocol::ids;
use crate::protocol::movement::Movement;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_actions::handle_block_interaction;
use crate::session::error::ConnectionError;
use crate::session::io::codec_error;
use crate::session::play_state::PlaySession;
use crate::session::registry::SessionRegistry;
use std::io::Cursor;

pub async fn handle_play_packet(
    packet: Packet,
    phase: SessionState,
    session: &mut PlaySession,
    region: &RegionHandle,
    sessions: &SessionRegistry,
    writer: &mut (impl tokio::io::AsyncWrite + Unpin),
) -> Result<(), ConnectionError> {
    if let Some(movement) = Movement::decode(packet.id, packet.data.clone())
        .map_err(|error| codec_error(phase, error))?
    {
        session.apply_movement(movement);
        tracing::debug!(phase = %phase, packet_id = packet.id, "movement packet applied");
        return Ok(());
    }

    if let Some(interaction) = BlockInteraction::decode(packet.id, packet.data.clone())
        .map_err(|error| codec_error(phase, error))?
    {
        handle_block_interaction(interaction, phase, region, sessions, writer).await?;
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

pub(super) fn decode_keepalive(data: Vec<u8>) -> Result<i64, codec::CodecError> {
    let mut cursor = Cursor::new(data);
    let id = codec::read_i64(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(codec::CodecError::Eof);
    }
    Ok(id)
}
