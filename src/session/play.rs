use crate::player::{PlayerProfile, PlayerStore};
use crate::protocol::ids;
use crate::protocol::play;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::bootstrap::send_play_bootstrap;
use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::chunk_stream::ChunkStream;
use crate::session::error::ConnectionError;
use crate::session::io::{read_packet, write_packet};
use crate::session::item_visibility;
use crate::session::play_intervals::{
    CHUNK_DRAIN_INTERVAL, HUNGER_INTERVAL, KEEPALIVE_INTERVAL, TIME_INTERVAL, TIME_STEP_TICKS,
};
pub use crate::session::play_model::PlaySettings;
use crate::session::play_model::RegisteredSession;
use crate::session::play_outbound::{OutboundStep, handle_outbound};
use crate::session::play_packet_context::PlayPacketContext;
use crate::session::play_packets::handle_play_packet;
use crate::session::play_state::PlaySession;
use crate::session::play_timers::delayed_interval;
use crate::session::registry::SessionRegistry;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::Instant;
pub async fn handle_play<S>(
    stream: &mut S,
    settings: PlaySettings,
    region: RegionHandle,
    sessions: SessionRegistry,
    mut profile: PlayerProfile,
    player_store: PlayerStore,
    is_op: bool,
) -> Result<(), ConnectionError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
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

async fn run_play<S>(
    stream: &mut S,
    settings: PlaySettings,
    region: RegionHandle,
    sessions: SessionRegistry,
    player_store: PlayerStore,
    registered: RegisteredSession,
    profile: &mut PlayerProfile,
) -> Result<(), ConnectionError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let phase = SessionState::Play;
    let bootstrap = crate::session::play_bootstrap_state::from_profile(settings, profile);
    let mut session = PlaySession::new(bootstrap, registered.id, registered.is_op);
    let mut outbound = registered.outbound;
    let mut chunk_stream = ChunkStream::new(
        crate::world::ChunkPos::new(bootstrap.chunk_x, bootstrap.chunk_z),
        bootstrap.view_distance,
    );
    let mut chunk_cache = ChunkPayloadCache::default();
    let (mut reader, mut writer) = tokio::io::split(stream);
    let initial_chunks = chunk_stream.initial_chunks();
    let chunks = send_play_bootstrap(
        &mut writer,
        bootstrap,
        &profile.inventory,
        &profile.vitals,
        &region,
        &initial_chunks,
        &mut chunk_cache,
    )
    .await?;
    item_visibility::send_items_in_chunks(&mut writer, phase, &region, chunks.clone()).await?;
    sessions.subscribe(session.id, chunks).await;
    session.record_keepalive_sent(1, Instant::now());
    let mut keepalives = delayed_interval(KEEPALIVE_INTERVAL);
    let mut times = delayed_interval(TIME_INTERVAL);
    let mut hunger_ticks = delayed_interval(HUNGER_INTERVAL);
    let mut chunk_drains = delayed_interval(CHUNK_DRAIN_INTERVAL);
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
                            chunk_cache: &mut chunk_cache,
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
                let now = Instant::now();
                session.record_keepalive_sent(next_keepalive_id, now);
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
                if session.keepalive_timed_out(Instant::now()) {
                    Err(ConnectionError::Protocol {
                        phase,
                        message: "keepalive timeout",
                    })
                } else {
                    session.advance_time(TIME_STEP_TICKS);
                    write_packet(
                        &mut writer,
                        phase,
                        ids::play::SET_TIME,
                        &play::encode_time(session.age, session.day_time),
                    )
                    .await
                }
            }
            _ = hunger_ticks.tick() => {
                crate::session::play_ticks::handle_hunger(
                    &mut writer,
                    phase,
                    profile,
                    &mut session,
                ).await
            }
            _ = chunk_drains.tick() => crate::session::play_chunk_drain::flush_pending(
                &mut writer,
                phase,
                &region,
                &sessions,
                session.id,
                &mut chunk_stream,
                &mut chunk_cache,
            ).await
        };
        if let Err(error) = step {
            break Err(error);
        }
    };
    session.write_profile(profile);
    result
}
