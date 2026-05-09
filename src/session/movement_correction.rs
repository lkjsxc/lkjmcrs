use crate::player::PlayerProfile;
use crate::protocol::{ids, play};
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::play_state::PlaySession;

pub(super) async fn send_position_correction<W>(
    writer: &mut W,
    phase: SessionState,
    max_players: usize,
    profile: &PlayerProfile,
    session: &PlaySession,
) -> Result<(), ConnectionError>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    let bootstrap = play::Bootstrap::new(max_players).with_player_state(
        (session.x, session.y, session.z),
        (session.yaw, session.pitch),
        (
            profile.game_mode.vanilla_id(),
            profile.game_mode.ability_flags(),
        ),
    );
    write_packet(
        writer,
        phase,
        ids::play::PLAYER_POSITION,
        &play::encode_initial_position(bootstrap),
    )
    .await
}
