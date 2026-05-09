use crate::world::blocks::MAX_Y;
use crate::world::storage_blocks::{block_state, state_code};
use crate::world::{BlockPos, ChunkSnapshot, MIN_Y, WorldStorageError};

const MAGIC: &[u8; 9] = b"LKJMCRSCO";
const FORMAT: u8 = 1;
const HEADER_LEN: usize = 20;
const RECORD_LEN: usize = 8;

#[derive(Debug)]
pub(super) struct StoredChunk {
    chunk_x: i32,
    chunk_z: i32,
    overrides: Vec<StoredBlock>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StoredBlock {
    local_x: u8,
    y: i32,
    local_z: u8,
    state_code: u16,
}

impl StoredChunk {
    pub(super) fn from_snapshot(chunk: &ChunkSnapshot) -> Self {
        Self {
            chunk_x: chunk.pos.x,
            chunk_z: chunk.pos.z,
            overrides: chunk
                .override_entries()
                .into_iter()
                .map(|(pos, state)| StoredBlock {
                    local_x: pos.local_x() as u8,
                    y: pos.y,
                    local_z: pos.local_z() as u8,
                    state_code: state_code(state),
                })
                .collect(),
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.overrides.is_empty()
    }

    pub(super) fn encode(&self) -> Result<Vec<u8>, WorldStorageError> {
        let count = u16::try_from(self.overrides.len())
            .map_err(|_| WorldStorageError::InvalidFormat("too many overrides"))?;
        let mut bytes = Vec::with_capacity(HEADER_LEN + self.overrides.len() * RECORD_LEN);
        bytes.extend_from_slice(MAGIC);
        bytes.push(FORMAT);
        bytes.extend_from_slice(&self.chunk_x.to_le_bytes());
        bytes.extend_from_slice(&self.chunk_z.to_le_bytes());
        bytes.extend_from_slice(&count.to_le_bytes());
        for block in &self.overrides {
            bytes.push(block.local_x);
            bytes.extend_from_slice(&block.y.to_le_bytes());
            bytes.push(block.local_z);
            bytes.extend_from_slice(&block.state_code.to_le_bytes());
        }
        Ok(bytes)
    }

    pub(super) fn decode(bytes: &[u8]) -> Result<Self, WorldStorageError> {
        if bytes.len() < HEADER_LEN {
            return Err(WorldStorageError::InvalidFormat("truncated header"));
        }
        if &bytes[..MAGIC.len()] != MAGIC {
            return Err(WorldStorageError::InvalidFormat("invalid magic"));
        }
        if bytes[MAGIC.len()] != FORMAT {
            return Err(WorldStorageError::InvalidFormat("invalid format marker"));
        }
        let chunk_x = read_i32(bytes, 10)?;
        let chunk_z = read_i32(bytes, 14)?;
        let count = read_u16(bytes, 18)? as usize;
        if bytes.len() != HEADER_LEN + count * RECORD_LEN {
            return Err(WorldStorageError::InvalidFormat("invalid length"));
        }
        let mut overrides = Vec::with_capacity(count);
        let mut last = None;
        for index in 0..count {
            let offset = HEADER_LEN + index * RECORD_LEN;
            let block = StoredBlock {
                local_x: bytes[offset],
                y: read_i32(bytes, offset + 1)?,
                local_z: bytes[offset + 5],
                state_code: read_u16(bytes, offset + 6)?,
            };
            validate_block(block, last)?;
            last = Some(block);
            overrides.push(block);
        }
        Ok(Self {
            chunk_x,
            chunk_z,
            overrides,
        })
    }

    pub(super) fn apply_to(
        self,
        mut chunk: ChunkSnapshot,
    ) -> Result<ChunkSnapshot, WorldStorageError> {
        let pos = chunk.pos;
        if self.chunk_x != pos.x || self.chunk_z != pos.z {
            return Err(WorldStorageError::InvalidChunkKey);
        }
        for block in self.overrides {
            let state = block_state(block.state_code)?;
            let pos = BlockPos::new(
                pos.x * 16 + i32::from(block.local_x),
                block.y,
                pos.z * 16 + i32::from(block.local_z),
            );
            if chunk.set_block(pos, state) != Some(state) {
                return Err(WorldStorageError::InvalidBlock(
                    i32::from(block.local_x),
                    block.y,
                    i32::from(block.local_z),
                ));
            }
        }
        Ok(chunk)
    }
}

fn validate_block(block: StoredBlock, last: Option<StoredBlock>) -> Result<(), WorldStorageError> {
    if block.local_x > 15 || block.local_z > 15 || !(MIN_Y..=MAX_Y).contains(&block.y) {
        return Err(WorldStorageError::InvalidBlock(
            i32::from(block.local_x),
            block.y,
            i32::from(block.local_z),
        ));
    }
    block_state(block.state_code)?;
    if last.is_some_and(|previous| sort_key(previous) >= sort_key(block)) {
        return Err(WorldStorageError::InvalidFormat("unsorted overrides"));
    }
    Ok(())
}

fn sort_key(block: StoredBlock) -> (u8, i32, u8) {
    (block.local_x, block.y, block.local_z)
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
