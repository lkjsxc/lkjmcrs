use crate::player::{GameMode, Inventory, InventorySlot, PlayerDefaults};

const MAX_STACK: u8 = 64;
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
        if remaining > 0 {
            self.slots.push(InventorySlot {
                slot: self.next_slot(),
                item_id: item_id.to_string(),
                count: remaining.min(MAX_STACK),
                data: None,
            });
        }
    }

    fn selected_slot(&self) -> Option<&InventorySlot> {
        let slot_id = i32::from(self.selected_hotbar_slot);
        self.slots.iter().find(|slot| slot.slot == slot_id)
    }

    fn next_slot(&self) -> i32 {
        (0..=255)
            .find(|candidate| self.slots.iter().all(|slot| slot.slot != *candidate))
            .unwrap_or(255)
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
}
