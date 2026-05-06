use crate::player::Inventory;
use crate::protocol::{ids, inventory};
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use tokio::io::AsyncWrite;

pub async fn send_bootstrap_inventory<W>(
    writer: &mut W,
    phase: SessionState,
    player_inventory: &Inventory,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    send_held_item_slot(writer, phase, player_inventory.selected_hotbar_slot).await?;
    for slot_id in Inventory::synced_slot_ids() {
        send_slot(writer, phase, player_inventory, slot_id).await?;
    }
    Ok(())
}

pub async fn send_held_item_slot<W>(
    writer: &mut W,
    phase: SessionState,
    selected_slot: u8,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::HELD_ITEM_SLOT,
        &inventory::encode_held_item_slot(selected_slot),
    )
    .await
}

pub async fn send_changed_slots<W>(
    writer: &mut W,
    phase: SessionState,
    before: &Inventory,
    after: &Inventory,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    for slot_id in changed_slots(before, after) {
        send_slot(writer, phase, after, slot_id).await?;
    }
    Ok(())
}

fn changed_slots(before: &Inventory, after: &Inventory) -> Vec<i32> {
    Inventory::synced_slot_ids()
        .filter(|slot_id| before.slot(*slot_id) != after.slot(*slot_id))
        .collect()
}

async fn send_slot<W>(
    writer: &mut W,
    phase: SessionState,
    player_inventory: &Inventory,
    slot_id: i32,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::SET_PLAYER_INVENTORY,
        &inventory::encode_set_player_inventory(slot_id, player_inventory.slot(slot_id)),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::changed_slots;
    use crate::player::{Inventory, InventorySlot};

    #[test]
    fn detects_changed_synced_slots() {
        let before = Inventory::default();
        let after = Inventory {
            selected_hotbar_slot: 0,
            slots: vec![InventorySlot {
                slot: 35,
                item_id: "minecraft:dirt".to_string(),
                count: 1,
                data: None,
            }],
        };

        assert_eq!(changed_slots(&before, &after), vec![35]);
    }
}
