use crate::probe::ProbeError;
use crate::protocol::{chunk, codec};
use std::io::{Cursor, Read};

const HEIGHTMAP_COUNT: i32 = 2;
const HEIGHTMAP_LONG_COUNT: i32 = 37;
const BLOCK_ENTRY_COUNT: usize = 4096;
const BIOME_ENTRY_COUNT: usize = 64;
const LIGHT_SECTION_COUNT: usize = chunk::SECTION_COUNT + 2;
const LIGHT_ARRAY_BYTES: usize = 2048;

pub(super) fn validate_level_chunk_with_light(
    data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let _chunk_x = read_i32(&mut cursor)?;
    let _chunk_z = read_i32(&mut cursor)?;
    validate_heightmaps(&mut cursor)?;
    validate_chunk_data(&mut cursor)?;
    if codec::read_var_i32(&mut cursor)? != 0 {
        return Err(Box::new(ProbeError::Phase("chunk block entities")));
    }
    validate_light_data(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("chunk trailing bytes")));
    }
    Ok(())
}

pub(super) fn level_chunk_pos(data: &[u8]) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    if data.len() < 8 {
        return Err(Box::new(ProbeError::Phase("chunk position")));
    }
    let x = i32::from_be_bytes(data[0..4].try_into()?);
    let z = i32::from_be_bytes(data[4..8].try_into()?);
    Ok((x, z))
}

fn validate_heightmaps(cursor: &mut Cursor<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    if codec::read_var_i32(cursor)? != HEIGHTMAP_COUNT {
        return Err(Box::new(ProbeError::Phase("heightmap count")));
    }
    for _ in 0..HEIGHTMAP_COUNT {
        let _heightmap_type = codec::read_var_i32(cursor)?;
        if codec::read_var_i32(cursor)? != HEIGHTMAP_LONG_COUNT {
            return Err(Box::new(ProbeError::Phase("heightmap long count")));
        }
        skip_bytes(cursor, HEIGHTMAP_LONG_COUNT as usize * 8)?;
    }
    Ok(())
}

fn validate_chunk_data(cursor: &mut Cursor<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    let chunk_data_len = codec::read_var_i32(cursor)?;
    if chunk_data_len < 0 {
        return Err(Box::new(ProbeError::Phase("negative chunk data length")));
    }
    let chunk_data_end = cursor.position() + chunk_data_len as u64;
    for _ in 0..chunk::SECTION_COUNT {
        validate_section(cursor)?;
    }
    if cursor.position() != chunk_data_end {
        return Err(Box::new(ProbeError::Phase("chunk data boundary")));
    }
    Ok(())
}

fn validate_section(cursor: &mut Cursor<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    let _non_air_blocks = codec::read_u16(cursor)?;
    let block_bits = validate_container(cursor, BLOCK_ENTRY_COUNT)?;
    if block_bits != 0 && block_bits != 4 {
        return Err(Box::new(ProbeError::Phase("block palette bits")));
    }
    let biome_bits = validate_container(cursor, BIOME_ENTRY_COUNT)?;
    if biome_bits != 0 {
        return Err(Box::new(ProbeError::Phase("biome palette bits")));
    }
    Ok(())
}

fn validate_container(
    cursor: &mut Cursor<Vec<u8>>,
    entry_count: usize,
) -> Result<u8, Box<dyn std::error::Error>> {
    let bits = read_u8(cursor)?;
    if bits == 0 {
        let _single_value = codec::read_var_i32(cursor)?;
        return Ok(bits);
    }
    let palette_len = codec::read_var_i32(cursor)?;
    if palette_len <= 0 {
        return Err(Box::new(ProbeError::Phase("palette length")));
    }
    for _ in 0..palette_len {
        let _palette_value = codec::read_var_i32(cursor)?;
    }
    skip_bytes(cursor, fixed_long_count(entry_count, bits)? * 8)?;
    Ok(bits)
}

fn validate_light_data(cursor: &mut Cursor<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..4 {
        validate_bitset(cursor)?;
    }
    validate_light_arrays(cursor, LIGHT_SECTION_COUNT)?;
    validate_light_arrays(cursor, 0)
}

fn validate_bitset(cursor: &mut Cursor<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    if codec::read_var_i32(cursor)? != 1 {
        return Err(Box::new(ProbeError::Phase("light bitset length")));
    }
    let _mask = codec::read_i64(cursor)?;
    Ok(())
}

fn validate_light_arrays(
    cursor: &mut Cursor<Vec<u8>>,
    expected_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if codec::read_var_i32(cursor)? != expected_count as i32 {
        return Err(Box::new(ProbeError::Phase("light array count")));
    }
    for _ in 0..expected_count {
        if codec::read_var_i32(cursor)? != LIGHT_ARRAY_BYTES as i32 {
            return Err(Box::new(ProbeError::Phase("light array length")));
        }
        skip_bytes(cursor, LIGHT_ARRAY_BYTES)?;
    }
    Ok(())
}

fn fixed_long_count(entry_count: usize, bits: u8) -> Result<usize, Box<dyn std::error::Error>> {
    if bits > 32 {
        return Err(Box::new(ProbeError::Phase("palette bits too wide")));
    }
    Ok(entry_count.div_ceil(64 / bits as usize))
}

fn read_i32(cursor: &mut Cursor<Vec<u8>>) -> Result<i32, codec::CodecError> {
    let mut bytes = [0; 4];
    cursor
        .read_exact(&mut bytes)
        .map_err(|_| codec::CodecError::Eof)?;
    Ok(i32::from_be_bytes(bytes))
}

fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> Result<u8, codec::CodecError> {
    let mut bytes = [0; 1];
    cursor
        .read_exact(&mut bytes)
        .map_err(|_| codec::CodecError::Eof)?;
    Ok(bytes[0])
}

fn skip_bytes(
    cursor: &mut Cursor<Vec<u8>>,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let next = cursor.position() + count as u64;
    if next > cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("packet ended early")));
    }
    cursor.set_position(next);
    Ok(())
}
