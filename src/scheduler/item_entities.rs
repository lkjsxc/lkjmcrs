use crate::scheduler::region_state::RegionActor;
use crate::world::{BlockPos, ChunkPos, DroppedItemEntity};
use std::collections::HashSet;
use std::time::Instant;
use tokio::sync::oneshot;

const FIRST_ITEM_ENTITY_ID: i32 = 1000;

impl RegionActor {
    pub(super) fn spawn_item(
        &mut self,
        pos: BlockPos,
        item_id: String,
        count: u8,
        reply: oneshot::Sender<DroppedItemEntity>,
    ) {
        let entity_id = self.next_item_entity_id;
        self.next_item_entity_id += 1;
        let entity = DroppedItemEntity::new(entity_id, pos, item_id, count);
        self.item_entities.insert(entity_id, entity.clone());
        let _ = reply.send(entity);
    }

    pub(super) fn items_in_chunks(
        &self,
        chunks: Vec<ChunkPos>,
        reply: oneshot::Sender<Vec<DroppedItemEntity>>,
    ) {
        let chunks: HashSet<_> = chunks.into_iter().collect();
        let items = self
            .item_entities
            .values()
            .filter(|entity| chunks.contains(&entity.chunk))
            .cloned()
            .collect();
        let _ = reply.send(items);
    }

    pub(super) fn collect_nearby(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        accepted_items: Vec<String>,
        reply: oneshot::Sender<Option<DroppedItemEntity>>,
    ) {
        let now = Instant::now();
        let Some(entity_id) = self
            .item_entities
            .values()
            .find(|entity| {
                accepted_items.iter().any(|item| item == &entity.item_id)
                    && entity.can_pickup_at(x, y, z, now)
            })
            .map(|entity| entity.entity_id)
        else {
            let _ = reply.send(None);
            return;
        };
        let _ = reply.send(self.item_entities.remove(&entity_id));
    }

    pub(super) const fn first_item_entity_id() -> i32 {
        FIRST_ITEM_ENTITY_ID
    }
}
