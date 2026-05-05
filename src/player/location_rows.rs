use crate::player::storage::PlayerStoreError;
use crate::player::{NamedLocation, PlayerPosition};
use rusqlite::{Connection, OptionalExtension, params};
use uuid::Uuid;

type LocationRow = (String, String, f64, f64, f64, f64, f64);

pub(super) fn count_homes(connection: &Connection, uuid: Uuid) -> Result<usize, PlayerStoreError> {
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM player_homes WHERE uuid = ?1",
        [uuid.to_string()],
        |row| row.get(0),
    )?;
    usize::try_from(count).map_err(|_| PlayerStoreError::InvalidLocation)
}

pub(super) fn get_home(
    connection: &Connection,
    uuid: Uuid,
    name: &str,
) -> Result<Option<NamedLocation>, PlayerStoreError> {
    get_location(
        connection,
        "SELECT name, world, x, y, z, yaw, pitch
         FROM player_homes WHERE uuid = ?1 AND name = ?2",
        params![uuid.to_string(), name],
    )
}

pub(super) fn list_home_names(
    connection: &Connection,
    uuid: Uuid,
) -> Result<Vec<String>, PlayerStoreError> {
    list_names(
        connection,
        "SELECT name FROM player_homes WHERE uuid = ?1 ORDER BY name",
        params![uuid.to_string()],
    )
}

pub(super) fn upsert_home(
    connection: &Connection,
    uuid: Uuid,
    location: &NamedLocation,
) -> Result<(), PlayerStoreError> {
    connection.execute(
        "INSERT INTO player_homes
         (uuid, name, world, x, y, z, yaw, pitch)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(uuid, name) DO UPDATE SET
         world=excluded.world, x=excluded.x, y=excluded.y, z=excluded.z,
         yaw=excluded.yaw, pitch=excluded.pitch",
        params![
            uuid.to_string(),
            &location.name,
            &location.world,
            location.position.x,
            location.position.y,
            location.position.z,
            location.position.yaw,
            location.position.pitch
        ],
    )?;
    Ok(())
}

pub(super) fn get_warp(
    connection: &Connection,
    name: &str,
) -> Result<Option<NamedLocation>, PlayerStoreError> {
    get_location(
        connection,
        "SELECT name, world, x, y, z, yaw, pitch FROM warps WHERE name = ?1",
        params![name],
    )
}

pub(super) fn list_warp_names(connection: &Connection) -> Result<Vec<String>, PlayerStoreError> {
    list_names(
        connection,
        "SELECT name FROM warps ORDER BY name",
        params![],
    )
}

pub(super) fn upsert_warp(
    connection: &Connection,
    created_by_uuid: Uuid,
    location: &NamedLocation,
) -> Result<(), PlayerStoreError> {
    connection.execute(
        "INSERT INTO warps
         (name, world, x, y, z, yaw, pitch, created_by_uuid)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(name) DO UPDATE SET
         world=excluded.world, x=excluded.x, y=excluded.y, z=excluded.z,
         yaw=excluded.yaw, pitch=excluded.pitch,
         created_by_uuid=excluded.created_by_uuid",
        params![
            &location.name,
            &location.world,
            location.position.x,
            location.position.y,
            location.position.z,
            location.position.yaw,
            location.position.pitch,
            created_by_uuid.to_string()
        ],
    )?;
    Ok(())
}

fn get_location(
    connection: &Connection,
    sql: &str,
    params: impl rusqlite::Params,
) -> Result<Option<NamedLocation>, PlayerStoreError> {
    connection
        .query_row(sql, params, row_to_location)
        .optional()?
        .map(location_from_row)
        .transpose()
}

fn list_names(
    connection: &Connection,
    sql: &str,
    params: impl rusqlite::Params,
) -> Result<Vec<String>, PlayerStoreError> {
    let mut statement = connection.prepare(sql)?;
    let rows = statement.query_map(params, |row| row.get::<_, String>(0))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(PlayerStoreError::from)
}

fn row_to_location(row: &rusqlite::Row<'_>) -> Result<LocationRow, rusqlite::Error> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
    ))
}

fn location_from_row(row: LocationRow) -> Result<NamedLocation, PlayerStoreError> {
    Ok(NamedLocation {
        name: row.0,
        world: row.1,
        position: PlayerPosition {
            x: row.2,
            y: row.3,
            z: row.4,
            yaw: row.5 as f32,
            pitch: row.6 as f32,
        },
    })
}
