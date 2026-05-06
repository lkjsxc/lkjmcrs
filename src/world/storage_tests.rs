use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, WorldStorage};
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn schema_initializes_with_version_one() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);

    assert_eq!(storage.schema_version().unwrap(), 1);
    assert!(root.join("world.sqlite3").exists());
    cleanup(root);
}

#[test]
fn missing_chunk_loads_flat_base() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let loaded = storage.load_chunk(ChunkPos::new(0, 0)).unwrap();

    assert_eq!(
        loaded.block_at_pos(BlockPos::new(0, 80, 0)),
        BlockState::Air
    );
    cleanup(root);
}

#[test]
fn override_save_and_load_round_trips_multiple_blocks() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let pos = ChunkPos::new(-1, 0);
    let west = BlockPos::new(-1, 80, 0);
    let east = BlockPos::new(-16, 79, 15);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(west, BlockState::Stone);
    chunk.set_block(east, BlockState::Dirt);

    storage.save_chunk(&chunk).unwrap();
    let loaded = storage.load_chunk(pos).unwrap();

    assert_eq!(loaded.block_at_pos(west), BlockState::Stone);
    assert_eq!(loaded.block_at_pos(east), BlockState::Dirt);
    cleanup(root);
}

#[test]
fn reset_to_base_deletes_override_rows() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let pos = ChunkPos::new(0, 0);
    let block = BlockPos::new(0, 80, 0);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(block, BlockState::Stone);
    storage.save_chunk(&chunk).unwrap();

    chunk.set_block(block, BlockState::Air);
    storage.save_chunk(&chunk).unwrap();

    assert_eq!(override_count(&root), 0);
    cleanup(root);
}

#[test]
fn rejects_unsupported_schema() {
    let root = temp_root();
    fs::create_dir_all(&root).unwrap();
    let connection = Connection::open(root.join("world.sqlite3")).unwrap();
    connection.pragma_update(None, "user_version", 2).unwrap();

    let error = WorldStorage::new(&root)
        .load_chunk(ChunkPos::new(0, 0))
        .unwrap_err()
        .to_string();
    assert!(error.contains("schema version 2"));
    cleanup(root);
}

fn override_count(root: &Path) -> i64 {
    Connection::open(root.join("world.sqlite3"))
        .unwrap()
        .query_row("SELECT COUNT(*) FROM chunk_overrides", [], |row| row.get(0))
        .unwrap()
}

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("lkjmcrs-storage-{nanos}"))
}

fn cleanup(root: PathBuf) {
    let _ = fs::remove_dir_all(root);
}
