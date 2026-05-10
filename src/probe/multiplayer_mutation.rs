use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::live_play;
use crate::probe::play_client::PlayClient;
use crate::probe::position::BlockPos;
use crate::probe::survival_expect;
use crate::protocol::ids;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut actor = PlayClient::connect(host, "ProbeA").await?;
    let ground = spawn_ground(&actor);
    let placed = BlockPos::new(ground.x, ground.y + 1, ground.z);
    move_to_spawn(&mut actor).await?;
    block_mutation::acquire_dirt_from(&mut actor.stream, ground, 9, "actor dirt").await?;
    let mut observer = PlayClient::connect_with_block(host, "ProbeB", None).await?;

    block_mutation::send_use_item_on_at(&mut actor.stream, 20, ground).await?;
    block_mutation::expect_ack_and_update_at(&mut actor.stream, 20, placed, 10).await?;
    expect_observer_update_at(&mut observer, placed, 10).await?;

    block_mutation::mine_dirt_like_at(&mut actor.stream, 21, placed, 10, 0).await?;
    expect_observer_update_at(&mut observer, placed, 0).await
}

pub(super) async fn expect_observer_update_at(
    observer: &mut PlayClient,
    pos: BlockPos,
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
    block_mutation::validate_update_at(packet.data, pos, state)
}

fn spawn_ground(client: &PlayClient) -> BlockPos {
    BlockPos::new(
        client.spawn_block.x,
        client.spawn_block.y - 1,
        client.spawn_block.z,
    )
}

async fn move_to_spawn(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    live_play::send_position_look_at(
        &mut client.stream,
        f64::from(client.spawn_block.x) + 0.5,
        f64::from(client.spawn_block.y),
        f64::from(client.spawn_block.z) + 0.5,
        0.0,
        0.0,
    )
    .await
}
