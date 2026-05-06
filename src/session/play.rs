use crate::player::{PlayerProfile, PlayerStore};
use crate::protocol::ids;
use crate::protocol::play;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_actions::send_block_update;
use crate::session::bootstrap::send_play_bootstrap;
use crate::session::chat::{send_kick, send_system_chat};
use crate::session::chunk_stream::ChunkStream;
use crate::session::error::ConnectionError;
use crate::session::game_mode::apply_game_mode;
use crate::session::io::{read_packet, write_packet};
use crate::session::outbound::PlayOutbound;
use crate::session::play_packets::{PlayPacketContext, handle_play_packet};
use crate::session::play_state::PlaySession;
use crate::session::registry::{SessionId, SessionRegistry};
use tokio::net::TcpStream;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);
const TIME_INTERVAL: Duration = Duration::from_secs(1);
const TIME_STEP_TICKS: i64 = 20;

struct RegisteredSession {
    id: SessionId,
    outbound: tokio::sync::mpsc::Receiver<PlayOutbound>,
    is_op: bool,
}

pub async fn handle_play(
    stream: &mut TcpStream,
    max_players: usize,
    view_distance: i32,
    simulation_distance: i32,
    region: RegionHandle,
    sessions: SessionRegistry,
    mut profile: PlayerProfile,
    player_store: PlayerStore,
    is_op: bool,
) -> Result<(), ConnectionError> {
    let (session_id, outbound) = sessions.register(&profile).await;
    let registered = RegisteredSession {
        id: session_id,
        outbound,
        is_op,
    };
    let result = run_play(
        stream,
        max_players,
        view_distance,
        simulation_distance,
        region,
        sessions.clone(),
        player_store.clone(),
        registered,
        &mut profile,
    )
    .await;
    sessions.unregister(session_id).await;
    let save = player_store
        .save(profile)
        .await
        .map_err(|source| ConnectionError::Player {
            phase: SessionState::Play,
            source,
        });
    save?;
    result
}

async fn run_play(
    stream: &mut TcpStream,
    max_players: usize,
    view_distance: i32,
    simulation_distance: i32,
    region: RegionHandle,
    sessions: SessionRegistry,
    player_store: PlayerStore,
    registered: RegisteredSession,
    profile: &mut PlayerProfile,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Play;
    let bootstrap =
        bootstrap_from_profile(max_players, view_distance, simulation_distance, profile);
    let mut session = PlaySession::new(bootstrap, registered.id, registered.is_op);
    let mut outbound = registered.outbound;
    let mut chunk_stream = ChunkStream::new(
        crate::world::ChunkPos::new(bootstrap.chunk_x, bootstrap.chunk_z),
        bootstrap.view_distance,
    );
    let (mut reader, mut writer) = stream.split();
    let chunks = send_play_bootstrap(&mut writer, bootstrap, &region).await?;
    sessions.subscribe(session.id, chunks).await;
    session.record_keepalive_sent(1);
    let mut keepalives = time::interval_at(Instant::now() + KEEPALIVE_INTERVAL, KEEPALIVE_INTERVAL);
    keepalives.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut times = time::interval_at(Instant::now() + TIME_INTERVAL, TIME_INTERVAL);
    times.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut next_keepalive_id = 2_i64;

    let result = loop {
        let step = tokio::select! {
            packet = read_packet(&mut reader, phase) => {
                match packet {
                    Ok(packet) => handle_play_packet(
                        packet,
                        phase,
                        &mut session,
                        &mut chunk_stream,
                        profile,
                        PlayPacketContext {
                            region: &region,
                            sessions: &sessions,
                            max_players,
                            player_store: &player_store,
                            writer: &mut writer,
                        },
                    ).await,
                    Err(error) => Err(error),
                }
            }
            message = outbound.recv() => {
                match message {
                    Some(PlayOutbound::BlockUpdate { pos, state }) => {
                        send_block_update(&mut writer, phase, pos, state).await
                    }
                    Some(PlayOutbound::SystemChat { message }) => {
                        send_system_chat(&mut writer, phase, &message).await
                    }
                    Some(PlayOutbound::ApplyGameMode { game_mode }) => {
                        apply_game_mode(
                            &mut writer,
                            phase,
                            max_players,
                            profile,
                            game_mode,
                        ).await
                    }
                    Some(PlayOutbound::Kick { reason }) => {
                        send_kick(&mut writer, phase, &reason).await?;
                        break Ok(());
                    }
                    None => break Ok(()),
                }
            }
            _ = keepalives.tick() => {
                session.record_keepalive_sent(next_keepalive_id);
                let written = write_packet(
                    &mut writer,
                    phase,
                    ids::play::KEEPALIVE,
                    &play::encode_keepalive(next_keepalive_id),
                )
                .await;
                next_keepalive_id += 1;
                written
            }
            _ = times.tick() => {
                session.advance_time(TIME_STEP_TICKS);
                write_packet(
                    &mut writer,
                    phase,
                    ids::play::SET_TIME,
                    &play::encode_time(session.age, session.day_time),
                )
                .await
            }
        };
        if let Err(error) = step {
            break Err(error);
        }
    };
    session.write_profile(profile);
    result
}

fn bootstrap_from_profile(
    max_players: usize,
    view_distance: i32,
    simulation_distance: i32,
    profile: &PlayerProfile,
) -> play::Bootstrap {
    play::Bootstrap::new(max_players)
        .with_distances(view_distance, simulation_distance)
        .with_player_state(
            (profile.position.x, profile.position.y, profile.position.z),
            (profile.position.yaw, profile.position.pitch),
            (
                profile.game_mode.vanilla_id(),
                profile.game_mode.ability_flags(),
            ),
        )
}
