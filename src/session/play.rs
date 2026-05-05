use crate::protocol::chunk;
use crate::protocol::ids;
use crate::protocol::play;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::{read_packet, write_packet};
use crate::session::play_packets::handle_play_packet;
use crate::session::play_state::PlaySession;
use crate::world::FlatWorld;
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);
const TIME_INTERVAL: Duration = Duration::from_secs(1);
const TIME_STEP_TICKS: i64 = 20;

pub async fn handle_play(
    stream: &mut TcpStream,
    max_players: usize,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Play;
    let bootstrap = play::Bootstrap::new(max_players);
    let mut session = PlaySession::new(bootstrap);
    let (mut reader, mut writer) = stream.split();
    send_play_bootstrap(&mut writer, bootstrap).await?;
    session.record_keepalive_sent(1);
    let mut keepalives = time::interval_at(Instant::now() + KEEPALIVE_INTERVAL, KEEPALIVE_INTERVAL);
    keepalives.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut times = time::interval_at(Instant::now() + TIME_INTERVAL, TIME_INTERVAL);
    times.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut next_keepalive_id = 2_i64;

    loop {
        tokio::select! {
            packet = read_packet(&mut reader, phase) => {
                handle_play_packet(packet?, phase, &mut session)?;
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

async fn send_play_bootstrap<W>(
    stream: &mut W,
    bootstrap: play::Bootstrap,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let phase = SessionState::Play;
    write_packet(
        stream,
        phase,
        ids::play::LOGIN,
        &play::encode_login(bootstrap),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::DEFAULT_SPAWN_POSITION,
        &play::encode_default_spawn_position(bootstrap),
    )
    .await?;
    write_packet(stream, phase, ids::play::SET_TIME, &play::encode_time(0, 0)).await?;
    write_packet(
        stream,
        phase,
        ids::play::PLAYER_ABILITIES,
        &play::encode_player_abilities(),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::GAME_STATE_CHANGE,
        &play::encode_start_waiting_for_chunks(),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::CHUNK_CACHE_CENTER,
        &play::encode_chunk_cache_center(0, 0),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::CHUNK_CACHE_RADIUS,
        &play::encode_chunk_cache_radius(bootstrap.view_distance),
    )
    .await?;
    let chunks = FlatWorld::default().spawn_chunks(bootstrap.view_distance);
    debug_assert_eq!(chunks.len(), bootstrap.chunk_count());
    write_packet(stream, phase, ids::play::CHUNK_BATCH_START, &[]).await?;
    for chunk_snapshot in &chunks {
        write_packet(
            stream,
            phase,
            ids::play::LEVEL_CHUNK_WITH_LIGHT,
            &chunk::encode_level_chunk_with_light(chunk_snapshot),
        )
        .await?;
        write_packet(
            stream,
            phase,
            ids::play::UPDATE_LIGHT,
            &chunk::encode_update_light(chunk_snapshot),
        )
        .await?;
    }
    write_packet(
        stream,
        phase,
        ids::play::CHUNK_BATCH_FINISHED,
        &chunk::encode_chunk_batch_finished(chunks.len()),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::PLAYER_POSITION,
        &play::encode_initial_position(bootstrap),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::KEEPALIVE,
        &play::encode_keepalive(1),
    )
    .await
}
