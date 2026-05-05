use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::play_client::PlayClient;
use crate::protocol::ids;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut actor = PlayClient::connect(host, "ProbeA").await?;
    let mut observer = PlayClient::connect(host, "ProbeB").await?;

    block_mutation::send_use_item_on(&mut actor.stream, 20).await?;
    block_mutation::expect_ack_and_update(&mut actor.stream, 20, 1).await?;
    expect_observer_update(&mut observer, 1).await?;

    block_mutation::send_start_destroy(&mut actor.stream, 21).await?;
    block_mutation::expect_ack_and_update(&mut actor.stream, 21, 0).await?;
    expect_observer_update(&mut observer, 0).await
}

async fn expect_observer_update(
    observer: &mut PlayClient,
    state: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        block_mutation::read_next_non_time(&mut observer.stream, "observer update").await?;
    if packet.id == ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("observer unexpected ack")));
    }
    if packet.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("observer update id")));
    }
    block_mutation::validate_update(packet.data, state)
}
