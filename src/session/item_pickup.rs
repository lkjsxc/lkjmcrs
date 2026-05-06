use crate::player::PlayerProfile;
use crate::scheduler::RegionHandle;
use crate::session::SessionState;
use crate::session::entity_packets;
use crate::session::error::ConnectionError;
use crate::session::inventory_sync;
use crate::session::play_state::PlaySession;
use crate::session::registry::SessionRegistry;
use tokio::io::AsyncWrite;

const PLAYER_ENTITY_ID: i32 = 1;
const PICKUP_ITEMS: [&str; 2] = ["minecraft:stone", "minecraft:dirt"];

pub async fn attempt_pickup<W>(
    writer: &mut W,
    phase: SessionState,
    region: &RegionHandle,
    sessions: &SessionRegistry,
    session: &PlaySession,
    profile: &mut PlayerProfile,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let accepted_items = PICKUP_ITEMS
        .iter()
        .filter(|item| profile.inventory.can_add_simple_item(item, 1))
        .map(|item| (*item).to_string())
        .collect::<Vec<_>>();
    if accepted_items.is_empty() {
        return Ok(());
    }
    let Some(item) = region
        .collect_nearby(session.x, session.y, session.z, accepted_items)
        .await
        .map_err(|source| ConnectionError::Region { phase, source })?
    else {
        return Ok(());
    };
    let before = profile.inventory.clone();
    profile.inventory.add_simple_item(&item.item_id, item.count);
    entity_packets::send_collect(writer, phase, &item, PLAYER_ENTITY_ID).await?;
    entity_packets::send_destroy(writer, phase, item.entity_id).await?;
    sessions
        .broadcast_item_collect(item.chunk, item.clone(), PLAYER_ENTITY_ID, session.id)
        .await;
    sessions
        .broadcast_item_destroy(item.chunk, item.entity_id, session.id)
        .await;
    inventory_sync::send_changed_slots(writer, phase, &before, &profile.inventory).await
}
