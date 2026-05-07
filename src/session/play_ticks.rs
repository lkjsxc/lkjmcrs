use crate::player::PlayerProfile;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::hunger::{self, HungerTick};
use crate::session::play_state::PlaySession;
use tokio::io::AsyncWrite;

pub async fn handle_hunger<W>(
    writer: &mut W,
    phase: SessionState,
    profile: &mut PlayerProfile,
    session: &mut PlaySession,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    match hunger::tick(profile, session) {
        HungerTick::None => Ok(()),
        HungerTick::Changed => crate::session::vitals::send_update(writer, phase, profile).await,
        HungerTick::Starve => {
            crate::session::vitals::apply_damage(writer, phase, profile, session, 1.0).await
        }
    }
}
