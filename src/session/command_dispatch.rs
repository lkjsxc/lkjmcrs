use crate::player::{GameMode, PlayerPosition, PlayerProfile};
use crate::protocol::ids;
use crate::protocol::play;
use crate::session::SessionState;
use crate::session::chat::send_system_chat;
use crate::session::commands::{self, ServerCommand};
use crate::session::error::ConnectionError;
use crate::session::game_mode::apply_game_mode;
use crate::session::io::write_packet;
use crate::session::play_state::PlaySession;
use crate::session::registry::SessionRegistry;
use tokio::io::AsyncWrite;

pub struct CommandDispatchContext<'a, W>
where
    W: AsyncWrite + Unpin,
{
    pub phase: SessionState,
    pub max_players: usize,
    pub sessions: &'a SessionRegistry,
    pub writer: &'a mut W,
}

pub async fn dispatch<W>(
    input: &str,
    session: &mut PlaySession,
    profile: &mut PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let command = match commands::parse(input) {
        Ok(command) => command,
        Err(message) => return send_system_chat(context.writer, context.phase, message).await,
    };
    if command.requires_op() && !session.is_op {
        return send_system_chat(context.writer, context.phase, "Permission denied").await;
    }
    match command {
        ServerCommand::Help => send_help(context.writer, context.phase).await,
        ServerCommand::Spawn => spawn(session, profile, context).await,
        ServerCommand::Say(message) => {
            context
                .sessions
                .broadcast_system_chat(format!("[Server] {message}"))
                .await;
            Ok(())
        }
        ServerCommand::Gamemode { mode, target } => gamemode(mode, target, profile, context).await,
        ServerCommand::Kick { target, reason } => kick(target, reason, context).await,
    }
}

async fn send_help<W>(writer: &mut W, phase: SessionState) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    send_system_chat(
        writer,
        phase,
        "Commands: /help, /spawn, /say, /gamemode, /kick",
    )
    .await
}

async fn spawn<W>(
    session: &mut PlaySession,
    profile: &mut PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    session.move_to_spawn();
    profile.position = PlayerPosition {
        x: session.x,
        y: session.y,
        z: session.z,
        yaw: session.yaw,
        pitch: session.pitch,
    };
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

async fn gamemode<W>(
    mode: GameMode,
    target: Option<String>,
    profile: &mut PlayerProfile,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let target_name = target.unwrap_or_else(|| profile.name.clone());
    if target_name.eq_ignore_ascii_case(&profile.name) {
        apply_game_mode(
            context.writer,
            context.phase,
            context.max_players,
            profile,
            mode,
        )
        .await?;
        return send_system_chat(context.writer, context.phase, "Gamemode updated").await;
    }
    if context.sessions.apply_gamemode(&target_name, mode).await {
        send_system_chat(context.writer, context.phase, "Gamemode updated").await
    } else {
        send_system_chat(context.writer, context.phase, "Player not found").await
    }
}

async fn kick<W>(
    target: String,
    reason: String,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if context.sessions.kick(&target, reason).await {
        send_system_chat(context.writer, context.phase, "Player kicked").await
    } else {
        send_system_chat(context.writer, context.phase, "Player not found").await
    }
}
