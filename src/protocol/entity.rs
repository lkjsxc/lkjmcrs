use crate::protocol::{codec, inventory};
use uuid::Uuid;

pub const ITEM_ENTITY_TYPE_ID: i32 = 71;
const ITEM_METADATA_INDEX: u8 = 8;
const ITEM_STACK_METADATA_TYPE: i32 = 7;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntitySpawn {
    pub entity_id: i32,
    pub uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemMetadata<'a> {
    pub entity_id: i32,
    pub item_id: &'a str,
    pub count: u8,
}

pub fn encode_spawn_entity(entity: EntitySpawn) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, entity.entity_id);
    codec::write_uuid(&mut out, entity.uuid);
    codec::write_var_i32(&mut out, ITEM_ENTITY_TYPE_ID);
    codec::write_f64(&mut out, entity.x);
    codec::write_f64(&mut out, entity.y);
    codec::write_f64(&mut out, entity.z);
    codec::write_u8(&mut out, 0);
    codec::write_i8(&mut out, 0);
    codec::write_i8(&mut out, 0);
    codec::write_i8(&mut out, 0);
    codec::write_var_i32(&mut out, 0);
    out
}

pub fn encode_item_metadata(entity: ItemMetadata<'_>) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, entity.entity_id);
    codec::write_u8(&mut out, ITEM_METADATA_INDEX);
    codec::write_var_i32(&mut out, ITEM_STACK_METADATA_TYPE);
    let slot = inventory::Slot {
        item_id: entity.item_id,
        count: entity.count,
    };
    inventory::encode_slot(&mut out, Some(slot));
    codec::write_u8(&mut out, 0xff);
    out
}

pub fn encode_collect(collected: i32, collector: i32, count: u8) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, collected);
    codec::write_var_i32(&mut out, collector);
    codec::write_var_i32(&mut out, i32::from(count));
    out
}

pub fn encode_destroy(entity_ids: &[i32]) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, entity_ids.len() as i32);
    for id in entity_ids {
        codec::write_var_i32(&mut out, *id);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        EntitySpawn, ITEM_ENTITY_TYPE_ID, ItemMetadata, encode_collect, encode_destroy,
        encode_item_metadata,
    };
    use crate::protocol::codec;
    use std::io::{Cursor, Read};
    use uuid::Uuid;

    #[test]
    fn spawn_entity_uses_protocol_774_zero_velocity_tail() {
        let entity = EntitySpawn {
            entity_id: 1000,
            uuid: Uuid::from_u128(1),
            x: 1.5,
            y: 79.5,
            z: 0.5,
        };
        let payload = super::encode_spawn_entity(entity);
        let mut cursor = Cursor::new(payload);
        assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 1000);
        let _uuid = codec::read_uuid(&mut cursor).unwrap();
        assert_eq!(
            codec::read_var_i32(&mut cursor).unwrap(),
            ITEM_ENTITY_TYPE_ID
        );
        for _ in 0..3 {
            let _ = codec::read_f64(&mut cursor).unwrap();
        }
        let mut tail = Vec::new();
        cursor.read_to_end(&mut tail).unwrap();
        assert_eq!(tail, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn item_metadata_uses_slot_payload_and_terminator() {
        let entity = ItemMetadata {
            entity_id: 1000,
            item_id: "minecraft:dirt",
            count: 1,
        };
        let payload = encode_item_metadata(entity);
        assert_eq!(&payload[payload.len() - 5..], &[1, 28, 0, 0, 0xff]);
    }

    #[test]
    fn collect_and_destroy_are_varint_payloads() {
        assert_eq!(encode_collect(1000, 1, 1), vec![0xe8, 0x07, 1, 1]);
        assert_eq!(encode_destroy(&[1000]), vec![1, 0xe8, 0x07]);
    }
}
