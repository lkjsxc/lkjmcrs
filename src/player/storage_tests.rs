use crate::player::{GameMode, InventorySlot, PlayerDefaults, PlayerProfile, PlayerStore};
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
    assert_eq!(profile.game_mode, GameMode::Creative);
    assert!(root.join("players.sqlite3").exists());
    cleanup(root);
}

#[tokio::test]
async fn creates_survival_profile_with_starter_items() {
    let root = temp_root();
    let store = PlayerStore::open(&root).unwrap();
    let profile = store
        .load_or_create(
            Uuid::from_u128(3),
            "Survival".to_string(),
            PlayerDefaults {
                game_mode: GameMode::Survival,
                survival_starter_stone: 4,
            },
        )
        .await
        .unwrap();

    assert_eq!(profile.game_mode, GameMode::Survival);
    assert_eq!(profile.inventory.slots[0].count, 4);
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
