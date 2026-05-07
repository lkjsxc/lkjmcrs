use crate::player::{GameMode, PlayerProfile, PlayerStore};
use crate::session::SessionState;
use crate::session::chat::send_system_chat;
use crate::session::chunk_stream::ChunkStream;
use crate::session::commands::{self, ServerCommand};
use crate::session::error::ConnectionError;
use crate::session::game_mode::apply_game_mode;
use crate::session::play_state::PlaySession;
use crate::session::registry::SessionRegistry;
use crate::session::travel_commands;
use tokio::io::AsyncWrite;

pub struct CommandDispatchContext<'a, W>
where
    W: AsyncWrite + Unpin,
{
    pub phase: SessionState,
    pub max_players: usize,
    pub region: &'a crate::scheduler::RegionHandle,
    pub chunk_stream: &'a mut ChunkStream,
    pub player_store: &'a PlayerStore,
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
        ServerCommand::Spawn => travel_commands::spawn(session, profile, context).await,
        ServerCommand::SetHome(name) => {
            travel_commands::set_home(name, session, profile, context).await
        }
        ServerCommand::Home(name) => travel_commands::home(name, session, profile, context).await,
        ServerCommand::Homes => travel_commands::homes(profile, context).await,
        ServerCommand::SetWarp(name) => {
            travel_commands::set_warp(name, session, profile, context).await
        }
        ServerCommand::Warp(name) => travel_commands::warp(name, session, profile, context).await,
        ServerCommand::Warps => travel_commands::warps(context).await,
        ServerCommand::Say(message) => {
            context
                .sessions
                .broadcast_system_chat(format!("[Server] {message}"))
                .await;
            Ok(())
        }
        ServerCommand::Gamemode { mode, target } => gamemode(mode, target, profile, context).await,
        ServerCommand::Damage { target, amount } => damage(target, amount, context).await,
        ServerCommand::Vitals {
            target,
            health,
            hunger,
            saturation,
        } => vitals(target, health, hunger, saturation, context).await,
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
        "Commands: /help, /spawn, /sethome, /home, /homes, /warp, /warps, /say, /gamemode, /damage, /vitals, /kick",
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

async fn damage<W>(
    target: String,
    amount: f32,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if context.sessions.damage(&target, amount).await {
        send_system_chat(context.writer, context.phase, "Damage applied").await
    } else {
        send_system_chat(context.writer, context.phase, "Player not found").await
    }
}

async fn vitals<W>(
    target: String,
    health: f32,
    hunger: u8,
    saturation: f32,
    context: CommandDispatchContext<'_, W>,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if context
        .sessions
        .set_vitals(&target, health, hunger, saturation)
        .await
    {
        send_system_chat(context.writer, context.phase, "Vitals updated").await
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
