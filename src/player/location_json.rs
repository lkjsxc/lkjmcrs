use crate::player::{NamedLocation, OVERWORLD, PlayerStoreError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct StoredWarp {
    pub location: NamedLocation,
    pub created_by_uuid: Uuid,
}

pub(super) fn encode_location(location: &NamedLocation) -> Result<Vec<u8>, PlayerStoreError> {
    validate_location(location)?;
    serde_json::to_vec(location).map_err(PlayerStoreError::from)
}

pub(super) fn decode_location(bytes: &[u8]) -> Result<NamedLocation, PlayerStoreError> {
    let location: NamedLocation = serde_json::from_slice(bytes)?;
    validate_location(&location)?;
    Ok(location)
}

pub(super) fn encode_warp(
    created_by_uuid: Uuid,
    location: &NamedLocation,
) -> Result<Vec<u8>, PlayerStoreError> {
    validate_location(location)?;
    serde_json::to_vec(&StoredWarp {
        location: location.clone(),
        created_by_uuid,
    })
    .map_err(PlayerStoreError::from)
}

pub(super) fn decode_warp(bytes: &[u8]) -> Result<NamedLocation, PlayerStoreError> {
    let warp: StoredWarp = serde_json::from_slice(bytes)?;
    validate_location(&warp.location)?;
    Ok(warp.location)
}

pub(super) fn home_key(uuid: Uuid, name: &str) -> String {
    format!("{uuid}/{name}")
}

pub(super) fn home_prefix(uuid: Uuid) -> String {
    format!("{uuid}/")
}

fn validate_location(location: &NamedLocation) -> Result<(), PlayerStoreError> {
    if location.name.is_empty() || location.world != OVERWORLD {
        Err(PlayerStoreError::InvalidLocation)
    } else {
        Ok(())
    }
}
