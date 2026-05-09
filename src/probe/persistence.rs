use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::play_client::PlayClient;
use crate::probe::position::{BlockPos, MIN_Y};
use crate::protocol::{chunk, codec};
use std::io::{Cursor, Read};

const TARGET_STATE: i32 = 10;
const BLOCK_ENTRY_COUNT: usize = 4096;
const BIOME_ENTRY_COUNT: usize = 64;
const HEIGHTMAP_COUNT: usize = 2;
const HEIGHTMAP_LONG_BYTES: usize = 37 * 8;

pub(super) async fn place(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlayClient::connect(host, "PersistA").await?;
    block_mutation::acquire_dirt(&mut client.stream, BlockPos::new(1, 79, 0), "persist dirt")
        .await?;
    block_mutation::send_use_item_on_at(&mut client.stream, 30, BlockPos::new(3, 79, 0)).await?;
    block_mutation::expect_ack_and_update_at(
        &mut client.stream,
        30,
        BlockPos::new(3, 80, 0),
        TARGET_STATE,
    )
    .await
}

pub(super) async fn check(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    PlayClient::connect_with_block(host, "PersistB", Some(TARGET_STATE)).await?;
    Ok(())
}

pub(super) fn block_state_at(
    data: Vec<u8>,
    pos: BlockPos,
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let chunk_x = read_i32(&mut cursor)?;
    let chunk_z = read_i32(&mut cursor)?;
    skip_heightmaps(&mut cursor)?;
    let chunk_data_len = codec::read_var_i32(&mut cursor)?;
    if chunk_data_len < 0 {
        return Err(Box::new(ProbeError::Phase("persist chunk length")));
    }
    let end = cursor.position() + chunk_data_len as u64;
    let state = read_sections(&mut cursor, chunk_x, chunk_z, pos)?;
    if cursor.position() != end {
        return Err(Box::new(ProbeError::Phase("persist chunk boundary")));
    }
    Ok(state)
}

fn read_sections(
    cursor: &mut Cursor<Vec<u8>>,
    chunk_x: i32,
    chunk_z: i32,
    pos: BlockPos,
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let mut observed = None;
    for section in 0..chunk::SECTION_COUNT {
        let min_y = MIN_Y + section as i32 * 16;
        let target = target_index(chunk_x, chunk_z, min_y, pos);
        let _non_air = codec::read_u16(cursor)?;
        observed = observed.or(read_container(cursor, BLOCK_ENTRY_COUNT, target)?);
        read_container(cursor, BIOME_ENTRY_COUNT, None)?;
    }
    Ok(observed)
}

fn target_index(chunk_x: i32, chunk_z: i32, min_y: i32, pos: BlockPos) -> Option<usize> {
    if chunk_x != pos.x.div_euclid(16)
        || chunk_z != pos.z.div_euclid(16)
        || !(min_y..min_y + 16).contains(&pos.y)
    {
        return None;
    }
    let local_x = pos.x.rem_euclid(16) as usize;
    let local_y = (pos.y - min_y) as usize;
    let local_z = pos.z.rem_euclid(16) as usize;
    Some(local_y * 256 + local_z * 16 + local_x)
}

fn read_container(
    cursor: &mut Cursor<Vec<u8>>,
    count: usize,
    index: Option<usize>,
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let bits = read_u8(cursor)?;
    if bits == 0 {
        let value = codec::read_var_i32(cursor)?;
        return Ok(index.map(|_| value));
    }
    let palette_len = codec::read_var_i32(cursor)?;
    let mut palette = Vec::new();
    for _ in 0..palette_len {
        palette.push(codec::read_var_i32(cursor)?);
    }
    let longs = read_packed_longs(cursor, count, bits)?;
    index
        .map(|item| decode_index(&palette, &longs, bits, item))
        .transpose()
}

fn read_packed_longs(
    cursor: &mut Cursor<Vec<u8>>,
    count: usize,
    bits: u8,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    if bits == 0 || bits > 32 {
        return Err(Box::new(ProbeError::Phase("persist palette bits")));
    }
    let long_count = count.div_ceil(64 / bits as usize);
    let mut longs = Vec::new();
    for _ in 0..long_count {
        longs.push(codec::read_i64(cursor)? as u64);
    }
    Ok(longs)
}

fn decode_index(
    palette: &[i32],
    longs: &[u64],
    bits: u8,
    index: usize,
) -> Result<i32, Box<dyn std::error::Error>> {
    let values_per_long = 64 / bits as usize;
    let long = longs[index / values_per_long];
    let offset = (index % values_per_long) * bits as usize;
    let palette_index = ((long >> offset) & ((1_u64 << bits) - 1)) as usize;
    palette
        .get(palette_index)
        .copied()
        .ok_or_else(|| Box::new(ProbeError::Phase("persist palette index")).into())
}

fn skip_heightmaps(cursor: &mut Cursor<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    if codec::read_var_i32(cursor)? != HEIGHTMAP_COUNT as i32 {
        return Err(Box::new(ProbeError::Phase("persist heightmaps")));
    }
    for _ in 0..HEIGHTMAP_COUNT {
        let _kind = codec::read_var_i32(cursor)?;
        if codec::read_var_i32(cursor)? != 37 {
            return Err(Box::new(ProbeError::Phase("persist heightmap longs")));
        }
        skip(cursor, HEIGHTMAP_LONG_BYTES)?;
    }
    Ok(())
}

fn skip(cursor: &mut Cursor<Vec<u8>>, count: usize) -> Result<(), Box<dyn std::error::Error>> {
    let next = cursor.position() + count as u64;
    if next > cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("persist packet eof")));
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
