use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot, WorldStorage};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn saves_and_loads_one_overridden_chunk() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let pos = ChunkPos::new(-1, 0);
    let block = BlockPos::new(-1, 80, 0);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(block, BlockState::Stone);

    storage.save_chunk(&chunk).unwrap();
    let loaded = storage.load_chunk(pos).unwrap();

    assert_eq!(loaded.block_at_pos(block), BlockState::Stone);
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
fn reset_to_base_deletes_chunk_file() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let pos = ChunkPos::new(0, 0);
    let block = BlockPos::new(0, 80, 0);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(block, BlockState::Stone);
    storage.save_chunk(&chunk).unwrap();

    chunk.set_block(block, BlockState::Air);
    storage.save_chunk(&chunk).unwrap();

    assert!(!root.join("chunks/c.0.0.json").exists());
    cleanup(root);
}

#[test]
fn rejects_coordinate_mismatch() {
    let root = temp_root();
    fs::create_dir_all(root.join("chunks")).unwrap();
    fs::write(
        root.join("chunks/c.0.0.json"),
        r#"{"schema":1,"chunk_x":1,"chunk_z":0,"overrides":[]}"#,
    )
    .unwrap();

    assert!(storage_error(&root).contains("coordinate mismatch"));
    cleanup(root);
}

#[test]
fn rejects_unsupported_schema() {
    let root = temp_root();
    fs::create_dir_all(root.join("chunks")).unwrap();
    fs::write(
        root.join("chunks/c.0.0.json"),
        r#"{"schema":2,"chunk_x":0,"chunk_z":0,"overrides":[]}"#,
    )
    .unwrap();

    assert!(storage_error(&root).contains("schema version"));
    cleanup(root);
}

fn storage_error(root: &PathBuf) -> String {
    WorldStorage::new(root)
        .load_chunk(ChunkPos::new(0, 0))
        .unwrap_err()
        .to_string()
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
