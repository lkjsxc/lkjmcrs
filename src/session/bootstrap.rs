use crate::protocol::chunk;
use crate::protocol::ids;
use crate::protocol::play;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::world::ChunkPos;
use tokio::io::AsyncWrite;

pub async fn send_play_bootstrap<W>(
    stream: &mut W,
    bootstrap: play::Bootstrap,
    region: &RegionHandle,
) -> Result<Vec<ChunkPos>, ConnectionError>
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
        &play::encode_player_abilities_for(bootstrap),
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
        &play::encode_chunk_cache_center(bootstrap.chunk_x, bootstrap.chunk_z),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::CHUNK_CACHE_RADIUS,
        &play::encode_chunk_cache_radius(bootstrap.view_distance),
    )
    .await?;
    let chunks = region
        .spawn_chunks_around(
            ChunkPos::new(bootstrap.chunk_x, bootstrap.chunk_z),
            bootstrap.view_distance,
        )
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    debug_assert_eq!(chunks.len(), bootstrap.chunk_count());
    send_chunks(stream, &chunks).await?;
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
    .await?;
    Ok(chunks.iter().map(|chunk| chunk.pos).collect())
}

async fn send_chunks<W>(
    stream: &mut W,
    chunks: &[crate::world::ChunkSnapshot],
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let phase = SessionState::Play;
    write_packet(stream, phase, ids::play::CHUNK_BATCH_START, &[]).await?;
    for chunk_snapshot in chunks {
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
    .await
}
