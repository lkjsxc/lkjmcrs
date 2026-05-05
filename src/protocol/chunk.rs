use crate::protocol::chunk_palette;
use crate::protocol::codec;
use crate::world::blocks::MIN_Y;
use crate::world::{BlockState, ChunkSnapshot};

pub(super) const SECTION_COUNT: usize = 24;
const LIGHT_SECTION_COUNT: usize = SECTION_COUNT + 2;
const FULL_LIGHT: [u8; 2048] = [0xff; 2048];

const AIR_ID: i32 = 0;
const STONE_ID: i32 = 1;
const GRASS_BLOCK_ID: i32 = 9;
const DIRT_ID: i32 = 10;
const BEDROCK_ID: i32 = 85;

pub fn encode_level_chunk_with_light(chunk: &ChunkSnapshot) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i32(&mut out, chunk.pos.x);
    codec::write_i32(&mut out, chunk.pos.z);
    write_heightmaps(&mut out);
    let data = encode_chunk_data(chunk);
    codec::write_var_i32(&mut out, data.len() as i32);
    out.extend_from_slice(&data);
    codec::write_var_i32(&mut out, 0);
    write_light_data(&mut out);
    out
}

pub fn encode_update_light(chunk: &ChunkSnapshot) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, chunk.pos.x);
    codec::write_var_i32(&mut out, chunk.pos.z);
    write_light_data(&mut out);
    out
}

pub fn encode_chunk_batch_finished(size: usize) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, size as i32);
    out
}

fn encode_chunk_data(chunk: &ChunkSnapshot) -> Vec<u8> {
    let mut out = Vec::new();
    for section in 0..SECTION_COUNT {
        let min_y = MIN_Y + (section as i32 * 16);
        encode_section(&mut out, chunk, min_y);
    }
    out
}

fn encode_section(out: &mut Vec<u8>, chunk: &ChunkSnapshot, min_y: i32) {
    let states = section_states(chunk, min_y);
    let block_count = states.iter().filter(|state| **state != AIR_ID).count();
    codec::write_u16(out, block_count as u16);
    chunk_palette::write_block_states(out, &states);
    chunk_palette::write_single_value(out, 0);
}

fn section_states(chunk: &ChunkSnapshot, min_y: i32) -> Vec<i32> {
    let mut states = Vec::with_capacity(4096);
    for y in 0..16 {
        let state = block_state_id(chunk.block_at(min_y + y));
        states.extend(std::iter::repeat_n(state, 256));
    }
    states
}

fn write_heightmaps(out: &mut Vec<u8>) {
    codec::write_var_i32(out, 2);
    write_heightmap(out, 1);
    write_heightmap(out, 4);
}

fn write_heightmap(out: &mut Vec<u8>, ty: i32) {
    codec::write_var_i32(out, ty);
    let longs = chunk_palette::fixed_packed_longs(std::iter::repeat_n(80_u64, 256), 9, 256);
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

fn block_state_id(state: BlockState) -> i32 {
    match state {
        BlockState::Air => AIR_ID,
        BlockState::Bedrock => BEDROCK_ID,
        BlockState::Stone => STONE_ID,
        BlockState::Dirt => DIRT_ID,
        BlockState::GrassBlock => GRASS_BLOCK_ID,
    }
}

#[cfg(test)]
mod tests {
    use super::{BEDROCK_ID, DIRT_ID, GRASS_BLOCK_ID, STONE_ID, block_state_id};
    use crate::protocol::chunk::{encode_level_chunk_with_light, encode_update_light};
    use crate::world::{BlockState, ChunkPos, ChunkSnapshot};

    #[test]
    fn flat_block_state_ids_match_minecraft_data_defaults() {
        assert_eq!(block_state_id(BlockState::Stone), STONE_ID);
        assert_eq!(block_state_id(BlockState::GrassBlock), GRASS_BLOCK_ID);
        assert_eq!(block_state_id(BlockState::Dirt), DIRT_ID);
        assert_eq!(block_state_id(BlockState::Bedrock), BEDROCK_ID);
    }

    #[test]
    fn chunk_and_light_packets_are_non_empty() {
        let chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
        assert!(encode_level_chunk_with_light(&chunk).len() > 4096);
        assert!(encode_update_light(&chunk).len() > 4096);
    }
}
