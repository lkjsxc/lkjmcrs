use crate::protocol::ids;
use crate::protocol::play;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::block_actions::send_block_update;
use crate::session::bootstrap::send_play_bootstrap;
use crate::session::error::ConnectionError;
use crate::session::io::{read_packet, write_packet};
use crate::session::outbound::PlayOutbound;
use crate::session::play_packets::handle_play_packet;
use crate::session::play_state::PlaySession;
use crate::session::registry::{SessionId, SessionRegistry};
use tokio::net::TcpStream;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);
const TIME_INTERVAL: Duration = Duration::from_secs(1);
const TIME_STEP_TICKS: i64 = 20;

pub async fn handle_play(
    stream: &mut TcpStream,
    max_players: usize,
    region: RegionHandle,
    sessions: SessionRegistry,
) -> Result<(), ConnectionError> {
    let (session_id, outbound) = sessions.register().await;
    let result = run_play(
        stream,
        max_players,
        region,
        sessions.clone(),
        session_id,
        outbound,
    )
    .await;
    sessions.unregister(session_id).await;
    result
}

async fn run_play(
    stream: &mut TcpStream,
    max_players: usize,
    region: RegionHandle,
    sessions: SessionRegistry,
    session_id: SessionId,
    mut outbound: tokio::sync::mpsc::Receiver<PlayOutbound>,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Play;
    let bootstrap = play::Bootstrap::new(max_players);
    let mut session = PlaySession::new(bootstrap, session_id);
    let (mut reader, mut writer) = stream.split();
    let chunks = send_play_bootstrap(&mut writer, bootstrap, &region).await?;
    sessions.subscribe(session.id, chunks).await;
    session.record_keepalive_sent(1);
    let mut keepalives = time::interval_at(Instant::now() + KEEPALIVE_INTERVAL, KEEPALIVE_INTERVAL);
    keepalives.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut times = time::interval_at(Instant::now() + TIME_INTERVAL, TIME_INTERVAL);
    times.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut next_keepalive_id = 2_i64;

    loop {
        tokio::select! {
            packet = read_packet(&mut reader, phase) => {
                handle_play_packet(
                    packet?,
                    phase,
                    &mut session,
                    &region,
                    &sessions,
                    &mut writer,
                ).await?;
            }
            message = outbound.recv() => {
                match message {
                    Some(PlayOutbound::BlockUpdate { pos, state }) => {
                        send_block_update(&mut writer, phase, pos, state).await?;
                    }
                    None => break Ok(()),
                }
            }
            _ = keepalives.tick() => {
                session.record_keepalive_sent(next_keepalive_id);
                write_packet(
                    &mut writer,
                    phase,
                    ids::play::KEEPALIVE,
                    &play::encode_keepalive(next_keepalive_id),
                )
                .await?;
                next_keepalive_id += 1;
            }
            _ = times.tick() => {
                session.advance_time(TIME_STEP_TICKS);
                write_packet(
                    &mut writer,
                    phase,
                    ids::play::SET_TIME,
                    &play::encode_time(session.age, session.day_time),
                )
                .await?;
            }
        }
    }
}
