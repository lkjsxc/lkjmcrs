use crate::world::WorldStorageError;
use rusqlite::Connection;

pub const SCHEMA_VERSION: u32 = 1;

pub fn ensure_schema(connection: &Connection) -> Result<(), WorldStorageError> {
    let version: u32 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    match version {
        0 => create_schema(connection),
        SCHEMA_VERSION => Ok(()),
        other => Err(WorldStorageError::UnsupportedSchema(other)),
    }
}

fn create_schema(connection: &Connection) -> Result<(), WorldStorageError> {
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS chunk_overrides (
            chunk_x INTEGER NOT NULL,
            chunk_z INTEGER NOT NULL,
            local_x INTEGER NOT NULL CHECK(local_x BETWEEN 0 AND 15),
            y INTEGER NOT NULL,
            local_z INTEGER NOT NULL CHECK(local_z BETWEEN 0 AND 15),
            state TEXT NOT NULL,
            PRIMARY KEY (chunk_x, chunk_z, local_x, y, local_z)
        );
        PRAGMA user_version = 1;",
    )?;
    Ok(())
}
