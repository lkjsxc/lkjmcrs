use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::item_entities;
use crate::probe::play_client::PlayClient;
use crate::probe::survival_expect;
use crate::protocol::ids;
use crate::world::BlockPos;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

const NAME: &str = "SurvivalItem";

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut first = PlayClient::connect(host, NAME).await?;
    expect_empty_selected_slot(&mut first, 0, 30).await?;
    break_grass_for_dirt(&mut first).await?;
    reject_far_selected_item(&mut first).await?;
    place_dirt(&mut first).await?;
    break_dirt_for_persistence(&mut first).await?;
    first.stream.shutdown().await?;
    drop(first);
    sleep(Duration::from_millis(500)).await;

    let mut second = PlayClient::connect_with_block(host, NAME, Some(0)).await?;
    place_persisted_dirt(&mut second).await?;
    expect_empty_selected_slot(&mut second, 4, 38).await
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

async fn break_grass_for_dirt(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    break_at(
        &mut client.stream,
        34,
        BlockPos::new(1, 79, 0),
        0,
        "grass break",
    )
    .await?;
    item_entities::collect_drop(&mut client.stream, 28, "grass pickup").await?;
    Ok(())
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
    .await?;
    item_entities::collect_drop(&mut client.stream, 28, "dirt pickup").await?;
    Ok(())
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
    let ack = survival_expect::read_next_material_packet(stream, "survival material ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("survival material ack id")));
    }
    block_mutation::validate_ack(ack.data, sequence)?;
    let update =
        survival_expect::read_next_material_packet(stream, "survival material update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(format_packet_error(
            "survival material update id",
            update.id,
        ));
    }
    survival_expect::validate_update_state(update.data, pos, state, phase)
}

fn format_packet_error(phase: &'static str, packet_id: i32) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::other(format!(
        "{phase}: got packet 0x{packet_id:x}"
    )))
}
