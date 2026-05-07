use crate::player::{PlayerProfile, PlayerStore};
use crate::protocol::block_interaction::BlockInteraction;
use crate::protocol::chat::{self, PlayChat};
use crate::protocol::client_command;
use crate::protocol::codec::{self, Packet};
use crate::protocol::ids;
use crate::protocol::movement::Movement;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_actions::handle_block_interaction;
use crate::session::chat::send_system_chat;
use crate::session::chunk_stream::{ChunkStream, StreamContext};
use crate::session::command_dispatch::{self, CommandDispatchContext};
use crate::session::error::ConnectionError;
use crate::session::inventory_sync;
use crate::session::io::codec_error;
use crate::session::play_state::PlaySession;
use crate::session::registry::SessionRegistry;
use crate::session::vitals;

pub struct PlayPacketContext<'a, W>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    pub region: &'a RegionHandle,
    pub sessions: &'a SessionRegistry,
    pub max_players: usize,
    pub player_store: &'a PlayerStore,
    pub writer: &'a mut W,
}

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
        session.apply_movement(movement);
        let stream_context = StreamContext {
            region: context.region,
            sessions: context.sessions,
            session_id: session.id,
        };
        chunk_stream
            .stream_after_movement(session.x, session.z, phase, stream_context, context.writer)
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
        handle_chat(chat, phase, session, chunk_stream, profile, context).await?;
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
                vitals::respawn(context.writer, phase, context.max_players, profile, session)
                    .await?;
            }
        }
        _ => {
            tracing::debug!(phase = %phase, packet_id = packet.id, "play packet ignored");
        }
    }
    Ok(())
}

async fn handle_chat<W>(
    chat: PlayChat,
    phase: SessionState,
    session: &mut PlaySession,
    chunk_stream: &mut ChunkStream,
    profile: &mut PlayerProfile,
    context: PlayPacketContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    match chat {
        PlayChat::Message(message) => {
            context
                .sessions
                .broadcast_system_chat(format!("<{}> {message}", profile.name))
                .await;
            Ok(())
        }
        PlayChat::Command(command) => {
            command_dispatch::dispatch(
                &command,
                session,
                profile,
                CommandDispatchContext {
                    phase,
                    max_players: context.max_players,
                    region: context.region,
                    chunk_stream,
                    player_store: context.player_store,
                    sessions: context.sessions,
                    writer: context.writer,
                },
            )
            .await
        }
        PlayChat::HeldSlot(slot) if (0..=8).contains(&slot) => {
            profile.inventory.selected_hotbar_slot = slot as u8;
            inventory_sync::send_held_item_slot(
                context.writer,
                phase,
                profile.inventory.selected_hotbar_slot,
            )
            .await?;
            Ok(())
        }
        PlayChat::HeldSlot(_) => {
            inventory_sync::send_held_item_slot(
                context.writer,
                phase,
                profile.inventory.selected_hotbar_slot,
            )
            .await?;
            send_system_chat(context.writer, phase, "Invalid held item slot").await
        }
    }
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
