use crate::world::blocks::MAX_Y;
use crate::world::storage_blocks::{block_state, state_code};
use crate::world::{BlockPos, ChunkPos, ChunkSnapshot, MIN_Y, WorldStorageError};

const MAGIC: &[u8; 8] = b"LKJMCRSS";
const FORMAT: u8 = 1;
const HEADER_LEN: usize = 23;
const RECORD_LEN: usize = 5;

#[derive(Debug)]
pub(super) struct StoredSection {
    chunk: ChunkPos,
    section_y: i32,
    overrides: Vec<StoredBlock>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StoredBlock {
    local_x: u8,
    local_y: u8,
    local_z: u8,
    state_code: u16,
}

impl StoredSection {
    pub(super) fn from_entries(
        chunk: ChunkPos,
        section_y: i32,
        entries: Vec<(BlockPos, crate::world::BlockState)>,
    ) -> Self {
        let mut overrides = entries
            .into_iter()
            .map(|(pos, state)| StoredBlock {
                local_x: pos.local_x() as u8,
                local_y: pos.y.rem_euclid(16) as u8,
                local_z: pos.local_z() as u8,
                state_code: state_code(state),
            })
            .collect::<Vec<_>>();
        overrides.sort_by_key(sort_key);
        Self {
            chunk,
            section_y,
            overrides,
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.overrides.is_empty()
    }

    pub(super) fn encode(&self) -> Result<Vec<u8>, WorldStorageError> {
        let count = u16::try_from(self.overrides.len())
            .map_err(|_| WorldStorageError::InvalidFormat("too many section overrides"))?;
        let mut bytes = Vec::with_capacity(HEADER_LEN + self.overrides.len() * RECORD_LEN);
        bytes.extend_from_slice(MAGIC);
        bytes.push(FORMAT);
        bytes.extend_from_slice(&self.chunk.x.to_le_bytes());
        bytes.extend_from_slice(&self.chunk.z.to_le_bytes());
        bytes.extend_from_slice(&self.section_y.to_le_bytes());
        bytes.extend_from_slice(&count.to_le_bytes());
        for block in &self.overrides {
            bytes.push(block.local_x);
            bytes.push(block.local_y);
            bytes.push(block.local_z);
            bytes.extend_from_slice(&block.state_code.to_le_bytes());
        }
        Ok(bytes)
    }

    pub(super) fn decode(bytes: &[u8]) -> Result<Self, WorldStorageError> {
        if bytes.len() < HEADER_LEN {
            return Err(WorldStorageError::InvalidFormat("truncated section header"));
        }
        if &bytes[..MAGIC.len()] != MAGIC || bytes[MAGIC.len()] != FORMAT {
            return Err(WorldStorageError::InvalidFormat("invalid section marker"));
        }
        let chunk = ChunkPos::new(read_i32(bytes, 9)?, read_i32(bytes, 13)?);
        let section_y = read_i32(bytes, 17)?;
        let count = read_u16(bytes, 21)? as usize;
        if bytes.len() != HEADER_LEN + count * RECORD_LEN {
            return Err(WorldStorageError::InvalidFormat("invalid section length"));
        }
        let mut overrides = Vec::with_capacity(count);
        let mut last = None;
        for index in 0..count {
            let offset = HEADER_LEN + index * RECORD_LEN;
            let block = StoredBlock {
                local_x: bytes[offset],
                local_y: bytes[offset + 1],
                local_z: bytes[offset + 2],
                state_code: read_u16(bytes, offset + 3)?,
            };
            validate_block(block, section_y, last)?;
            last = Some(block);
            overrides.push(block);
        }
        Ok(Self {
            chunk,
            section_y,
            overrides,
        })
    }

    pub(super) fn apply_to(&self, chunk: &mut ChunkSnapshot) -> Result<(), WorldStorageError> {
        if self.chunk != chunk.pos {
            return Err(WorldStorageError::InvalidChunkKey);
        }
        for block in &self.overrides {
            let y = self.section_y * 16 + i32::from(block.local_y);
            let pos = BlockPos::new(
                chunk.pos.x * 16 + i32::from(block.local_x),
                y,
                chunk.pos.z * 16 + i32::from(block.local_z),
            );
            let state = block_state(block.state_code)?;
            if chunk.set_block(pos, state) != Some(state) {
                return Err(WorldStorageError::InvalidBlock(pos.x, pos.y, pos.z));
            }
        }
        Ok(())
    }
}

pub(super) fn section_y(y: i32) -> i32 {
    y.div_euclid(16)
}

pub(super) fn section_range() -> std::ops::RangeInclusive<i32> {
    section_y(MIN_Y)..=section_y(MAX_Y)
}

fn validate_block(
    block: StoredBlock,
    section_y: i32,
    last: Option<StoredBlock>,
) -> Result<(), WorldStorageError> {
    let y = section_y * 16 + i32::from(block.local_y);
    if block.local_x > 15 || block.local_z > 15 || !(MIN_Y..=MAX_Y).contains(&y) {
        return Err(WorldStorageError::InvalidBlock(
            i32::from(block.local_x),
            y,
            i32::from(block.local_z),
        ));
    }
    block_state(block.state_code)?;
    if last.is_some_and(|previous| sort_key(&previous) >= sort_key(&block)) {
        return Err(WorldStorageError::InvalidFormat(
            "unsorted section overrides",
        ));
    }
    Ok(())
}

fn sort_key(block: &StoredBlock) -> (u8, u8, u8) {
    (block.local_x, block.local_y, block.local_z)
}

fn read_i32(bytes: &[u8], offset: usize) -> Result<i32, WorldStorageError> {
    let value = bytes
        .get(offset..offset + 4)
        .ok_or(WorldStorageError::InvalidFormat("truncated i32"))?;
    Ok(i32::from_le_bytes(value.try_into().unwrap()))
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, WorldStorageError> {
    let value = bytes
        .get(offset..offset + 2)
        .ok_or(WorldStorageError::InvalidFormat("truncated u16"))?;
    Ok(u16::from_le_bytes(value.try_into().unwrap()))
}
