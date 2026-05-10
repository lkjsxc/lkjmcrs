use crate::probe::block_mutation;
use crate::probe::item_entities;
use crate::probe::live_play;
use crate::probe::play_client::PlayClient;
use crate::probe::position::BlockPos;
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};

const NAME: &str = "SectionPersistA";
const DIRT: i32 = 10;
const AIR: i32 = 0;
const DIRT_ITEM: i32 = 28;
const MINED: BlockPos = BlockPos::new(1, 79, 0);
const LOW: BlockPos = BlockPos::new(3, 80, 0);
const HIGH: BlockPos = BlockPos::new(4, 96, 0);

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlayClient::connect(host, NAME).await?;
    acquire_dirt(&mut client).await?;
    place_low(&mut client).await?;
    reset_low(&mut client).await?;
    place_high(&mut client).await?;
    client.stream.shutdown().await?;
    drop(client);
    sleep(Duration::from_millis(2_000)).await;

    PlayClient::connect_with_blocks(host, NAME, vec![(MINED, AIR), (LOW, AIR), (HIGH, DIRT)])
        .await?;
    Ok(())
}

async fn acquire_dirt(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::acquire_dirt_from(&mut client.stream, MINED, 9, "section dirt").await
}

async fn place_low(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::send_use_item_on_at(&mut client.stream, 130, BlockPos::new(3, 79, 0)).await?;
    block_mutation::expect_ack_and_update_at(&mut client.stream, 130, LOW, DIRT).await
}

async fn place_high(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    live_play::send_position_look_at(&mut client.stream, 4.5, 97.0, 0.5, 0.0, 0.0).await?;
    block_mutation::send_use_item_on_at(&mut client.stream, 131, BlockPos::new(4, 95, 0)).await?;
    block_mutation::expect_ack_and_update_at(&mut client.stream, 131, HIGH, DIRT).await
}

async fn reset_low(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    live_play::send_position_look_at(&mut client.stream, 3.5, 81.0, 0.5, 0.0, 0.0).await?;
    block_mutation::mine_dirt_like_at(&mut client.stream, 132, LOW, DIRT, AIR).await?;
    item_entities::collect_drop_at(
        &mut client.stream,
        DIRT_ITEM,
        "section reset dirt",
        f64::from(LOW.x) + 0.5,
        f64::from(LOW.y) + 1.0,
        f64::from(LOW.z) + 0.5,
    )
    .await?;
    Ok(())
}
