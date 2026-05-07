use crate::player::{Inventory, PlayerProfile, PlayerStoreError};

pub(super) fn encode_profile(profile: &PlayerProfile) -> Result<Vec<u8>, PlayerStoreError> {
    validate_inventory(&profile.inventory)?;
    serde_json::to_vec(profile).map_err(PlayerStoreError::from)
}

pub(super) fn decode_profile(bytes: &[u8]) -> Result<PlayerProfile, PlayerStoreError> {
    let profile: PlayerProfile = serde_json::from_slice(bytes)?;
    validate_inventory(&profile.inventory)?;
    Ok(profile)
}

fn validate_inventory(inventory: &Inventory) -> Result<(), PlayerStoreError> {
    if inventory.selected_hotbar_slot > 8 {
        return Err(PlayerStoreError::InvalidSelectedHotbarSlot);
    }
    for slot in &inventory.slots {
        if slot.slot < 0 || slot.count == 0 {
            return Err(PlayerStoreError::InvalidInventorySlot);
        }
    }
    Ok(())
}
