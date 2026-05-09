use crate::world::{BlockPos, ChunkPos};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub const PLAYER_HALF_WIDTH: f64 = 0.3;
pub const PLAYER_HEIGHT: f64 = 1.8;
pub const ITEM_HALF_WIDTH: f64 = 0.125;
pub const ITEM_HEIGHT: f64 = 0.25;
pub const PICKUP_EXPAND: f64 = 0.25;
pub const PICKUP_DELAY: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq)]
pub struct DroppedItemEntity {
    pub entity_id: i32,
    pub uuid: Uuid,
    pub chunk: ChunkPos,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub item_id: String,
    pub count: u8,
    pub pickup_ready_at: Instant,
}

impl DroppedItemEntity {
    pub fn new(entity_id: i32, pos: BlockPos, item_id: impl Into<String>, count: u8) -> Self {
        let x = f64::from(pos.x) + 0.5;
        let y = f64::from(pos.y) + 0.5;
        let z = f64::from(pos.z) + 0.5;
        Self {
            entity_id,
            uuid: Uuid::from_u128(
                0x6c_6b_6a_6d_63_72_73_00_00_00_00_00_00_00_00_00 + entity_id as u128,
            ),
            chunk: pos.chunk(),
            x,
            y,
            z,
            item_id: item_id.into(),
            count,
            pickup_ready_at: Instant::now() + PICKUP_DELAY,
        }
    }

    pub fn can_pickup_at(&self, x: f64, y: f64, z: f64, now: Instant) -> bool {
        now >= self.pickup_ready_at && pickup_box(x, y, z).intersects(item_box(self))
    }

    #[cfg(test)]
    pub fn ready_for_tests(mut self) -> Self {
        self.pickup_ready_at = Instant::now();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Aabb {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    min_z: f64,
    max_z: f64,
}

impl Aabb {
    fn intersects(self, other: Self) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
            && self.min_z <= other.max_z
            && self.max_z >= other.min_z
    }
}

fn pickup_box(x: f64, y: f64, z: f64) -> Aabb {
    Aabb {
        min_x: x - PLAYER_HALF_WIDTH - PICKUP_EXPAND,
        max_x: x + PLAYER_HALF_WIDTH + PICKUP_EXPAND,
        min_y: y - PICKUP_EXPAND,
        max_y: y + PLAYER_HEIGHT + PICKUP_EXPAND,
        min_z: z - PLAYER_HALF_WIDTH - PICKUP_EXPAND,
        max_z: z + PLAYER_HALF_WIDTH + PICKUP_EXPAND,
    }
}

fn item_box(item: &DroppedItemEntity) -> Aabb {
    Aabb {
        min_x: item.x - ITEM_HALF_WIDTH,
        max_x: item.x + ITEM_HALF_WIDTH,
        min_y: item.y,
        max_y: item.y + ITEM_HEIGHT,
        min_z: item.z - ITEM_HALF_WIDTH,
        max_z: item.z + ITEM_HALF_WIDTH,
    }
}

#[cfg(test)]
mod tests {
    use super::DroppedItemEntity;
    use crate::world::BlockPos;
    use std::time::Instant;

    #[test]
    fn pickup_requires_delay() {
        let item = DroppedItemEntity::new(1000, BlockPos::new(0, 79, 0), "minecraft:dirt", 1);
        assert!(!item.can_pickup_at(0.5, 80.0, 0.5, Instant::now()));
    }

    #[test]
    fn pickup_uses_expanded_player_box() {
        let item = DroppedItemEntity::new(1000, BlockPos::new(0, 79, 0), "minecraft:dirt", 1)
            .ready_for_tests();
        assert!(item.can_pickup_at(0.5, 80.0, 0.5, Instant::now()));
        assert!(item.can_pickup_at(1.05, 80.0, 0.5, Instant::now()));
        assert!(!item.can_pickup_at(1.2, 80.0, 0.5, Instant::now()));
    }

    #[test]
    fn pickup_requires_vertical_overlap() {
        let item = DroppedItemEntity::new(1000, BlockPos::new(0, 79, 0), "minecraft:dirt", 1)
            .ready_for_tests();
        assert!(!item.can_pickup_at(0.5, 82.2, 0.5, Instant::now()));
        assert!(item.can_pickup_at(0.5, 79.0, 0.5, Instant::now()));
    }
}
