use crate::player::{GameMode, PlayerProfile};
use crate::protocol::gameplay;
use crate::protocol::ids;
use crate::protocol::play;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use tokio::io::AsyncWrite;

pub async fn apply_game_mode<W>(
    writer: &mut W,
    phase: SessionState,
    max_players: usize,
    profile: &mut PlayerProfile,
    game_mode: GameMode,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    profile.game_mode = game_mode;
    let bootstrap = play::Bootstrap::new(max_players).with_player_state(
        (profile.position.x, profile.position.y, profile.position.z),
        (profile.position.yaw, profile.position.pitch),
        (game_mode.vanilla_id(), game_mode.ability_flags()),
    );
    write_packet(
        writer,
        phase,
        ids::play::GAME_STATE_CHANGE,
        &gameplay::encode_game_mode_change(game_mode.vanilla_id()),
    )
    .await?;
    write_packet(
        writer,
        phase,
        ids::play::PLAYER_ABILITIES,
        &play::encode_player_abilities_for(bootstrap),
    )
    .await
}
