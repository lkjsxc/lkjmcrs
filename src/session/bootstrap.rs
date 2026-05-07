use crate::player::{Inventory, Vitals};
use crate::protocol::chunk;
use crate::protocol::commands;
use crate::protocol::ids;
use crate::protocol::play;
use crate::protocol::vitals::{self, HealthUpdate};
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::inventory_sync;
use crate::session::io::write_packet;
use crate::world::{BlockState, ChunkPos, ChunkSnapshot};
use tokio::io::AsyncWrite;

pub async fn send_play_bootstrap<W>(
    stream: &mut W,
    bootstrap: play::Bootstrap,
    inventory: &Inventory,
    player_vitals: &Vitals,
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
        .spawn_chunks_around(
            ChunkPos::new(bootstrap.chunk_x, bootstrap.chunk_z),
            bootstrap.view_distance,
        )
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?;
    debug_assert_eq!(chunks.len(), bootstrap.chunk_count());
    send_chunk_batch(stream, &chunks).await?;
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
    chunks: &[ChunkSnapshot],
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
            &chunk::encode_level_chunk_with_light(&WireChunk(chunk_snapshot)),
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

struct WireChunk<'a>(&'a ChunkSnapshot);

impl chunk::ChunkColumn for WireChunk<'_> {
    fn position(&self) -> chunk::ChunkPosition {
        chunk::ChunkPosition {
            x: self.0.pos.x,
            z: self.0.pos.z,
        }
    }

    fn block_state_id_at_local(&self, x: usize, y: i32, z: usize) -> i32 {
        block_state_id(self.0.block_at_local(x, y, z))
    }
}

pub(crate) fn block_state_id(state: BlockState) -> i32 {
    match state {
        BlockState::Air => chunk::AIR_ID,
        BlockState::Bedrock => chunk::BEDROCK_ID,
        BlockState::Stone => chunk::STONE_ID,
        BlockState::Dirt => chunk::DIRT_ID,
        BlockState::GrassBlock => chunk::GRASS_BLOCK_ID,
    }
}

#[cfg(test)]
mod tests {
    use super::WireChunk;
    use crate::protocol::{chunk, codec};
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
