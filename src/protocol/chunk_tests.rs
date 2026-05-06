use crate::protocol::chunk::{SECTION_COUNT, encode_level_chunk_with_light, encode_unload_chunk};
use crate::protocol::chunk_palette::fixed_long_count;
use crate::protocol::{chunk, codec, ids};
use crate::world::{ChunkPos, ChunkSnapshot};
use std::io::Cursor;

const EXPECTED_FLAT_CHUNK_DATA_LEN: i32 = 6294;
const HISTORICAL_MALFORMED_CHUNK_DATA_LEN: i32 = 6345;

#[derive(Debug)]
struct SectionShape {
    non_air_blocks: u16,
    block_bits: u8,
    block_longs: usize,
    biome_bits: u8,
    biome_longs: usize,
}

#[test]
fn level_chunk_heightmaps_use_fixed_storage_long_counts() {
    let chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
    let packet = encode_level_chunk_with_light(&chunk);

    assert_eq!(parse_heightmap_long_counts(packet), vec![37, 37]);
}

#[test]
fn level_chunk_packet_sections_consume_exact_chunk_data() {
    let chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
    let packet = encode_level_chunk_with_light(&chunk);
    let sections = parse_level_chunk_sections(packet);

    assert_eq!(sections.len(), SECTION_COUNT);
    assert_eq!(sections[0].non_air_blocks, 0);
    assert_eq!(sections[0].block_bits, 0);
    assert_eq!(sections[0].block_longs, 0);
    assert_eq!(sections[4].non_air_blocks, 4096);
    assert_eq!(sections[4].block_bits, 4);
    assert_eq!(sections[4].block_longs, 256);
    assert_eq!(sections[5].block_bits, 0);
    assert_eq!(sections[5].block_longs, 0);
    assert_eq!(sections[8].block_bits, 4);
    assert_eq!(sections[8].block_longs, 256);
    assert!(sections.iter().all(|section| section.biome_bits == 0));
    assert!(sections.iter().all(|section| section.biome_longs == 0));
}

#[test]
fn flat_chunk_data_length_stays_on_documented_shape() {
    let chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
    let packet = encode_level_chunk_with_light(&chunk);

    assert_eq!(parse_chunk_data_len(packet), EXPECTED_FLAT_CHUNK_DATA_LEN);
    assert_ne!(
        EXPECTED_FLAT_CHUNK_DATA_LEN,
        HISTORICAL_MALFORMED_CHUNK_DATA_LEN
    );
}

#[test]
fn level_chunk_and_update_light_packets_remain_separate() {
    let chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
    assert!(encode_level_chunk_with_light(&chunk).len() > 4096);
    assert!(chunk::encode_update_light(&chunk).len() > 4096);
}

#[test]
fn unload_chunk_payload_is_z_then_x() {
    assert_eq!(ids::play::UNLOAD_CHUNK, 0x25);
    assert_eq!(
        encode_unload_chunk(ChunkPos::new(7, -3)),
        (-3_i32)
            .to_be_bytes()
            .into_iter()
            .chain(7_i32.to_be_bytes())
            .collect::<Vec<_>>()
    );
}

fn parse_level_chunk_sections(packet: Vec<u8>) -> Vec<SectionShape> {
    let mut cursor = Cursor::new(packet);
    skip_bytes(&mut cursor, 8);
    let _heightmaps = parse_heightmaps(&mut cursor);
    let chunk_data_len = codec::read_var_i32(&mut cursor).unwrap() as u64;
    let chunk_data_start = cursor.position();
    let sections = (0..SECTION_COUNT)
        .map(|_| parse_section(&mut cursor))
        .collect::<Vec<_>>();
    assert_eq!(cursor.position() - chunk_data_start, chunk_data_len);
    assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 0);
    sections
}

fn parse_heightmap_long_counts(packet: Vec<u8>) -> Vec<i32> {
    let mut cursor = Cursor::new(packet);
    skip_bytes(&mut cursor, 8);
    parse_heightmaps(&mut cursor)
}

fn parse_chunk_data_len(packet: Vec<u8>) -> i32 {
    let mut cursor = Cursor::new(packet);
    skip_bytes(&mut cursor, 8);
    let _heightmaps = parse_heightmaps(&mut cursor);
    codec::read_var_i32(&mut cursor).unwrap()
}

fn parse_heightmaps(cursor: &mut Cursor<Vec<u8>>) -> Vec<i32> {
    let heightmap_count = codec::read_var_i32(cursor).unwrap();
    let mut long_counts = Vec::new();
    for _ in 0..heightmap_count {
        let _heightmap_type = codec::read_var_i32(cursor).unwrap();
        let long_count = codec::read_var_i32(cursor).unwrap();
        long_counts.push(long_count);
        skip_bytes(cursor, long_count as u64 * 8);
    }
    long_counts
}

fn parse_section(cursor: &mut Cursor<Vec<u8>>) -> SectionShape {
    let non_air_blocks = read_u16(cursor);
    let (block_bits, block_longs) = parse_container(cursor, 4096);
    let (biome_bits, biome_longs) = parse_container(cursor, 64);
    SectionShape {
        non_air_blocks,
        block_bits,
        block_longs,
        biome_bits,
        biome_longs,
    }
}

fn parse_container(cursor: &mut Cursor<Vec<u8>>, entry_count: usize) -> (u8, usize) {
    let bits = read_u8(cursor);
    if bits == 0 {
        let _single_value = codec::read_var_i32(cursor).unwrap();
        return (bits, 0);
    }
    let palette_len = codec::read_var_i32(cursor).unwrap();
    for _ in 0..palette_len {
        let _palette_value = codec::read_var_i32(cursor).unwrap();
    }
    let long_count = fixed_long_count(entry_count, bits);
    skip_bytes(cursor, long_count as u64 * 8);
    (bits, long_count)
}

fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> u8 {
    let position = cursor.position() as usize;
    let value = cursor.get_ref()[position];
    cursor.set_position(position as u64 + 1);
    value
}

fn read_u16(cursor: &mut Cursor<Vec<u8>>) -> u16 {
    let high = read_u8(cursor) as u16;
    let low = read_u8(cursor) as u16;
    (high << 8) | low
}

fn skip_bytes(cursor: &mut Cursor<Vec<u8>>, count: u64) {
    cursor.set_position(cursor.position() + count);
}
