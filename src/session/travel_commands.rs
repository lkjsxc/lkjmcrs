use crate::player::{NamedLocation, PlayerPosition, PlayerProfile};
use crate::protocol::ids;
use crate::protocol::play;
use crate::session::SessionState;
use crate::session::chat::send_system_chat;
use crate::session::command_dispatch::CommandDispatchContext;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::play_state::PlaySession;
use tokio::io::AsyncWrite;

pub(super) async fn spawn<W>(
    session: &mut PlaySession,
    profile: &mut PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    teleport_to(session, profile, PlayerPosition::default(), context).await
}

pub(super) async fn set_home<W>(
    name: String,
    session: &PlaySession,
    profile: &PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let location = current_location(name.clone(), session);
    context
        .player_store
        .set_home(profile.uuid, location)
        .await
        .map_err(|source| player_error(context.phase, source))?;
    send_system_chat(
        context.writer,
        context.phase,
        &format!("Home saved: {name}"),
    )
    .await
}

pub(super) async fn home<W>(
    name: String,
    session: &mut PlaySession,
    profile: &mut PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let location = context
        .player_store
        .home(profile.uuid, name)
        .await
        .map_err(|source| player_error(context.phase, source))?;
    match location {
        Some(location) => teleport_to(session, profile, location.position, context).await,
        None => send_system_chat(context.writer, context.phase, "Home not found").await,
    }
}

pub(super) async fn homes<W>(
    profile: &PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let names = context
        .player_store
        .home_names(profile.uuid)
        .await
        .map_err(|source| player_error(context.phase, source))?;
    send_name_list(context.writer, context.phase, "Homes", names).await
}

pub(super) async fn set_warp<W>(
    name: String,
    session: &PlaySession,
    profile: &PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let location = current_location(name.clone(), session);
    context
        .player_store
        .set_warp(profile.uuid, location)
        .await
        .map_err(|source| player_error(context.phase, source))?;
    send_system_chat(
        context.writer,
        context.phase,
        &format!("Warp saved: {name}"),
    )
    .await
}

pub(super) async fn warp<W>(
    name: String,
    session: &mut PlaySession,
    profile: &mut PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let location = context
        .player_store
        .warp(name)
        .await
        .map_err(|source| player_error(context.phase, source))?;
    match location {
        Some(location) => teleport_to(session, profile, location.position, context).await,
        None => send_system_chat(context.writer, context.phase, "Warp not found").await,
    }
}

pub(super) async fn warps<W>(context: CommandDispatchContext<'_, W>) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let names = context
        .player_store
        .warp_names()
        .await
        .map_err(|source| player_error(context.phase, source))?;
    send_name_list(context.writer, context.phase, "Warps", names).await
}

async fn teleport_to<W>(
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
    let bootstrap = play::Bootstrap::new(context.max_players).with_player_state(
        (session.x, session.y, session.z),
        (session.yaw, session.pitch),
        (
            profile.game_mode.vanilla_id(),
            profile.game_mode.ability_flags(),
        ),
    );
    write_packet(
        context.writer,
        context.phase,
        ids::play::PLAYER_POSITION,
        &play::encode_initial_position(bootstrap),
    )
    .await
}

async fn send_name_list<W>(
    writer: &mut W,
    phase: SessionState,
    label: &str,
    names: Vec<String>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let value = if names.is_empty() {
        format!("{label}: none")
    } else {
        format!("{label}: {}", names.join(", "))
    };
    send_system_chat(writer, phase, &value).await
}

fn current_location(name: String, session: &PlaySession) -> NamedLocation {
    NamedLocation::overworld(
        name,
        PlayerPosition {
            x: session.x,
            y: session.y,
            z: session.z,
            yaw: session.yaw,
            pitch: session.pitch,
        },
    )
}

fn player_error(phase: SessionState, source: crate::player::PlayerStoreError) -> ConnectionError {
    ConnectionError::Player { phase, source }
}
