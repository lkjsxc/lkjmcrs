use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::play_client::PlayClient;
use crate::probe::survival_expect;
use crate::protocol::ids;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut actor = PlayClient::connect(host, "ProbeA").await?;
    block_mutation::acquire_dirt(
        &mut actor.stream,
        crate::probe::position::BlockPos::new(0, 79, 0),
        "actor dirt",
    )
    .await?;
    let mut observer = PlayClient::connect_with_block(host, "ProbeB", Some(0)).await?;

    block_mutation::send_use_item_on_at(
        &mut actor.stream,
        20,
        crate::probe::position::BlockPos::new(0, 79, 0),
    )
    .await?;
    block_mutation::expect_ack_and_update(&mut actor.stream, 20, 10).await?;
    expect_observer_update(&mut observer, 10).await?;

    block_mutation::mine_dirt_like_at(
        &mut actor.stream,
        21,
        crate::probe::position::BlockPos::new(0, 80, 0),
        10,
        0,
    )
    .await?;
    expect_observer_update(&mut observer, 0).await
}

pub(super) async fn expect_observer_update(
    observer: &mut PlayClient,
    state: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet = survival_expect::read_next_survival_packet(&mut observer.stream).await?;
    if packet.id == ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("observer unexpected ack")));
    }
    if packet.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(std::io::Error::other(format!(
            "observer update id: got {}",
            packet.id
        ))));
    }
    block_mutation::validate_update(packet.data, state)
}
