use crate::player::{GameMode, PlayerProfile};
use crate::protocol::ids;
use crate::protocol::play;
use crate::protocol::vitals::{self, HealthUpdate};
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::session::play_state::PlaySession;
use tokio::io::AsyncWrite;

const PLAYER_ENTITY_ID: i32 = 1;

pub async fn apply_damage<W>(
    writer: &mut W,
    phase: SessionState,
    profile: &mut PlayerProfile,
    session: &mut PlaySession,
    amount: f32,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if session.dead {
        return Ok(());
    }
    profile.vitals.apply_damage(amount);
    send_update(writer, phase, profile).await?;
    if profile.vitals.is_dead() {
        session.dead = true;
        send_death(writer, phase, &profile.name).await?;
    }
    Ok(())
}

pub async fn set_values<W>(
    writer: &mut W,
    phase: SessionState,
    profile: &mut PlayerProfile,
    session: &mut PlaySession,
    health: f32,
    hunger: u8,
    saturation: f32,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    profile.vitals.health = health.clamp(0.0, 20.0);
    profile.vitals.hunger = hunger.min(20);
    profile.vitals.saturation = saturation.clamp(0.0, 20.0);
    send_update(writer, phase, profile).await?;
    if profile.vitals.is_dead() {
        if !session.dead {
            session.dead = true;
            send_death(writer, phase, &profile.name).await?;
        }
    } else {
        session.dead = false;
    }
    Ok(())
}

pub async fn respawn<W>(
    writer: &mut W,
    phase: SessionState,
    max_players: usize,
    spawn_position: crate::player::PlayerPosition,
    profile: &mut PlayerProfile,
    session: &mut PlaySession,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    if !session.dead {
        return Ok(());
    }
    profile.vitals.reset();
    session.dead = false;
    session.move_to(spawn_position);
    session.copy_position_to_profile(profile);
    let bootstrap = bootstrap_from_profile(max_players, profile);
    write_packet(
        writer,
        phase,
        ids::play::RESPAWN,
        &play::encode_respawn(bootstrap),
    )
    .await?;
    write_packet(
        writer,
        phase,
        ids::play::PLAYER_POSITION,
        &play::encode_initial_position(bootstrap),
    )
    .await?;
    send_update(writer, phase, profile).await
}

pub async fn send_update<W>(
    writer: &mut W,
    phase: SessionState,
    profile: &PlayerProfile,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::UPDATE_HEALTH,
        &vitals::encode_update_health(HealthUpdate {
            health: profile.vitals.health,
            hunger: i32::from(profile.vitals.hunger),
            saturation: profile.vitals.saturation,
        }),
    )
    .await
}

async fn send_death<W>(
    writer: &mut W,
    phase: SessionState,
    name: &str,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let message = format!("{name} died");
    write_packet(
        writer,
        phase,
        ids::play::DEATH_COMBAT_EVENT,
        &vitals::encode_death_combat_event(PLAYER_ENTITY_ID, &message),
    )
    .await
}

fn bootstrap_from_profile(max_players: usize, profile: &PlayerProfile) -> play::Bootstrap {
    let game_mode = profile.game_mode;
    play::Bootstrap::new(max_players).with_player_state(
        (profile.position.x, profile.position.y, profile.position.z),
        (profile.position.yaw, profile.position.pitch),
        (game_mode.vanilla_id(), ability_flags(game_mode)),
    )
}

fn ability_flags(game_mode: GameMode) -> i8 {
    game_mode.ability_flags()
}
