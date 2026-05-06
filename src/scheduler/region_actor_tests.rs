use crate::scheduler::RegionActor;
use crate::world::{BlockPos, BlockState, ChunkPos, RegionId, WorldStorage};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn applies_tasks_in_mailbox_order() {
    let handle = RegionActor::spawn(RegionId(7));
    assert_eq!(handle.id(), RegionId(7));
    assert_eq!(handle.apply("a").await.unwrap(), 1);
    assert_eq!(handle.apply("b").await.unwrap(), 2);
    assert_eq!(handle.applied_count().await.unwrap(), 2);
}

#[tokio::test]
async fn owns_spawn_chunks_and_mutations() {
    let handle = RegionActor::spawn(RegionId(1));
    assert_eq!(handle.spawn_chunks(1).await.unwrap().len(), 9);
    let pos = BlockPos::new(0, 80, 0);
    assert_eq!(handle.get_block(pos).await.unwrap(), Some(BlockState::Air));
    let mutation = handle.set_block(pos, BlockState::Stone).await.unwrap();
    assert_eq!(mutation.state, BlockState::Stone);
    assert!(mutation.accepted());
    assert!(mutation.changed);
    let chunk = handle
        .chunk_snapshot(ChunkPos::new(0, 0))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chunk.block_at_pos(pos), BlockState::Stone);
}

#[tokio::test]
async fn does_not_create_unloaded_chunks_on_mutation() {
    let handle = RegionActor::spawn(RegionId(1));
    let mutation = handle
        .set_block(BlockPos::new(1000, 80, 0), BlockState::Stone)
        .await
        .unwrap();
    assert!(!mutation.loaded);
    assert!(!mutation.accepted());
    assert!(!mutation.changed);
}

#[tokio::test]
async fn processes_mailbox_while_storage_load_is_pending() {
    let root = temp_root();
    let storage = WorldStorage::with_delay_for_tests(&root, Duration::from_millis(200));
    let handle = RegionActor::spawn_persistent(RegionId(2), storage);
    let loading = tokio::spawn({
        let handle = handle.clone();
        async move { handle.spawn_chunks(0).await.unwrap() }
    });

    tokio::time::sleep(Duration::from_millis(20)).await;
    assert_eq!(handle.apply("during-load").await.unwrap(), 1);
    assert_eq!(loading.await.unwrap().len(), 1);
    cleanup(root);
}

#[tokio::test]
async fn save_failure_rolls_back_tentative_mutation() {
    let root = temp_root();
    let storage = WorldStorage::with_save_failure_for_tests(&root);
    let handle = RegionActor::spawn_persistent(RegionId(3), storage);
    let pos = BlockPos::new(0, 80, 0);
    handle.spawn_chunks(0).await.unwrap();

    let mutation = handle.set_block(pos, BlockState::Stone).await.unwrap();

    assert_eq!(mutation.state, BlockState::Air);
    assert!(!mutation.accepted());
    assert_eq!(handle.get_block(pos).await.unwrap(), Some(BlockState::Air));
    cleanup(root);
}

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("lkjmcrs-region-{nanos}"))
}

fn cleanup(root: PathBuf) {
    let _ = fs::remove_dir_all(root);
}
