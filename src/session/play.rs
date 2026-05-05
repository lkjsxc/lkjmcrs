use crate::protocol::chunk;
use crate::protocol::ids;
use crate::protocol::play;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::{read_packet, write_packet};
use crate::world::FlatWorld;
use tokio::net::TcpStream;

pub async fn handle_play(
    stream: &mut TcpStream,
    max_players: usize,
) -> Result<(), ConnectionError> {
    let phase = SessionState::Play;
    let bootstrap = play::Bootstrap::new(max_players);
    send_play_bootstrap(stream, bootstrap).await?;
    loop {
        let packet = read_packet(stream, phase).await?;
        match packet.id {
            ids::play::SERVERBOUND_TELEPORT_CONFIRM
            | ids::play::SERVERBOUND_CHUNK_BATCH_RECEIVED
            | ids::play::SERVERBOUND_SETTINGS
            | ids::play::SERVERBOUND_KEEPALIVE
            | ids::play::SERVERBOUND_POSITION
            | ids::play::SERVERBOUND_POSITION_LOOK
            | ids::play::SERVERBOUND_LOOK
            | ids::play::SERVERBOUND_FLYING
            | ids::play::SERVERBOUND_PLAYER_LOADED
            | ids::play::SERVERBOUND_PONG => {
                tracing::debug!(phase = %phase, packet_id = packet.id, "play packet accepted");
            }
            _ => {
                tracing::debug!(phase = %phase, packet_id = packet.id, "play packet ignored");
            }
        }
    }
}

async fn send_play_bootstrap(
    stream: &mut TcpStream,
    bootstrap: play::Bootstrap,
) -> Result<(), ConnectionError> {
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
    let chunks = FlatWorld::default().spawn_chunks(1);
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
