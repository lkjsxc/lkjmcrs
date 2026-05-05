use crate::scheduler::RegionActor;
use crate::world::{BlockPos, BlockState, ChunkPos, RegionId};

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
