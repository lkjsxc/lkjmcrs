use crate::player::{Inventory, InventorySlot};

#[test]
fn selected_item_can_be_consumed() {
    let mut inventory = Inventory {
        selected_hotbar_slot: 0,
        slots: vec![InventorySlot {
            slot: 0,
            item_id: "minecraft:stone".to_string(),
            count: 1,
            data: None,
        }],
    };

    assert!(inventory.consume_selected("minecraft:stone"));
    assert!(inventory.slots.is_empty());
}

#[test]
fn simple_items_stack_before_new_slots() {
    let mut inventory = Inventory {
        selected_hotbar_slot: 0,
        slots: vec![InventorySlot {
            slot: 0,
            item_id: "minecraft:dirt".to_string(),
            count: 63,
            data: None,
        }],
    };

    inventory.add_simple_item("minecraft:dirt", 2);

    assert_eq!(inventory.slots[0].count, 64);
    assert_eq!(inventory.slots[1].count, 1);
}

#[test]
fn simple_items_use_synced_slot_bounds() {
    let mut inventory = Inventory {
        selected_hotbar_slot: 0,
        slots: (0..=35)
            .map(|slot| InventorySlot {
                slot,
                item_id: format!("minecraft:test_{slot}"),
                count: 1,
                data: None,
            })
            .collect(),
    };

    inventory.add_simple_item("minecraft:dirt", 1);

    assert_eq!(inventory.slots.len(), 36);
    assert!(Inventory::is_synced_slot(35));
    assert!(!Inventory::is_synced_slot(36));
    assert!(Inventory::is_hotbar_slot(8));
    assert!(!Inventory::is_hotbar_slot(9));
}

#[test]
fn selected_item_ignores_empty_slots() {
    let inventory = Inventory {
        selected_hotbar_slot: 0,
        slots: vec![InventorySlot {
            slot: 0,
            item_id: "minecraft:stone".to_string(),
            count: 0,
            data: None,
        }],
    };

    assert_eq!(inventory.selected_item_id(), None);
}

#[test]
fn capacity_detects_full_inventory() {
    let inventory = Inventory {
        selected_hotbar_slot: 0,
        slots: (0..=35)
            .map(|slot| InventorySlot {
                slot,
                item_id: format!("minecraft:test_{slot}"),
                count: 64,
                data: None,
            })
            .collect(),
    };

    assert!(!inventory.can_add_simple_item("minecraft:dirt", 1));
}
