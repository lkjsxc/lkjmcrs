use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::chunk;
use crate::probe::live_play;
use crate::probe::play_client::PlayClient;
use crate::probe::position::BlockPos;
use crate::probe::survival_expect;
use crate::probe::terrain_chunk::DecodedChunk;
use crate::probe::validation::validate_chunk_batch_finished;
use crate::protocol::{codec, ids};
use std::collections::HashSet;
use std::io::Cursor;
use tokio::net::TcpStream;

const STREAM_PLACE_SEQUENCE: i32 = 20;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlayClient::connect(host, "ChunkStream").await?;
    block_mutation::acquire_dirt(&mut client.stream, BlockPos::new(2, 79, 0), "stream dirt")
        .await?;
    live_play::send_position_look_at(&mut client.stream, 16.5, 80.0, 0.5, 0.0, 0.0).await?;
    expect_cache_center(&mut client.stream, 1, 0).await?;
    expect_unload_column(&mut client.stream, -2).await?;
    let streamed_surface = expect_column_batch(&mut client.stream, 3).await?;
    live_play::send_position_look_at(&mut client.stream, 44.5, 80.0, 0.5, 0.0, 0.0).await?;
    expect_cache_center(&mut client.stream, 2, 0).await?;
    expect_unload_column(&mut client.stream, -1).await?;
    expect_column_batch(&mut client.stream, 4).await?;
    place_in_streamed_chunk(&mut client.stream, streamed_surface).await
}

async fn expect_cache_center(
    stream: &mut TcpStream,
    x: i32,
    z: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet = read_next_stream_packet(stream, "stream cache center").await?;
    if packet.id != ids::play::CHUNK_CACHE_CENTER {
        return Err(Box::new(ProbeError::Phase("stream cache center id")));
    }
    let mut cursor = Cursor::new(packet.data);
    if codec::read_var_i32(&mut cursor)? != x || codec::read_var_i32(&mut cursor)? != z {
        return Err(Box::new(ProbeError::Phase("stream cache center payload")));
    }
    Ok(())
}

async fn expect_unload_column(
    stream: &mut TcpStream,
    expected_x: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut positions = HashSet::new();
    for _ in 0..5 {
        let packet = read_next_stream_packet(stream, "stream unload").await?;
        if packet.id != ids::play::UNLOAD_CHUNK {
            return Err(Box::new(ProbeError::Phase("stream unload id")));
        }
        let mut cursor = Cursor::new(packet.data);
        let z = codec::read_i32(&mut cursor)?;
        let x = codec::read_i32(&mut cursor)?;
        positions.insert((x, z));
    }
    validate_new_column(positions, expected_x)
}

async fn expect_column_batch(
    stream: &mut TcpStream,
    expected_x: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let start = read_next_stream_packet(stream, "stream batch start").await?;
    if start.id != ids::play::CHUNK_BATCH_START {
        return Err(Box::new(ProbeError::Phase("stream batch start id")));
    }
    let mut positions = HashSet::new();
    let mut surface = None;
    for _ in 0..5 {
        let chunk_packet = read_next_stream_packet(stream, "stream chunk").await?;
        if chunk_packet.id != ids::play::LEVEL_CHUNK_WITH_LIGHT {
            return Err(Box::new(ProbeError::Phase("stream chunk id")));
        }
        let pos = chunk::level_chunk_pos(&chunk_packet.data)?;
        positions.insert(pos);
        chunk::validate_level_chunk_with_light(chunk_packet.data.clone())?;
        if pos == (expected_x, 0) {
            surface = DecodedChunk::from_packet(chunk_packet.data)?.surface_y(0, 0);
        }
    }
    validate_new_column(positions, expected_x)?;
    let finished = read_next_stream_packet(stream, "stream batch finished").await?;
    if finished.id != ids::play::CHUNK_BATCH_FINISHED {
        return Err(Box::new(ProbeError::Phase("stream batch finished id")));
    }
    validate_chunk_batch_finished(finished.data, 5)?;
    Ok(surface.unwrap_or(79))
}

async fn read_next_stream_packet(
    stream: &mut TcpStream,
    phase: &'static str,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    loop {
        let packet = survival_expect::read_next_live_packet(stream).await?;
        if !matches!(
            packet.id,
            ids::play::SET_PLAYER_INVENTORY | ids::play::HELD_ITEM_SLOT
        ) {
            return Ok(packet);
        }
        tracing::debug!(phase, "inventory packet skipped during stream probe");
    }
}

fn validate_new_column(
    positions: HashSet<(i32, i32)>,
    expected_x: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected: HashSet<_> = (-2..=2).map(|z| (expected_x, z)).collect();
    if positions != expected {
        return Err(Box::new(ProbeError::Phase("stream chunk positions")));
    }
    Ok(())
}

async fn place_in_streamed_chunk(
    stream: &mut TcpStream,
    surface_y: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(48, surface_y, 0);
    let placed = BlockPos::new(48, surface_y + 1, 0);
    block_mutation::send_use_item_on_at(stream, STREAM_PLACE_SEQUENCE, base).await?;
    let ack = survival_expect::read_next_material_packet(stream, "stream placement ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("stream placement ack id")));
    }
    block_mutation::validate_ack(ack.data, STREAM_PLACE_SEQUENCE)?;
    let update =
        survival_expect::read_next_material_packet(stream, "stream placement update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("stream placement update id")));
    }
    block_mutation::validate_update_at(update.data, placed, 10)
}
