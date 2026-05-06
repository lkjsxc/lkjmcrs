use crate::protocol::{entity, ids};
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::world::DroppedItemEntity;
use tokio::io::AsyncWrite;

pub async fn send_item_spawn<W>(
    writer: &mut W,
    phase: SessionState,
    item: &DroppedItemEntity,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::SPAWN_ENTITY,
        &entity::encode_spawn_entity(item),
    )
    .await?;
    write_packet(
        writer,
        phase,
        ids::play::ENTITY_METADATA,
        &entity::encode_item_metadata(item),
    )
    .await
}

pub async fn send_collect<W>(
    writer: &mut W,
    phase: SessionState,
    item: &DroppedItemEntity,
    collector: i32,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::COLLECT,
        &entity::encode_collect(item.entity_id, collector, item.count),
    )
    .await
}

pub async fn send_destroy<W>(
    writer: &mut W,
    phase: SessionState,
    entity_id: i32,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::ENTITY_DESTROY,
        &entity::encode_destroy(&[entity_id]),
    )
    .await
}
