use crate::player::PlayerProfile;
use crate::protocol::chat::PlayChat;
use crate::session::SessionState;
use crate::session::chat::send_system_chat;
use crate::session::chunk_stream::ChunkStream;
use crate::session::command_dispatch::{self, CommandDispatchContext};
use crate::session::error::ConnectionError;
use crate::session::inventory_sync;
use crate::session::play_packet_context::PlayPacketContext;
use crate::session::play_state::PlaySession;

pub(super) async fn handle_play_chat<W>(
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
                    spawn_position: context.spawn_position,
                    region: context.region,
                    chunk_stream,
                    chunk_cache: context.chunk_cache,
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
            .await
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
