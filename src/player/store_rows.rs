use crate::player::storage::PlayerStoreError;
use crate::player::{GameMode, Inventory, InventorySlot, PlayerPosition, PlayerProfile, Vitals};
use rusqlite::{Connection, OptionalExtension, params};
use uuid::Uuid;

type ProfileRow = (String, String, f64, f64, f64, f64, f64, i64, f64, i64, f64);

pub(super) fn load_profile(
    connection: &Connection,
    uuid: Uuid,
) -> Result<Option<PlayerProfile>, PlayerStoreError> {
    let row = connection
        .query_row(
            "SELECT name, game_mode, x, y, z, yaw, pitch, selected_hotbar_slot,
             health, hunger, saturation
             FROM player_profiles WHERE uuid = ?1",
            [uuid.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, f64>(2)?,
                    row.get::<_, f64>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, f64>(5)?,
                    row.get::<_, f64>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, f64>(8)?,
                    row.get::<_, i64>(9)?,
                    row.get::<_, f64>(10)?,
                ))
            },
        )
        .optional()?;
    row.map(|row| profile_from_row(connection, uuid, row))
        .transpose()
}

pub(super) fn save_profile(
    connection: &mut Connection,
    profile: &PlayerProfile,
) -> Result<(), PlayerStoreError> {
    checked_hotbar(i64::from(profile.inventory.selected_hotbar_slot))?;
    let tx = connection.transaction()?;
    let uuid = profile.uuid.to_string();
    tx.execute(
        profile_upsert_sql(),
        params![
            uuid,
            &profile.name,
            profile.game_mode.as_str(),
            profile.position.x,
            profile.position.y,
            profile.position.z,
            profile.position.yaw,
            profile.position.pitch,
            profile.inventory.selected_hotbar_slot,
            profile.vitals.health,
            profile.vitals.hunger,
            profile.vitals.saturation
        ],
    )?;
    tx.execute(
        "DELETE FROM player_inventory_slots WHERE uuid = ?1",
        [profile.uuid.to_string()],
    )?;
    for slot in profile.inventory.slots.iter().filter(|slot| slot.count > 0) {
        save_slot(&tx, profile.uuid, slot)?;
    }
    tx.commit()?;
    Ok(())
}

fn profile_from_row(
    connection: &Connection,
    uuid: Uuid,
    row: ProfileRow,
) -> Result<PlayerProfile, PlayerStoreError> {
    let game_mode =
        GameMode::parse(&row.1).ok_or_else(|| PlayerStoreError::InvalidGameMode(row.1.clone()))?;
    Ok(PlayerProfile {
        uuid,
        name: row.0,
        game_mode,
        position: PlayerPosition {
            x: row.2,
            y: row.3,
            z: row.4,
            yaw: row.5 as f32,
            pitch: row.6 as f32,
        },
        inventory: Inventory {
            selected_hotbar_slot: checked_hotbar(row.7)?,
            slots: load_slots(connection, uuid)?,
        },
        vitals: Vitals {
            health: row.8 as f32,
            hunger: checked_u8(row.9)?,
            saturation: row.10 as f32,
        },
    })
}

fn load_slots(connection: &Connection, uuid: Uuid) -> Result<Vec<InventorySlot>, PlayerStoreError> {
    let mut statement = connection.prepare(
        "SELECT slot, item_id, count, data FROM player_inventory_slots
         WHERE uuid = ?1 ORDER BY slot",
    )?;
    let rows = statement.query_map([uuid.to_string()], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, Option<String>>(3)?,
        ))
    })?;
    rows.map(|row| slot_from_row(row?)).collect()
}

fn slot_from_row(
    row: (i32, String, i64, Option<String>),
) -> Result<InventorySlot, PlayerStoreError> {
    let count = checked_u8(row.2)?;
    if row.0 < 0 || count == 0 {
        return Err(PlayerStoreError::InvalidInventorySlot);
    }
    Ok(InventorySlot {
        slot: row.0,
        item_id: row.1,
        count,
        data: row.3,
    })
}

fn save_slot(
    connection: &Connection,
    uuid: Uuid,
    slot: &InventorySlot,
) -> Result<(), PlayerStoreError> {
    if slot.slot < 0 || slot.count == 0 {
        return Err(PlayerStoreError::InvalidInventorySlot);
    }
    connection.execute(
        "INSERT INTO player_inventory_slots
         (uuid, slot, item_id, count, data) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            uuid.to_string(),
            slot.slot,
            slot.item_id,
            slot.count,
            slot.data
        ],
    )?;
    Ok(())
}

fn checked_u8(value: i64) -> Result<u8, PlayerStoreError> {
    u8::try_from(value).map_err(|_| PlayerStoreError::InvalidInventorySlot)
}

fn checked_hotbar(value: i64) -> Result<u8, PlayerStoreError> {
    let slot = checked_u8(value)?;
    if slot <= 8 {
        Ok(slot)
    } else {
        Err(PlayerStoreError::InvalidSelectedHotbarSlot)
    }
}

fn profile_upsert_sql() -> &'static str {
    "INSERT INTO player_profiles
     (uuid, name, game_mode, x, y, z, yaw, pitch, selected_hotbar_slot,
      health, hunger, saturation)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
     ON CONFLICT(uuid) DO UPDATE SET
     name=excluded.name, game_mode=excluded.game_mode, x=excluded.x,
     y=excluded.y, z=excluded.z, yaw=excluded.yaw, pitch=excluded.pitch,
     selected_hotbar_slot=excluded.selected_hotbar_slot,
     health=excluded.health, hunger=excluded.hunger,
     saturation=excluded.saturation"
}
