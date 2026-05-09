use crate::player::PlayerProfile;
use crate::protocol::block_interaction::BlockInteraction;
use crate::protocol::chat;
use crate::protocol::client_command;
use crate::protocol::codec::{self, Packet};
use crate::protocol::ids;
use crate::protocol::movement::Movement;
use crate::session::SessionState;
use crate::session::block_actions::handle_block_interaction;
use crate::session::chunk_stream::{ChunkStream, StreamContext};
use crate::session::error::ConnectionError;
use crate::session::io::codec_error;
use crate::session::movement_authority;
use crate::session::movement_correction::send_position_correction;
use crate::session::play_chat::handle_play_chat;
use crate::session::play_packet_context::PlayPacketContext;
use crate::session::play_state::PlaySession;
use crate::session::vitals;

pub async fn handle_play_packet<W>(
    packet: Packet,
    phase: SessionState,
    session: &mut PlaySession,
    chunk_stream: &mut ChunkStream,
    profile: &mut PlayerProfile,
    context: PlayPacketContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    if let Some(movement) = Movement::decode(packet.id, packet.data.clone())
        .map_err(|error| codec_error(phase, error))?
    {
        if let Err(rejection) = movement_authority::validate(session, movement) {
            send_position_correction(context.writer, phase, context.max_players, profile, session)
                .await?;
            tracing::warn!(
                phase = %phase,
                reason = rejection.reason,
                "movement rejected"
            );
            return Ok(());
        }
        session.apply_movement(movement);
        let stream_context = StreamContext {
            region: context.region,
            sessions: context.sessions,
            session_id: session.id,
        };
        chunk_stream
            .stream_after_movement(
                session.x,
                session.z,
                phase,
                stream_context,
                context.writer,
                context.chunk_cache,
            )
            .await?;
        crate::session::item_pickup::attempt_pickup(
            context.writer,
            phase,
            context.region,
            context.sessions,
            session,
            profile,
        )
        .await?;
        tracing::debug!(phase = %phase, packet_id = packet.id, "movement packet applied");
        return Ok(());
    }

    if let Some(interaction) = BlockInteraction::decode(packet.id, packet.data.clone())
        .map_err(|error| codec_error(phase, error))?
    {
        handle_block_interaction(
            interaction,
            phase,
            context.region,
            context.sessions,
            context.writer,
            session,
            profile,
        )
        .await?;
        return Ok(());
    }

    if let Some(chat) =
        chat::decode(packet.id, packet.data.clone()).map_err(|error| codec_error(phase, error))?
    {
        handle_play_chat(chat, phase, session, chunk_stream, profile, context).await?;
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
                    expected = session.keepalive_id().unwrap_or(0),
                    "keepalive response id mismatch"
                );
            }
        }
        ids::play::SERVERBOUND_CLIENT_COMMAND => {
            let action = client_command::decode_action(packet.data)
                .map_err(|error| codec_error(phase, error))?;
            if action == 0 {
                vitals::respawn(
                    context.writer,
                    phase,
                    context.max_players,
                    context.spawn_position,
                    profile,
                    session,
                )
                .await?;
            }
        }
        _ => {
            tracing::debug!(phase = %phase, packet_id = packet.id, "play packet ignored");
        }
    }
    Ok(())
}

pub(super) fn decode_keepalive(data: Vec<u8>) -> Result<i64, codec::CodecError> {
    use std::io::Cursor;
    let mut cursor = Cursor::new(data);
    let id = codec::read_i64(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(codec::CodecError::Eof);
    }
    Ok(id)
}
