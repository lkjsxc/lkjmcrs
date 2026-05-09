use crate::probe::ProbeError;
use crate::protocol::codec;
use std::collections::HashMap;
use std::io::{Cursor, Read};

const AIR: i32 = 0;
type ErrorBox = Box<dyn std::error::Error>;

pub(super) struct DecodedChunk {
    blocks: HashMap<(usize, i32, usize), i32>,
}

impl DecodedChunk {
    pub(super) fn from_packet(data: Vec<u8>) -> Result<Self, ErrorBox> {
        let mut cursor = Cursor::new(data);
        let x = read_i32(&mut cursor)?;
        let z = read_i32(&mut cursor)?;
        skip_heightmaps(&mut cursor)?;
        let len = codec::read_var_i32(&mut cursor)?;
        let end = cursor.position() + len as u64;
        let blocks = read_sections(&mut cursor)?;
        if cursor.position() != end {
            return Err(Box::new(ProbeError::Phase("terrain chunk data boundary")));
        }
        let _chunk_pos = (x, z);
        Ok(Self { blocks })
    }

    pub(super) fn has_non_flat_surface(&self) -> bool {
        (0..16).any(|z| (0..16).any(|x| self.surface_y(x, z) != Some(79)))
    }

    pub(super) fn contains_state(&self, state: i32) -> bool {
        self.blocks.values().any(|value| *value == state)
    }

    pub(super) fn has_enclosed_underground_air(&self) -> bool {
        (0..16).any(|z| {
            (0..16).any(|x| {
                let Some(surface_y) = self.surface_y(x, z) else {
                    return false;
                };
                if surface_y < 17 {
                    return false;
                }
                (8..=surface_y - 8).any(|y| {
                    self.state(x, y, z) == AIR
                        && self.has_solid_between(x, y + 1, surface_y, z)
                        && self.has_solid_between(x, 8, y - 1, z)
                })
            })
        })
    }

    pub(super) fn surface_y(&self, x: usize, z: usize) -> Option<i32> {
        (-64..320).rev().find(|y| self.state(x, *y, z) != AIR)
    }

    fn state(&self, x: usize, y: i32, z: usize) -> i32 {
        self.blocks.get(&(x, y, z)).copied().unwrap_or(AIR)
    }

    fn has_solid_between(&self, x: usize, min_y: i32, max_y: i32, z: usize) -> bool {
        min_y <= max_y && (min_y..=max_y).any(|y| self.state(x, y, z) != AIR)
    }
}

fn read_sections(
    cursor: &mut Cursor<Vec<u8>>,
) -> Result<HashMap<(usize, i32, usize), i32>, ErrorBox> {
    let mut blocks = HashMap::new();
    for section in 0..crate::protocol::chunk::SECTION_COUNT {
        let min_y = crate::protocol::chunk::MIN_Y + section as i32 * 16;
        let _non_air = codec::read_u16(cursor)?;
        let states = read_container(cursor, 4096)?;
        read_container(cursor, 64)?;
        for (index, state) in states.into_iter().enumerate().filter(|(_, s)| *s != AIR) {
            let y = min_y + (index / 256) as i32;
            let z = (index % 256) / 16;
            let x = index % 16;
            blocks.insert((x, y, z), state);
        }
    }
    Ok(blocks)
}

fn read_container(cursor: &mut Cursor<Vec<u8>>, count: usize) -> Result<Vec<i32>, ErrorBox> {
    let bits = read_u8(cursor)?;
    if bits == 0 {
        let value = codec::read_var_i32(cursor)?;
        return Ok(vec![value; count]);
    }
    let palette_len = codec::read_var_i32(cursor)?;
    let mut palette = Vec::new();
    for _ in 0..palette_len {
        palette.push(codec::read_var_i32(cursor)?);
    }
    let longs = (0..count.div_ceil(64 / bits as usize))
        .map(|_| codec::read_i64(cursor).map(|v| v as u64))
        .collect::<Result<Vec<_>, _>>()?;
    decode_indexes(&palette, &longs, bits, count)
}

fn decode_indexes(
    palette: &[i32],
    longs: &[u64],
    bits: u8,
    count: usize,
) -> Result<Vec<i32>, ErrorBox> {
    let per_long = 64 / bits as usize;
    let mask = (1_u64 << bits) - 1;
    (0..count)
        .map(|index| {
            let palette_index =
                ((longs[index / per_long] >> ((index % per_long) * bits as usize)) & mask) as usize;
            palette
                .get(palette_index)
                .copied()
                .ok_or_else(|| Box::new(ProbeError::Phase("terrain palette index")).into())
        })
        .collect()
}

fn skip_heightmaps(cursor: &mut Cursor<Vec<u8>>) -> Result<(), ErrorBox> {
    for _ in 0..codec::read_var_i32(cursor)? {
        let _kind = codec::read_var_i32(cursor)?;
        let longs = codec::read_var_i32(cursor)?;
        skip(cursor, longs as usize * 8)?;
    }
    Ok(())
}

fn skip(cursor: &mut Cursor<Vec<u8>>, count: usize) -> Result<(), ErrorBox> {
    let next = cursor.position() + count as u64;
    if next > cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("terrain packet eof")));
    }
    cursor.set_position(next);
    Ok(())
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
