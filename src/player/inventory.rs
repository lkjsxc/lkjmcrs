use crate::player::{GameMode, Inventory, InventorySlot, PlayerDefaults};

const MAX_STACK: u8 = 64;
const MAX_SYNCED_SLOT: i32 = 35;
const STONE_ITEM: &str = "minecraft:stone";

impl Inventory {
    pub fn for_new_profile(defaults: PlayerDefaults) -> Self {
        let mut inventory = Self::default();
        if defaults.game_mode == GameMode::Survival && defaults.survival_starter_stone > 0 {
            inventory.slots.push(InventorySlot {
                slot: 0,
                item_id: STONE_ITEM.to_string(),
                count: defaults.survival_starter_stone.min(MAX_STACK),
                data: None,
            });
        }
        inventory
    }

    pub fn selected_item_count(&self, item_id: &str) -> u8 {
        self.selected_slot()
            .filter(|slot| slot.item_id == item_id)
            .map(|slot| slot.count)
            .unwrap_or(0)
    }

    pub fn selected_item_id(&self) -> Option<&str> {
        self.selected_slot()
            .filter(|slot| slot.count > 0)
            .map(|slot| slot.item_id.as_str())
    }

    pub fn consume_selected(&mut self, item_id: &str) -> bool {
        let slot_id = i32::from(self.selected_hotbar_slot);
        let Some(index) = self
            .slots
            .iter()
            .position(|slot| slot.slot == slot_id && slot.item_id == item_id && slot.count > 0)
        else {
            return false;
        };
        self.slots[index].count -= 1;
        if self.slots[index].count == 0 {
            self.slots.remove(index);
        }
        true
    }

    pub fn add_simple_item(&mut self, item_id: &str, count: u8) {
        if count == 0 {
            return;
        }
        let mut remaining = count;
        for slot in self.slots.iter_mut().filter(|slot| slot.item_id == item_id) {
            let room = MAX_STACK.saturating_sub(slot.count);
            let add = remaining.min(room);
            slot.count += add;
            remaining -= add;
            if remaining == 0 {
                return;
            }
        }
        if remaining > 0
            && let Some(slot) = self.next_slot()
        {
            self.slots.push(InventorySlot {
                slot,
                item_id: item_id.to_string(),
                count: remaining.min(MAX_STACK),
                data: None,
            });
        }
    }

    pub fn slot(&self, slot_id: i32) -> Option<&InventorySlot> {
        self.slots
            .iter()
            .find(|slot| slot.slot == slot_id && slot.count > 0)
    }

    pub fn synced_slot_ids() -> std::ops::RangeInclusive<i32> {
        0..=MAX_SYNCED_SLOT
    }

    pub fn is_synced_slot(slot_id: i32) -> bool {
        Self::synced_slot_ids().contains(&slot_id)
    }

    pub fn is_hotbar_slot(slot_id: i16) -> bool {
        (0..=8).contains(&slot_id)
    }

    fn selected_slot(&self) -> Option<&InventorySlot> {
        let slot_id = i32::from(self.selected_hotbar_slot);
        self.slots.iter().find(|slot| slot.slot == slot_id)
    }

    fn next_slot(&self) -> Option<i32> {
        Self::synced_slot_ids()
            .find(|candidate| self.slots.iter().all(|slot| slot.slot != *candidate))
    }
}

#[cfg(test)]
mod tests {
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
}
