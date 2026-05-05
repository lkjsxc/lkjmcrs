use crate::player::storage::PlayerStoreError;
use rusqlite::Connection;

const SCHEMA_VERSION: i32 = 3;

pub(super) fn initialize_schema(connection: &Connection) -> Result<(), PlayerStoreError> {
    let version: i32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version != 0 && version != SCHEMA_VERSION {
        return Err(PlayerStoreError::UnsupportedSchema(version));
    }
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS player_profiles (
          uuid TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          game_mode TEXT NOT NULL,
          x REAL NOT NULL,
          y REAL NOT NULL,
          z REAL NOT NULL,
          yaw REAL NOT NULL,
          pitch REAL NOT NULL,
          selected_hotbar_slot INTEGER NOT NULL,
          health REAL NOT NULL,
          hunger INTEGER NOT NULL,
          saturation REAL NOT NULL
        );
        CREATE TABLE IF NOT EXISTS player_inventory_slots (
          uuid TEXT NOT NULL,
          slot INTEGER NOT NULL,
          item_id TEXT NOT NULL,
          count INTEGER NOT NULL,
          data TEXT,
          PRIMARY KEY (uuid, slot)
        );
        CREATE TABLE IF NOT EXISTS player_homes (
          uuid TEXT NOT NULL,
          name TEXT NOT NULL,
          world TEXT NOT NULL,
          x REAL NOT NULL,
          y REAL NOT NULL,
          z REAL NOT NULL,
          yaw REAL NOT NULL,
          pitch REAL NOT NULL,
          PRIMARY KEY (uuid, name)
        );
        CREATE TABLE IF NOT EXISTS warps (
          name TEXT PRIMARY KEY,
          world TEXT NOT NULL,
          x REAL NOT NULL,
          y REAL NOT NULL,
          z REAL NOT NULL,
          yaw REAL NOT NULL,
          pitch REAL NOT NULL,
          created_by_uuid TEXT NOT NULL
        );
        PRAGMA user_version = 3;
        ",
    )?;
    Ok(())
}
