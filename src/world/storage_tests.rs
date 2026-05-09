use crate::world::storage_codec::StoredChunk;
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, TerrainGenerator, WorldStorage};
use redb::{Database, ReadableDatabase, TableDefinition};
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
fn missing_chunk_loads_configured_generator_base() {
    let root = temp_root();
    let storage = WorldStorage::with_generator(&root, TerrainGenerator::natural(19));
    let loaded = storage.load_chunk(ChunkPos::new(2, 0)).unwrap();

    assert!(!loaded.is_shared_flat_base());
    assert_ne!(
        loaded.base_entries_for_tests(),
        ChunkSnapshot::flat(ChunkPos::new(2, 0)).base_entries_for_tests()
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
    drop(storage);
    assert!(!chunk_value_exists(&root, "overworld/0/0"));
    cleanup(root);
}

#[test]
fn natural_override_resets_to_generated_base() {
    let root = temp_root();
    let storage = WorldStorage::with_generator(&root, TerrainGenerator::natural(23));
    let pos = ChunkPos::new(2, 0);
    let base = storage.load_chunk(pos).unwrap();
    let target = base
        .base_entries_for_tests()
        .into_iter()
        .find(|(x, _, z, state)| *x == 0 && *z == 0 && *state == BlockState::GrassBlock)
        .map(|(_, y, _, state)| (BlockPos::new(32, y, 0), state))
        .unwrap();
    let mut changed = storage.load_chunk(pos).unwrap();
    changed.set_block(target.0, BlockState::Stone);
    storage.save_chunk(&changed).unwrap();

    changed.set_block(target.0, target.1);
    storage.save_chunk(&changed).unwrap();

    assert_eq!(
        storage.load_chunk(pos).unwrap().block_at_pos(target.0),
        target.1
    );
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
    let mut bytes = encoded_chunk(ChunkPos::new(0, 0), BlockPos::new(0, 80, 0));
    bytes[26..28].copy_from_slice(&99_u16.to_le_bytes());
    insert_raw_chunk(&root, "overworld/0/0", &bytes);
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
    const CHUNKS: TableDefinition<&str, &[u8]> = TableDefinition::new("chunk_overrides");
    fs::create_dir_all(root).unwrap();
    let db = Database::create(root.join("world.redb")).unwrap();
    let write = db.begin_write().unwrap();
    {
        let mut table = write.open_table(CHUNKS).unwrap();
        table.insert(key, bytes).unwrap();
    }
    write.commit().unwrap();
}

fn chunk_value_exists(root: &std::path::Path, key: &str) -> bool {
    const CHUNKS: TableDefinition<&str, &[u8]> = TableDefinition::new("chunk_overrides");
    let db = Database::create(root.join("world.redb")).unwrap();
    let read = db.begin_read().unwrap();
    let table = read.open_table(CHUNKS).unwrap();
    table.get(key).unwrap().is_some()
}

fn encoded_chunk(pos: ChunkPos, block: BlockPos) -> Vec<u8> {
    let chunk = changed_chunk(pos, block);
    StoredChunk::from_snapshot(&chunk).encode().unwrap()
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
