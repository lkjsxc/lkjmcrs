use super::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, WorldStorage};
use redb::{Database, ReadableDatabase, TableDefinition};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn save_removes_only_sections_that_reset_to_base() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let pos = ChunkPos::new(0, 0);
    let low = BlockPos::new(0, 80, 0);
    let high = BlockPos::new(0, 96, 0);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(low, BlockState::Stone);
    chunk.set_block(high, BlockState::Stone);
    storage.save_chunk(&chunk).unwrap();

    chunk.set_block(low, BlockState::Air);
    storage.save_chunk(&chunk).unwrap();

    drop(storage);
    assert!(table_value_exists(&root, "chunk_meta", "overworld/0/0"));
    assert!(!table_value_exists(
        &root,
        "chunk_sections",
        "overworld/0/0/5"
    ));
    assert!(table_value_exists(
        &root,
        "chunk_sections",
        "overworld/0/0/6"
    ));
    cleanup(root);
}

fn table_value_exists(root: &std::path::Path, table: &str, key: &str) -> bool {
    let definition: TableDefinition<&str, &[u8]> = TableDefinition::new(table);
    let db = Database::create(root.join("world.redb")).unwrap();
    let read = db.begin_read().unwrap();
    let table = read.open_table(definition).unwrap();
    table.get(key).unwrap().is_some()
}

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("lkjmcrs-storage-redb-{nanos}"))
}

fn cleanup(root: PathBuf) {
    let _ = fs::remove_dir_all(root);
}
