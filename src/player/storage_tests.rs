use crate::player::{
    GameMode, InventorySlot, NamedLocation, PlayerDefaults, PlayerPosition, PlayerProfile,
    PlayerStore,
};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[tokio::test]
async fn creates_default_profile_when_missing() {
    let root = temp_root();
    let store = PlayerStore::open(&root).unwrap();
    let profile = store
        .load_or_create(
            Uuid::from_u128(1),
            "Probe".to_string(),
            PlayerDefaults::default(),
        )
        .await
        .unwrap();

    assert_eq!(profile.name, "Probe");
    assert_eq!(profile.game_mode, GameMode::Survival);
    assert!(root.join("players.sqlite3").exists());
    cleanup(root);
}

#[tokio::test]
async fn creates_survival_profile_with_empty_inventory() {
    let root = temp_root();
    let store = PlayerStore::open(&root).unwrap();
    let profile = store
        .load_or_create(
            Uuid::from_u128(3),
            "Survival".to_string(),
            PlayerDefaults {
                game_mode: GameMode::Survival,
            },
        )
        .await
        .unwrap();

    assert_eq!(profile.game_mode, GameMode::Survival);
    assert!(profile.inventory.slots.is_empty());
    cleanup(root);
}

#[tokio::test]
async fn saves_and_reloads_profile_state() {
    let root = temp_root();
    let store = PlayerStore::open(&root).unwrap();
    let uuid = Uuid::from_u128(2);
    let mut profile = PlayerProfile::new(uuid, "Probe");
    profile.game_mode = GameMode::Survival;
    profile.position.x = 12.25;
    profile.position.y = 81.5;
    profile.position.z = -4.75;
    profile.position.yaw = 45.0;
    profile.inventory.selected_hotbar_slot = 2;
    profile.inventory.slots.push(InventorySlot {
        slot: 2,
        item_id: "minecraft:stone".to_string(),
        count: 3,
        data: None,
    });

    store.save(profile).await.unwrap();
    let loaded = store
        .load_or_create(uuid, "Probe".to_string(), PlayerDefaults::default())
        .await
        .unwrap();

    assert_eq!(loaded.game_mode, GameMode::Survival);
    assert_eq!(loaded.position.x, 12.25);
    assert_eq!(loaded.inventory.selected_hotbar_slot, 2);
    assert_eq!(loaded.inventory.slots[0].count, 3);
    cleanup(root);
}

#[tokio::test]
async fn concurrent_profile_saves_wait_for_sqlite_writer() {
    let root = temp_root();
    let store = PlayerStore::open(&root).unwrap();
    let mut tasks = Vec::new();

    for index in 0..8 {
        let store = store.clone();
        tasks.push(tokio::spawn(async move {
            let mut profile = PlayerProfile::new(Uuid::from_u128(100 + index), "Probe");
            profile.position.x = index as f64;
            profile.inventory.slots.push(InventorySlot {
                slot: 0,
                item_id: "minecraft:dirt".to_string(),
                count: 1,
                data: None,
            });
            store.save(profile).await
        }));
    }

    for task in tasks {
        task.await.unwrap().unwrap();
    }
    cleanup(root);
}

#[test]
fn rejects_unsupported_schema_version() {
    let root = temp_root();
    let path = root.join("players.sqlite3");
    fs::create_dir_all(&root).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection.pragma_update(None, "user_version", 1).unwrap();

    assert!(PlayerStore::open(&root).is_err());
    cleanup(root);
}

#[tokio::test]
async fn saves_homes_and_warps() {
    let root = temp_root();
    let store = PlayerStore::open(&root).unwrap();
    let uuid = Uuid::from_u128(4);
    let home = NamedLocation::overworld("base".to_string(), position(9.0, 82.0, -2.0));
    let warp = NamedLocation::overworld("spawnish".to_string(), position(1.0, 80.0, 1.0));

    store.set_home(uuid, home).await.unwrap();
    store.set_warp(uuid, warp).await.unwrap();

    assert_eq!(store.home_names(uuid).await.unwrap(), vec!["base"]);
    assert_eq!(store.warp_names().await.unwrap(), vec!["spawnish"]);
    assert_eq!(
        store
            .home(uuid, "base".to_string())
            .await
            .unwrap()
            .unwrap()
            .position
            .x,
        9.0
    );
    assert_eq!(
        store
            .warp("spawnish".to_string())
            .await
            .unwrap()
            .unwrap()
            .position
            .z,
        1.0
    );
    cleanup(root);
}

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("lkjmcrs-player-{nanos}"))
}

fn cleanup(root: PathBuf) {
    let _ = fs::remove_dir_all(root);
}

fn position(x: f64, y: f64, z: f64) -> PlayerPosition {
    PlayerPosition {
        x,
        y,
        z,
        yaw: 10.0,
        pitch: 20.0,
    }
}
