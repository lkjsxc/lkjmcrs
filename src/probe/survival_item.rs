use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::play_client::PlayClient;
use crate::protocol::{codec, ids};
use crate::world::BlockPos;
use std::io::Cursor;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

const NAME: &str = "SurvivalItem";

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut first = PlayClient::connect(host, NAME).await?;
    place_initial_stone(&mut first).await?;
    break_stone_for_drop(&mut first).await?;
    reject_far_selected_item(&mut first).await?;
    place_near_stone_after_reject(&mut first).await?;
    break_grass_for_dirt(&mut first).await?;
    place_dirt(&mut first).await?;
    break_dirt_for_persistence(&mut first).await?;
    first.stream.shutdown().await?;
    drop(first);
    sleep(Duration::from_millis(500)).await;

    let mut second = PlayClient::connect_with_block(host, NAME, Some(0)).await?;
    place_persisted_dirt(&mut second).await?;
    expect_empty_selected_slot(&mut second, 4, 38).await
}

async fn place_initial_stone(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::send_use_item_on_at(&mut client.stream, 30, BlockPos::new(0, 79, 0)).await?;
    block_mutation::expect_ack_and_update(&mut client.stream, 30, 1).await
}

async fn break_stone_for_drop(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    break_at(
        &mut client.stream,
        31,
        BlockPos::new(0, 80, 0),
        0,
        "stone break",
    )
    .await
}

async fn reject_far_selected_item(
    client: &mut PlayClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(20, 79, 0);
    block_mutation::send_use_item_on_at(&mut client.stream, 32, base).await?;
    expect_update_at(
        &mut client.stream,
        32,
        BlockPos::new(20, 80, 0),
        0,
        "far reach",
    )
    .await
}

async fn place_near_stone_after_reject(
    client: &mut PlayClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(2, 79, 0);
    block_mutation::send_use_item_on_at(&mut client.stream, 33, base).await?;
    expect_update_at(
        &mut client.stream,
        33,
        BlockPos::new(2, 80, 0),
        1,
        "near stone",
    )
    .await
}

async fn break_grass_for_dirt(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    break_at(
        &mut client.stream,
        34,
        BlockPos::new(1, 79, 0),
        0,
        "grass break",
    )
    .await
}

async fn place_dirt(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(1, 79, 0);
    block_mutation::send_use_item_on_at(&mut client.stream, 35, base).await?;
    expect_update_at(
        &mut client.stream,
        35,
        BlockPos::new(1, 80, 0),
        10,
        "dirt place",
    )
    .await
}

async fn break_dirt_for_persistence(
    client: &mut PlayClient,
) -> Result<(), Box<dyn std::error::Error>> {
    break_at(
        &mut client.stream,
        36,
        BlockPos::new(1, 80, 0),
        0,
        "dirt break",
    )
    .await
}

async fn expect_empty_selected_slot(
    client: &mut PlayClient,
    x: i32,
    sequence: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(x, 79, 0);
    block_mutation::send_use_item_on_at(&mut client.stream, sequence, base).await?;
    expect_update_at(
        &mut client.stream,
        sequence,
        BlockPos::new(x, 80, 0),
        0,
        "empty slot",
    )
    .await
}

async fn place_persisted_dirt(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(3, 79, 0);
    block_mutation::send_use_item_on_at(&mut client.stream, 37, base).await?;
    expect_update_at(
        &mut client.stream,
        37,
        BlockPos::new(3, 80, 0),
        10,
        "persisted dirt",
    )
    .await
}

async fn break_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
    state: i32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::send_start_destroy_at(stream, sequence, pos).await?;
    expect_update_at(stream, sequence, pos, state, phase).await
}

async fn expect_update_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
    state: i32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let ack = block_mutation::read_next_non_time(stream, "survival material ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("survival material ack id")));
    }
    block_mutation::validate_ack(ack.data, sequence)?;
    let update = block_mutation::read_next_non_time(stream, "survival material update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("survival material update id")));
    }
    validate_update_state(update.data, pos, state, phase)
}

fn validate_update_state(
    data: Vec<u8>,
    pos: BlockPos,
    state: i32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    if codec::read_position(&mut cursor)? != (pos.x, pos.y, pos.z) {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    if codec::read_var_i32(&mut cursor)? != state {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    Ok(())
}
