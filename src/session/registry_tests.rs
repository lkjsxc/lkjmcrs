use crate::session::outbound::PlayOutbound;
use crate::session::registry::SessionRegistry;
use crate::world::{BlockPos, BlockState, ChunkPos};

#[tokio::test]
async fn broadcasts_only_to_subscribed_sessions() {
    let registry = SessionRegistry::default();
    let (subscribed, mut subscribed_rx) = registry.register().await;
    let (_other, mut other_rx) = registry.register().await;
    registry
        .subscribe(subscribed, [ChunkPos::new(0, 0), ChunkPos::new(1, 0)])
        .await;

    let sent = registry
        .broadcast_block_update(
            ChunkPos::new(0, 0),
            BlockPos::new(0, 80, 0),
            BlockState::Stone,
        )
        .await;

    assert_eq!(sent, 1);
    assert_eq!(
        subscribed_rx.try_recv().unwrap(),
        PlayOutbound::BlockUpdate {
            pos: BlockPos::new(0, 80, 0),
            state: BlockState::Stone,
        }
    );
    assert!(other_rx.try_recv().is_err());
}

#[tokio::test]
async fn unregister_removes_subscriptions() {
    let registry = SessionRegistry::default();
    let (id, _rx) = registry.register().await;
    registry.subscribe(id, [ChunkPos::new(0, 0)]).await;
    registry.unregister(id).await;

    let sent = registry
        .broadcast_block_update(
            ChunkPos::new(0, 0),
            BlockPos::new(0, 80, 0),
            BlockState::Stone,
        )
        .await;

    assert_eq!(sent, 0);
}

#[tokio::test]
async fn newly_subscribed_chunks_are_fanout_eligible() {
    let registry = SessionRegistry::default();
    let (id, mut rx) = registry.register().await;
    registry.subscribe(id, [ChunkPos::new(0, 0)]).await;
    registry.subscribe(id, [ChunkPos::new(3, 0)]).await;

    let sent = registry
        .broadcast_block_update(
            ChunkPos::new(3, 0),
            BlockPos::new(48, 80, 0),
            BlockState::Stone,
        )
        .await;

    assert_eq!(sent, 1);
    assert_eq!(
        rx.try_recv().unwrap(),
        PlayOutbound::BlockUpdate {
            pos: BlockPos::new(48, 80, 0),
            state: BlockState::Stone,
        }
    );
}
