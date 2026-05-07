use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, WorldStorage};
use redb::{Database, TableDefinition};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn storage_initializes_redb_file() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);

    storage.validate().unwrap();
    assert!(root.join("world.redb").exists());
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
fn reset_to_base_deletes_override_value() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let pos = ChunkPos::new(0, 0);
    let block = BlockPos::new(0, 80, 0);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(block, BlockState::Stone);
    storage.save_chunk(&chunk).unwrap();

    chunk.set_block(block, BlockState::Air);
    storage.save_chunk(&chunk).unwrap();

    let loaded = storage.load_chunk(pos).unwrap();
    assert_eq!(loaded.block_at_pos(block), BlockState::Air);
    cleanup(root);
}

#[test]
fn cloned_storage_serializes_concurrent_saves() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let first = storage.clone();
    let second = storage.clone();
    let first_chunk = changed_chunk(ChunkPos::new(0, 0), BlockPos::new(0, 80, 0));
    let second_chunk = changed_chunk(ChunkPos::new(1, 0), BlockPos::new(16, 80, 0));

    let left = std::thread::spawn(move || first.save_chunk(&first_chunk));
    let right = std::thread::spawn(move || second.save_chunk(&second_chunk));

    left.join().unwrap().unwrap();
    right.join().unwrap().unwrap();
    assert_eq!(
        storage
            .load_chunk(ChunkPos::new(0, 0))
            .unwrap()
            .block_at_pos(BlockPos::new(0, 80, 0)),
        BlockState::Stone
    );
    assert_eq!(
        storage
            .load_chunk(ChunkPos::new(1, 0))
            .unwrap()
            .block_at_pos(BlockPos::new(16, 80, 0)),
        BlockState::Stone
    );
    cleanup(root);
}

#[test]
fn rejects_invalid_stored_block_state() {
    let root = temp_root();
    insert_raw_chunk(
        &root,
        "overworld/0/0",
        br#"{"chunk_x":0,"chunk_z":0,"overrides":[{"local_x":0,"y":80,"local_z":0,"state":"minecraft:void"}]}"#,
    );
    let storage = WorldStorage::new(&root);

    let error = storage
        .load_chunk(ChunkPos::new(0, 0))
        .unwrap_err()
        .to_string();
    assert!(error.contains("invalid block state"));
    cleanup(root);
}

fn changed_chunk(pos: ChunkPos, block: BlockPos) -> ChunkSnapshot {
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(block, BlockState::Stone);
    chunk
}

fn insert_raw_chunk(root: &std::path::Path, key: &str, bytes: &[u8]) {
    const CHUNKS: TableDefinition<&str, &[u8]> = TableDefinition::new("chunks");
    fs::create_dir_all(root).unwrap();
    let db = Database::create(root.join("world.redb")).unwrap();
    let write = db.begin_write().unwrap();
    {
        let mut table = write.open_table(CHUNKS).unwrap();
        table.insert(key, bytes).unwrap();
    }
    write.commit().unwrap();
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
