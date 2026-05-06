use crate::player::InventorySlot;
use crate::protocol::codec;

pub const STONE_ITEM_ID: i32 = 1;
pub const DIRT_ITEM_ID: i32 = 28;

pub fn encode_held_item_slot(slot: u8) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, i32::from(slot));
    out
}

pub fn encode_set_player_inventory(slot_id: i32, slot: Option<&InventorySlot>) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, slot_id);
    encode_slot(&mut out, slot);
    out
}

pub fn encode_slot(out: &mut Vec<u8>, slot: Option<&InventorySlot>) {
    let Some(slot) = slot.filter(|slot| slot.count > 0) else {
        codec::write_var_i32(out, 0);
        return;
    };
    let Some(item_id) = protocol_item_id(&slot.item_id) else {
        codec::write_var_i32(out, 0);
        return;
    };
    codec::write_var_i32(out, i32::from(slot.count));
    codec::write_var_i32(out, item_id);
    codec::write_var_i32(out, 0);
    codec::write_var_i32(out, 0);
}

fn protocol_item_id(item_id: &str) -> Option<i32> {
    match item_id {
        "minecraft:stone" => Some(STONE_ITEM_ID),
        "minecraft:dirt" => Some(DIRT_ITEM_ID),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{encode_set_player_inventory, encode_slot};
    use crate::player::InventorySlot;

    #[test]
    fn encodes_empty_slot() {
        let mut out = Vec::new();
        encode_slot(&mut out, None);
        assert_eq!(out, vec![0]);
    }

    #[test]
    fn encodes_stone_slot() {
        let slot = InventorySlot {
            slot: 0,
            item_id: "minecraft:stone".to_string(),
            count: 3,
            data: None,
        };
        let mut out = Vec::new();
        encode_slot(&mut out, Some(&slot));
        assert_eq!(out, vec![3, 1, 0, 0]);
    }

    #[test]
    fn encodes_dirt_player_inventory_slot() {
        let slot = InventorySlot {
            slot: 2,
            item_id: "minecraft:dirt".to_string(),
            count: 1,
            data: None,
        };
        assert_eq!(
            encode_set_player_inventory(2, Some(&slot)),
            vec![2, 1, 28, 0, 0]
        );
    }
}
