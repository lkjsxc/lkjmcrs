use crate::player::PlayerProfile;
use crate::session::SessionState;
use crate::session::block_packets::send_block_update;
use crate::session::chat::{send_kick, send_system_chat};
use crate::session::entity_packets::{send_collect, send_destroy, send_item_spawn};
use crate::session::error::ConnectionError;
use crate::session::game_mode::apply_game_mode;
use crate::session::outbound::PlayOutbound;
use crate::session::play_state::PlaySession;
use crate::session::vitals;
use tokio::io::AsyncWrite;

pub enum OutboundStep {
    Continue,
    Close,
}

pub async fn handle_outbound<W>(
    writer: &mut W,
    phase: SessionState,
    max_players: usize,
    profile: &mut PlayerProfile,
    session: &mut PlaySession,
    message: Option<PlayOutbound>,
) -> Result<OutboundStep, ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    match message {
        Some(PlayOutbound::BlockUpdate { pos, state }) => {
            send_block_update(writer, phase, pos, state).await?;
        }
        Some(PlayOutbound::SystemChat { message }) => {
            send_system_chat(writer, phase, &message).await?;
        }
        Some(PlayOutbound::ApplyGameMode { game_mode }) => {
            apply_game_mode(writer, phase, max_players, profile, game_mode).await?;
        }
        Some(PlayOutbound::Damage { amount }) => {
            vitals::apply_damage(writer, phase, profile, session, amount).await?;
        }
        Some(PlayOutbound::Kick { reason }) => {
            send_kick(writer, phase, &reason).await?;
            return Ok(OutboundStep::Close);
        }
        Some(PlayOutbound::ItemSpawn { item }) => {
            send_item_spawn(writer, phase, &item).await?;
        }
        Some(PlayOutbound::ItemCollect { item, collector }) => {
            send_collect(writer, phase, &item, collector).await?;
        }
        Some(PlayOutbound::ItemDestroy { entity_id }) => {
            send_destroy(writer, phase, entity_id).await?;
        }
        None => return Ok(OutboundStep::Close),
    }
    Ok(OutboundStep::Continue)
}
