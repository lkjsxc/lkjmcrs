use crate::player::{Inventory, InventorySlot, PlayerDefaults};

const MAX_STACK: u8 = 64;
const MAX_SYNCED_SLOT: i32 = 35;

impl Inventory {
    pub fn for_new_profile(_defaults: PlayerDefaults) -> Self {
        Self::default()
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

    pub fn can_add_simple_item(&self, item_id: &str, count: u8) -> bool {
        if count == 0 {
            return true;
        }
        let stack_room: u16 = self
            .slots
            .iter()
            .filter(|slot| slot.item_id == item_id)
            .map(|slot| u16::from(MAX_STACK.saturating_sub(slot.count)))
            .sum();
        let empty_slots = Self::synced_slot_ids()
            .filter(|candidate| self.slots.iter().all(|slot| slot.slot != *candidate))
            .count() as u16;
        stack_room + empty_slots * u16::from(MAX_STACK) >= u16::from(count)
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
