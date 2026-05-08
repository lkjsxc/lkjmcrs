use crate::protocol::chunk::{
    AIR_ID, BEDROCK_ID, ChunkColumn, ChunkPosition, DIRT_ID, GRASS_BLOCK_ID, STONE_ID,
    encode_level_chunk_with_light, encode_update_light,
};
use crate::protocol::codec;
use std::io::Cursor;

struct FlatChunk;

impl ChunkColumn for FlatChunk {
    fn position(&self) -> ChunkPosition {
        ChunkPosition { x: 0, z: 0 }
    }

    fn block_state_id_at_local(&self, _x: usize, y: i32, _z: usize) -> i32 {
        match y {
            0 => BEDROCK_ID,
            1..=62 => STONE_ID,
            63..=78 => DIRT_ID,
            79 => GRASS_BLOCK_ID,
            _ => AIR_ID,
        }
    }

    fn heightmap_at_local(&self, _x: usize, _z: usize) -> u16 {
        80
    }
}

struct SlopedChunk;

impl ChunkColumn for SlopedChunk {
    fn position(&self) -> ChunkPosition {
        ChunkPosition { x: 0, z: 0 }
    }

    fn block_state_id_at_local(&self, _x: usize, y: i32, _z: usize) -> i32 {
        if y < 96 { STONE_ID } else { AIR_ID }
    }

    fn heightmap_at_local(&self, x: usize, z: usize) -> u16 {
        80 + ((x + z) % 7) as u16
    }
}

#[test]
fn exposes_minecraft_data_default_ids() {
    assert_eq!(STONE_ID, 1);
    assert_eq!(GRASS_BLOCK_ID, 9);
    assert_eq!(DIRT_ID, 10);
    assert_eq!(BEDROCK_ID, 85);
}

#[test]
fn chunk_and_light_packets_are_non_empty() {
    assert!(encode_level_chunk_with_light(&FlatChunk).len() > 4096);
    assert!(encode_update_light(&FlatChunk).len() > 4096);
}

#[test]
fn heightmaps_follow_column_heights() {
    let flat = decode_first_heightmap(&encode_level_chunk_with_light(&FlatChunk));
    assert!(flat.iter().all(|value| *value == 80));

    let sloped = decode_first_heightmap(&encode_level_chunk_with_light(&SlopedChunk));
    assert_eq!(sloped[0], 80);
    assert_eq!(sloped[1], 81);
    assert!(sloped.iter().any(|value| *value != 80));
}

fn decode_first_heightmap(data: &[u8]) -> Vec<u16> {
    let mut cursor = Cursor::new(data.to_vec());
    cursor.set_position(8);
    assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 2);
    let _kind = codec::read_var_i32(&mut cursor).unwrap();
    let longs = (0..codec::read_var_i32(&mut cursor).unwrap())
        .map(|_| codec::read_i64(&mut cursor).unwrap() as u64)
        .collect::<Vec<_>>();
    unpack_fixed(&longs, 9, 256)
}

fn unpack_fixed(longs: &[u64], bits: u8, count: usize) -> Vec<u16> {
    let per_long = 64 / bits as usize;
    let mask = (1_u64 << bits) - 1;
    (0..count)
        .map(|index| {
            ((longs[index / per_long] >> ((index % per_long) * bits as usize)) & mask) as u16
        })
        .collect()
}
