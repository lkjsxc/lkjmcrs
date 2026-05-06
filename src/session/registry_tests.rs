use crate::player::PlayerProfile;
use crate::session::outbound::PlayOutbound;
use crate::session::registry::SessionRegistry;
use crate::world::{BlockPos, BlockState, ChunkPos};
use uuid::Uuid;

#[tokio::test]
async fn broadcasts_only_to_subscribed_sessions() {
    let registry = SessionRegistry::default();
    let sub_profile = profile("Sub");
    let other_profile = profile("Other");
    let (subscribed, mut subscribed_rx) = registry.register(&sub_profile).await;
    let (_other, mut other_rx) = registry.register(&other_profile).await;
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
    let gone = profile("Gone");
    let (id, _rx) = registry.register(&gone).await;
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
    let mover = profile("Mover");
    let (id, mut rx) = registry.register(&mover).await;
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

#[tokio::test]
async fn unsubscribe_removes_block_update_fanout_eligibility() {
    let registry = SessionRegistry::default();
    let mover = profile("Mover");
    let (id, mut rx) = registry.register(&mover).await;
    registry.subscribe(id, [ChunkPos::new(0, 0)]).await;
    registry.unsubscribe(id, [ChunkPos::new(0, 0)]).await;

    let sent = registry
        .broadcast_block_update(
            ChunkPos::new(0, 0),
            BlockPos::new(0, 80, 0),
            BlockState::Stone,
        )
        .await;

    assert_eq!(sent, 0);
    assert!(rx.try_recv().is_err());
}

fn profile(name: &str) -> PlayerProfile {
    PlayerProfile::new(Uuid::from_u128(name.len() as u128), name)
}
