use crate::player::{Inventory, InventorySlot};
use crate::session::block_rules::{placement_state, simple_drop};
use crate::world::BlockState;

#[test]
fn simple_drops_match_contract() {
    assert_eq!(
        simple_drop(Some(BlockState::Stone)),
        Some("minecraft:stone")
    );
    assert_eq!(simple_drop(Some(BlockState::Dirt)), Some("minecraft:dirt"));
    assert_eq!(
        simple_drop(Some(BlockState::GrassBlock)),
        Some("minecraft:dirt")
    );
    assert_eq!(simple_drop(Some(BlockState::Air)), None);
    assert_eq!(simple_drop(Some(BlockState::Bedrock)), None);
    assert_eq!(simple_drop(Some(BlockState::Water)), None);
}

#[test]
fn survival_placement_uses_selected_material() {
    let inventory = Inventory {
        selected_hotbar_slot: 0,
        slots: vec![InventorySlot {
            slot: 0,
            item_id: "minecraft:dirt".to_string(),
            count: 1,
            data: None,
        }],
    };

    assert_eq!(
        placement_state(&inventory),
        Some((BlockState::Dirt, "minecraft:dirt"))
    );
}

#[test]
fn unsupported_survival_item_cannot_place() {
    let inventory = Inventory {
        selected_hotbar_slot: 0,
        slots: vec![InventorySlot {
            slot: 0,
            item_id: "minecraft:stick".to_string(),
            count: 1,
            data: None,
        }],
    };

    assert_eq!(placement_state(&inventory), None);
}
