use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::item_entities;
use crate::probe::play_client::PlayClient;
use crate::probe::position::BlockPos;

const NAME: &str = "ItemPickup";

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlayClient::connect_with_block(host, NAME, Some(0)).await?;
    block_mutation::mine_dirt_like_at(&mut client.stream, 70, BlockPos::new(5, 79, 0), 9, 0)
        .await?;
    let slot =
        item_entities::collect_drop_at(&mut client.stream, 28, "item pickup", 5.5, 80.0, 0.5)
            .await?;
    if slot.item_count != 1 || slot.item_id != Some(28) {
        return Err(Box::new(ProbeError::Phase("item pickup inventory")));
    }
    Ok(())
}
