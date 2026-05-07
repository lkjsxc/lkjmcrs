use crate::player::{PlayerPosition, PlayerProfile};
use crate::protocol::{ids, play};
use crate::session::chunk_stream::StreamContext;
use crate::session::command_dispatch::CommandDispatchContext;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::play_state::PlaySession;
use tokio::io::AsyncWrite;

pub(super) async fn teleport_to<W>(
    session: &mut PlaySession,
    profile: &mut PlayerProfile,
    position: PlayerPosition,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    session.x = position.x;
    session.y = position.y;
    session.z = position.z;
    session.yaw = position.yaw;
    session.pitch = position.pitch;
    profile.position = position;
    send_position(
        &mut *context.writer,
        context.phase,
        context.max_players,
        session,
        profile,
    )
    .await?;
    context
        .chunk_stream
        .stream_after_movement(
            session.x,
            session.z,
            context.phase,
            StreamContext {
                region: context.region,
                sessions: context.sessions,
                session_id: session.id,
            },
            context.writer,
            context.chunk_cache,
        )
        .await
}

async fn send_position<W>(
    writer: &mut W,
    phase: crate::session::SessionState,
    max_players: usize,
    session: &PlaySession,
    profile: &PlayerProfile,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
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
