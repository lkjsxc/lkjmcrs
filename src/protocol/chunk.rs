use crate::protocol::chunk_palette;
use crate::protocol::codec;

pub const SECTION_COUNT: usize = 24;
pub const MIN_Y: i32 = -64;
const LIGHT_SECTION_COUNT: usize = SECTION_COUNT + 2;
const FULL_LIGHT: [u8; 2048] = [0xff; 2048];

pub const AIR_ID: i32 = 0;
pub const STONE_ID: i32 = 1;
pub const GRASS_BLOCK_ID: i32 = 9;
pub const DIRT_ID: i32 = 10;
pub const BEDROCK_ID: i32 = 85;
pub const WATER_ID: i32 = 86;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

pub trait ChunkColumn {
    fn position(&self) -> ChunkPosition;
    fn block_state_id_at_local(&self, x: usize, y: i32, z: usize) -> i32;
    fn heightmap_at_local(&self, x: usize, z: usize) -> u16;
}

pub fn encode_level_chunk_with_light(chunk: &impl ChunkColumn) -> Vec<u8> {
    let mut out = Vec::new();
    let pos = chunk.position();
    codec::write_i32(&mut out, pos.x);
    codec::write_i32(&mut out, pos.z);
    out.extend_from_slice(&encode_level_chunk_body_with_light(chunk));
    out
}

pub fn encode_level_chunk_body_with_light(chunk: &impl ChunkColumn) -> Vec<u8> {
    let mut out = Vec::new();
    write_heightmaps(&mut out, chunk);
    let data = encode_chunk_data(chunk);
    codec::write_var_i32(&mut out, data.len() as i32);
    out.extend_from_slice(&data);
    codec::write_var_i32(&mut out, 0);
    write_light_data(&mut out);
    out
}

pub fn encode_update_light(chunk: &impl ChunkColumn) -> Vec<u8> {
    let mut out = Vec::new();
    let pos = chunk.position();
    codec::write_var_i32(&mut out, pos.x);
    codec::write_var_i32(&mut out, pos.z);
    write_light_data(&mut out);
    out
}

pub fn encode_chunk_batch_finished(size: usize) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, size as i32);
    out
}

pub fn encode_unload_chunk(pos: ChunkPosition) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i32(&mut out, pos.z);
    codec::write_i32(&mut out, pos.x);
    out
}

fn encode_chunk_data(chunk: &impl ChunkColumn) -> Vec<u8> {
    let mut out = Vec::new();
    for section in 0..SECTION_COUNT {
        let min_y = MIN_Y + (section as i32 * 16);
        encode_section(&mut out, chunk, min_y);
    }
    out
}

fn encode_section(out: &mut Vec<u8>, chunk: &impl ChunkColumn, min_y: i32) {
    let states = section_states(chunk, min_y);
    let block_count = states.iter().filter(|state| **state != AIR_ID).count();
    codec::write_u16(out, block_count as u16);
    chunk_palette::write_block_states(out, &states);
    chunk_palette::write_single_value(out, 0);
}

fn section_states(chunk: &impl ChunkColumn, min_y: i32) -> Vec<i32> {
    let mut states = Vec::with_capacity(4096);
    for y in 0..16 {
        for z in 0..16 {
            for x in 0..16 {
                states.push(chunk.block_state_id_at_local(x, min_y + y, z));
            }
        }
    }
    states
}

fn write_heightmaps(out: &mut Vec<u8>, chunk: &impl ChunkColumn) {
    codec::write_var_i32(out, 2);
    write_heightmap(out, 1, chunk);
    write_heightmap(out, 4, chunk);
}

fn write_heightmap(out: &mut Vec<u8>, ty: i32, chunk: &impl ChunkColumn) {
    codec::write_var_i32(out, ty);
    let values =
        (0..16).flat_map(|z| (0..16).map(move |x| u64::from(chunk.heightmap_at_local(x, z))));
    let longs = chunk_palette::fixed_packed_longs(values, 9, 256);
    codec::write_var_i32(out, longs.len() as i32);
    for value in longs {
        codec::write_i64(out, value as i64);
    }
}

fn write_light_data(out: &mut Vec<u8>) {
    let mask = (1_i64 << LIGHT_SECTION_COUNT) - 1;
    write_bitset(out, mask);
    write_bitset(out, 0);
    write_bitset(out, 0);
    write_bitset(out, mask);
    codec::write_var_i32(out, LIGHT_SECTION_COUNT as i32);
    for _ in 0..LIGHT_SECTION_COUNT {
        codec::write_var_i32(out, FULL_LIGHT.len() as i32);
        out.extend_from_slice(&FULL_LIGHT);
    }
    codec::write_var_i32(out, 0);
}

fn write_bitset(out: &mut Vec<u8>, mask: i64) {
    codec::write_var_i32(out, 1);
    codec::write_i64(out, mask);
}
