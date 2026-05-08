use crate::scheduler::RegionActor;
use crate::world::{BlockPos, BlockState, ChunkPos, RegionId, WorldStorage};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn applies_tasks_in_mailbox_order() {
    let handle = RegionActor::spawn(RegionId(7));
    assert_eq!(handle.id(), RegionId(7));
    assert_eq!(handle.mailbox_depth(), 0);
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
async fn loads_exact_chunk_positions() {
    let handle = RegionActor::spawn(RegionId(9));
    let chunks = handle
        .load_chunks(vec![ChunkPos::new(3, -2), ChunkPos::new(3, 2)])
        .await
        .unwrap();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].pos, ChunkPos::new(3, -2));
    assert_eq!(chunks[1].pos, ChunkPos::new(3, 2));
    assert!(
        handle
            .chunk_snapshot(ChunkPos::new(0, 0))
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn loads_generated_cached_and_persisted_chunks_in_requested_order() {
    let root = temp_root();
    let storage = WorldStorage::new(&root);
    let mut persisted = crate::world::ChunkSnapshot::flat(ChunkPos::new(6, 0));
    let persisted_block = BlockPos::new(96, 80, 0);
    persisted.set_block(persisted_block, BlockState::Stone);
    storage.save_chunk(&persisted).unwrap();
    let handle = RegionActor::spawn_persistent(RegionId(10), storage);
    handle.load_chunks(vec![ChunkPos::new(5, 0)]).await.unwrap();

    let chunks = handle
        .load_chunks(vec![
            ChunkPos::new(4, 0),
            ChunkPos::new(5, 0),
            ChunkPos::new(6, 0),
        ])
        .await
        .unwrap();

    assert_eq!(
        chunks.iter().map(|chunk| chunk.pos).collect::<Vec<_>>(),
        vec![
            ChunkPos::new(4, 0),
            ChunkPos::new(5, 0),
            ChunkPos::new(6, 0)
        ]
    );
    assert_eq!(chunks[2].block_at_pos(persisted_block), BlockState::Stone);
    cleanup(root);
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
async fn save_failure_keeps_authoritative_memory_state() {
    let root = temp_root();
    let storage = WorldStorage::with_save_failure_for_tests(&root);
    let handle = RegionActor::spawn_persistent(RegionId(3), storage);
    let pos = BlockPos::new(0, 80, 0);
    handle.spawn_chunks(0).await.unwrap();

    let mutation = handle.set_block(pos, BlockState::Stone).await.unwrap();

    assert_eq!(mutation.state, BlockState::Stone);
    assert!(mutation.accepted());
    assert_eq!(
        handle.get_block(pos).await.unwrap(),
        Some(BlockState::Stone)
    );
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

#[tokio::test]
async fn item_entities_spawn_and_collect_by_radius() {
    let handle = RegionActor::spawn(RegionId(4));
    let pos = BlockPos::new(0, 79, 0);
    let entity = handle.spawn_item(pos, "minecraft:dirt", 1).await.unwrap();

    assert_eq!(entity.entity_id, 1000);
    assert_eq!(
        handle
            .items_in_chunks(vec![ChunkPos::new(0, 0)])
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        handle
            .collect_nearby(0.5, 79.5, 0.5, vec!["minecraft:dirt".to_string()])
            .await
            .unwrap()
            .is_some()
    );
    assert!(
        handle
            .items_in_chunks(vec![ChunkPos::new(0, 0)])
            .await
            .unwrap()
            .is_empty()
    );
}
