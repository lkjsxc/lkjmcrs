use crate::player::location_json;
use crate::player::storage::PlayerStoreError;
use crate::player::store_json::{decode_profile, encode_profile};
use crate::player::{NamedLocation, PlayerProfile};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::path::Path;
use uuid::Uuid;

const PROFILES: TableDefinition<&str, &[u8]> = TableDefinition::new("profiles");
const HOMES: TableDefinition<&str, &[u8]> = TableDefinition::new("homes");
const WARPS: TableDefinition<&str, &[u8]> = TableDefinition::new("warps");

pub(super) fn open_database(path: &Path) -> Result<Database, PlayerStoreError> {
    let database = Database::create(path).map_err(redb_error)?;
    let write = database.begin_write().map_err(redb_error)?;
    {
        write.open_table(PROFILES).map_err(redb_error)?;
        write.open_table(HOMES).map_err(redb_error)?;
        write.open_table(WARPS).map_err(redb_error)?;
    }
    write.commit().map_err(redb_error)?;
    Ok(database)
}

pub(super) fn load_profile(
    db: &Database,
    uuid: Uuid,
) -> Result<Option<PlayerProfile>, PlayerStoreError> {
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(PROFILES).map_err(redb_error)?;
    table
        .get(uuid.to_string().as_str())
        .map_err(redb_error)?
        .map(|value| decode_profile(value.value()))
        .transpose()
}

pub(super) fn save_profile(db: &Database, profile: &PlayerProfile) -> Result<(), PlayerStoreError> {
    let bytes = encode_profile(profile)?;
    commit_value(db, PROFILES, &profile.uuid.to_string(), &bytes)
}

pub(super) fn set_home(
    db: &Database,
    uuid: Uuid,
    location: &NamedLocation,
) -> Result<(), PlayerStoreError> {
    let bytes = location_json::encode_location(location)?;
    commit_value(
        db,
        HOMES,
        &location_json::home_key(uuid, &location.name),
        &bytes,
    )
}

pub(super) fn home(
    db: &Database,
    uuid: Uuid,
    name: &str,
) -> Result<Option<NamedLocation>, PlayerStoreError> {
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(HOMES).map_err(redb_error)?;
    table
        .get(location_json::home_key(uuid, name).as_str())
        .map_err(redb_error)?
        .map(|value| location_json::decode_location(value.value()))
        .transpose()
}

pub(super) fn home_names(db: &Database, uuid: Uuid) -> Result<Vec<String>, PlayerStoreError> {
    let prefix = location_json::home_prefix(uuid);
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(HOMES).map_err(redb_error)?;
    let mut names = Vec::new();
    for entry in table.iter().map_err(redb_error)? {
        let (key, _) = entry.map_err(redb_error)?;
        if let Some(name) = key.value().strip_prefix(&prefix) {
            names.push(name.to_string());
        }
    }
    names.sort();
    Ok(names)
}

pub(super) fn set_warp(
    db: &Database,
    created_by_uuid: Uuid,
    location: &NamedLocation,
) -> Result<(), PlayerStoreError> {
    let bytes = location_json::encode_warp(created_by_uuid, location)?;
    commit_value(db, WARPS, &location.name, &bytes)
}

pub(super) fn warp(db: &Database, name: &str) -> Result<Option<NamedLocation>, PlayerStoreError> {
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(WARPS).map_err(redb_error)?;
    table
        .get(name)
        .map_err(redb_error)?
        .map(|value| location_json::decode_warp(value.value()))
        .transpose()
}

pub(super) fn warp_names(db: &Database) -> Result<Vec<String>, PlayerStoreError> {
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(WARPS).map_err(redb_error)?;
    let mut names = Vec::new();
    for entry in table.iter().map_err(redb_error)? {
        let (key, _) = entry.map_err(redb_error)?;
        names.push(key.value().to_string());
    }
    names.sort();
    Ok(names)
}

fn commit_value(
    db: &Database,
    definition: TableDefinition<&str, &[u8]>,
    key: &str,
    bytes: &[u8],
) -> Result<(), PlayerStoreError> {
    let write = db.begin_write().map_err(redb_error)?;
    {
        let mut table = write.open_table(definition).map_err(redb_error)?;
        table.insert(key, bytes).map_err(redb_error)?;
    }
    write.commit().map_err(redb_error)?;
    Ok(())
}

fn redb_error(error: impl std::fmt::Display) -> PlayerStoreError {
    PlayerStoreError::Redb(error.to_string())
}
