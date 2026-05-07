use crate::player::{Inventory, Vitals};
use crate::protocol::commands;
use crate::protocol::ids;
use crate::protocol::play;
use crate::protocol::vitals::{self, HealthUpdate};
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::chunk_stream_send::{EncodedChunk, send_encoded_chunk_batch};
use crate::session::error::ConnectionError;
use crate::session::inventory_sync;
use crate::session::io::write_packet;
use crate::world::ChunkPos;
use tokio::io::AsyncWrite;

pub async fn send_play_bootstrap<W>(
    stream: &mut W,
    bootstrap: play::Bootstrap,
    inventory: &Inventory,
    player_vitals: &Vitals,
    region: &RegionHandle,
    initial_chunks: &[ChunkPos],
    cache: &mut ChunkPayloadCache,
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
        ids::play::UPDATE_HEALTH,
        &vitals::encode_update_health(HealthUpdate {
            health: player_vitals.health,
            hunger: i32::from(player_vitals.hunger),
            saturation: player_vitals.saturation,
        }),
    )
    .await?;
    write_packet(
        stream,
        phase,
        ids::play::DECLARE_COMMANDS,
        &commands::encode_declare_commands(),
    )
    .await?;
    inventory_sync::send_bootstrap_inventory(stream, phase, inventory).await?;
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
        .load_chunks(initial_chunks.to_vec())
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    send_chunk_batch(stream, &chunks, cache).await?;
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

pub async fn send_chunk_batch<W>(
    stream: &mut W,
    chunks: &[crate::world::ChunkSnapshot],
    cache: &mut ChunkPayloadCache,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let chunks = chunks
        .iter()
        .map(|chunk| EncodedChunk::from_snapshot(chunk, cache))
        .collect::<Vec<_>>();
    send_encoded_chunk_batch(stream, &chunks).await
}

#[cfg(test)]
mod tests {
    use crate::protocol::{chunk, codec};
    use crate::session::chunk_wire::WireChunk;
    use crate::world::{ChunkPos, ChunkSnapshot};
    use std::io::Cursor;

    #[test]
    fn world_flat_chunk_wire_shape_matches_probe_contract() {
        let snapshot = ChunkSnapshot::flat(ChunkPos::new(0, 0));
        let packet = chunk::encode_level_chunk_with_light(&WireChunk(&snapshot));
        let mut cursor = Cursor::new(packet);
        cursor.set_position(8);
        skip_heightmaps(&mut cursor);
        assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 6294);
    }

    fn skip_heightmaps(cursor: &mut Cursor<Vec<u8>>) {
        let count = codec::read_var_i32(cursor).unwrap();
        for _ in 0..count {
            let _kind = codec::read_var_i32(cursor).unwrap();
            let longs = codec::read_var_i32(cursor).unwrap();
            cursor.set_position(cursor.position() + longs as u64 * 8);
        }
    }
}
