use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::chunk;
use crate::probe::live_play;
use crate::probe::play_client::PlayClient;
use crate::probe::position::BlockPos;
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
    expect_column_batch(&mut client.stream, 3).await?;
    live_play::send_position_look_at(&mut client.stream, 44.5, 80.0, 0.5, 0.0, 0.0).await?;
    expect_cache_center(&mut client.stream, 2, 0).await?;
    expect_unload_column(&mut client.stream, -1).await?;
    expect_column_batch(&mut client.stream, 4).await?;
    place_in_streamed_chunk(&mut client.stream).await
}

async fn expect_cache_center(
    stream: &mut TcpStream,
    x: i32,
    z: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet = block_mutation::read_next_non_time(stream, "stream cache center").await?;
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
        let packet = block_mutation::read_next_non_time(stream, "stream unload").await?;
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
) -> Result<(), Box<dyn std::error::Error>> {
    let start = block_mutation::read_next_non_time(stream, "stream batch start").await?;
    if start.id != ids::play::CHUNK_BATCH_START {
        return Err(Box::new(ProbeError::Phase("stream batch start id")));
    }
    let mut positions = HashSet::new();
    for _ in 0..5 {
        let chunk_packet = block_mutation::read_next_non_time(stream, "stream chunk").await?;
        if chunk_packet.id != ids::play::LEVEL_CHUNK_WITH_LIGHT {
            return Err(Box::new(ProbeError::Phase("stream chunk id")));
        }
        positions.insert(chunk::level_chunk_pos(&chunk_packet.data)?);
        chunk::validate_level_chunk_with_light(chunk_packet.data)?;
    }
    validate_new_column(positions, expected_x)?;
    let finished = block_mutation::read_next_non_time(stream, "stream batch finished").await?;
    if finished.id != ids::play::CHUNK_BATCH_FINISHED {
        return Err(Box::new(ProbeError::Phase("stream batch finished id")));
    }
    validate_chunk_batch_finished(finished.data, 5)
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

async fn place_in_streamed_chunk(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(48, 79, 0);
    let placed = BlockPos::new(48, 80, 0);
    block_mutation::send_use_item_on_at(stream, STREAM_PLACE_SEQUENCE, base).await?;
    let ack = block_mutation::read_next_non_time(stream, "stream placement ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("stream placement ack id")));
    }
    block_mutation::validate_ack(ack.data, STREAM_PLACE_SEQUENCE)?;
    let update = block_mutation::read_next_non_time(stream, "stream placement update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("stream placement update id")));
    }
    block_mutation::validate_update_at(update.data, placed, 10)
}
