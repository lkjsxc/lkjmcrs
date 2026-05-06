use crate::world::{BlockPos, ChunkPos};
use uuid::Uuid;

pub const PICKUP_RADIUS: f64 = 1.5;

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
        }
    }

    pub fn within_pickup_radius(&self, x: f64, y: f64, z: f64) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        let dz = self.z - z;
        dx * dx + dy * dy + dz * dz <= PICKUP_RADIUS * PICKUP_RADIUS
    }
}
