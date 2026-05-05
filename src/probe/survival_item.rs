use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::play_client::PlayClient;
use crate::protocol::ids;
use crate::world::BlockPos;
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};

const NAME: &str = "SurvivalItem";

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut first = PlayClient::connect(host, NAME).await?;
    place_initial_stone(&mut first).await?;
    expect_empty_selected_slot(&mut first, 1, 31).await?;
    break_for_drop(&mut first).await?;
    first.stream.shutdown().await?;
    drop(first);
    sleep(Duration::from_millis(500)).await;

    let mut second = PlayClient::connect(host, NAME).await?;
    place_persisted_drop(&mut second).await?;
    expect_empty_selected_slot(&mut second, 2, 34).await
}

async fn place_initial_stone(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::send_use_item_on_at(&mut client.stream, 30, BlockPos::new(0, 79, 0)).await?;
    block_mutation::expect_ack_and_update(&mut client.stream, 30, 1).await
}

async fn expect_empty_selected_slot(
    client: &mut PlayClient,
    x: i32,
    sequence: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(x, 79, 0);
    block_mutation::send_use_item_on_at(&mut client.stream, sequence, base).await?;
    let ack = block_mutation::read_next_non_time(&mut client.stream, "survival empty ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("survival empty ack id")));
    }
    block_mutation::validate_ack(ack.data, sequence)?;
    let update =
        block_mutation::read_next_non_time(&mut client.stream, "survival empty update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("survival empty update id")));
    }
    block_mutation::validate_update_at(update.data, BlockPos::new(x, 80, 0), 0)
}

async fn break_for_drop(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::send_start_destroy(&mut client.stream, 32).await?;
    block_mutation::expect_ack_and_update(&mut client.stream, 32, 0).await
}

async fn place_persisted_drop(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    let base = BlockPos::new(1, 79, 0);
    block_mutation::send_use_item_on_at(&mut client.stream, 33, base).await?;
    let ack = block_mutation::read_next_non_time(&mut client.stream, "survival drop ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("survival drop ack id")));
    }
    block_mutation::validate_ack(ack.data, 33)?;
    let update =
        block_mutation::read_next_non_time(&mut client.stream, "survival drop update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("survival drop update id")));
    }
    block_mutation::validate_update_at(update.data, BlockPos::new(1, 80, 0), 1)
}
