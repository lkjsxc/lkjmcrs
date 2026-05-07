use crate::player::{PlayerProfile, PlayerStore};
use crate::protocol::ids;
use crate::protocol::play;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::bootstrap::send_play_bootstrap;
use crate::session::chunk_stream::ChunkStream;
use crate::session::error::ConnectionError;
use crate::session::io::{read_packet, write_packet};
use crate::session::item_visibility;
use crate::session::outbound::PlayOutbound;
use crate::session::play_outbound::{OutboundStep, handle_outbound};
use crate::session::play_packets::{PlayPacketContext, handle_play_packet};
use crate::session::play_state::PlaySession;
use crate::session::registry::{SessionId, SessionRegistry};
use tokio::net::TcpStream;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);
const TIME_INTERVAL: Duration = Duration::from_secs(1);
const TIME_STEP_TICKS: i64 = 20;

#[derive(Debug, Clone, Copy)]
pub struct PlaySettings {
    pub max_players: usize,
    pub view_distance: i32,
    pub simulation_distance: i32,
}

struct RegisteredSession {
    id: SessionId,
    outbound: tokio::sync::mpsc::Receiver<PlayOutbound>,
    is_op: bool,
}

pub async fn handle_play(
    stream: &mut TcpStream,
    settings: PlaySettings,
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
        settings,
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
    settings: PlaySettings,
    region: RegionHandle,
    sessions: SessionRegistry,
    player_store: PlayerStore,
    registered: RegisteredSession,
    profile: &mut PlayerProfile,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Play;
    let bootstrap = bootstrap_from_profile(settings, profile);
    let mut session = PlaySession::new(bootstrap, registered.id, registered.is_op);
    let mut outbound = registered.outbound;
    let mut chunk_stream = ChunkStream::new(
        crate::world::ChunkPos::new(bootstrap.chunk_x, bootstrap.chunk_z),
        bootstrap.view_distance,
    );
    let (mut reader, mut writer) = stream.split();
    let chunks = send_play_bootstrap(
        &mut writer,
        bootstrap,
        &profile.inventory,
        &profile.vitals,
        &region,
    )
    .await?;
    item_visibility::send_items_in_chunks(&mut writer, phase, &region, chunks.clone()).await?;
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
                            max_players: settings.max_players,
                            player_store: &player_store,
                            writer: &mut writer,
                        },
                    ).await,
                    Err(error) => Err(error),
                }
            }
            message = outbound.recv() => {
                match handle_outbound(
                    &mut writer,
                    phase,
                    settings.max_players,
                    profile,
                    &mut session,
                    message,
                ).await {
                    Ok(OutboundStep::Continue) => Ok(()),
                    Ok(OutboundStep::Close) => break Ok(()),
                    Err(error) => Err(error),
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

fn bootstrap_from_profile(settings: PlaySettings, profile: &PlayerProfile) -> play::Bootstrap {
    play::Bootstrap::new(settings.max_players)
        .with_distances(settings.view_distance, settings.simulation_distance)
        .with_player_state(
            (profile.position.x, profile.position.y, profile.position.z),
            (profile.position.yaw, profile.position.pitch),
            (
                profile.game_mode.vanilla_id(),
                profile.game_mode.ability_flags(),
            ),
        )
}
