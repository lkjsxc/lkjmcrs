use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::item_entities;
use crate::probe::play_client::PlayClient;
use crate::probe::position::BlockPos;
use crate::probe::survival_expect;
use crate::protocol::ids;

const NAME: &str = "ItemPickup";

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlayClient::connect_with_block(host, NAME, Some(0)).await?;
    block_mutation::send_start_destroy_at(&mut client.stream, 70, BlockPos::new(5, 79, 0)).await?;
    expect_ack_and_update(&mut client.stream).await?;
    let slot =
        item_entities::collect_drop_at(&mut client.stream, 28, "item pickup", 5.5, 80.0, 0.5)
            .await?;
    if slot.item_count != 1 || slot.item_id != Some(28) {
        return Err(Box::new(ProbeError::Phase("item pickup inventory")));
    }
    Ok(())
}

async fn expect_ack_and_update(
    stream: &mut tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let ack = survival_expect::read_next_material_packet(stream, "item pickup ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("item pickup ack id")));
    }
    block_mutation::validate_ack(ack.data, 70)?;
    let update = survival_expect::read_next_material_packet(stream, "item pickup update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("item pickup update id")));
    }
    survival_expect::validate_update_state(
        update.data,
        BlockPos::new(5, 79, 0),
        0,
        "item pickup update",
    )
}
