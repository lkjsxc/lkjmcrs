use crate::world::WorldStorageError;
use crate::world::storage_section_codec::section_range;
use std::collections::{BTreeMap, BTreeSet};

const MAGIC: &[u8; 8] = b"LKJMCRSM";
const FORMAT: u8 = 1;
const LEN: usize = 33;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ChunkMeta {
    dirty_mask: u64,
    content_hash: u64,
    save_count: u64,
}

impl ChunkMeta {
    pub(super) fn from_sections(sections: &BTreeMap<i32, Vec<u8>>, previous: Option<Self>) -> Self {
        Self {
            dirty_mask: dirty_mask(sections.keys().copied()),
            content_hash: content_hash(sections),
            save_count: previous.map_or(1, |meta| meta.save_count + 1),
        }
    }

    pub(super) fn encode(self) -> [u8; LEN] {
        let mut bytes = [0; LEN];
        bytes[..MAGIC.len()].copy_from_slice(MAGIC);
        bytes[MAGIC.len()] = FORMAT;
        bytes[9..17].copy_from_slice(&self.dirty_mask.to_le_bytes());
        bytes[17..25].copy_from_slice(&self.content_hash.to_le_bytes());
        bytes[25..33].copy_from_slice(&self.save_count.to_le_bytes());
        bytes
    }

    pub(super) fn decode(bytes: &[u8]) -> Result<Self, WorldStorageError> {
        if bytes.len() != LEN {
            return Err(WorldStorageError::InvalidFormat(
                "invalid chunk meta length",
            ));
        }
        if &bytes[..MAGIC.len()] != MAGIC || bytes[MAGIC.len()] != FORMAT {
            return Err(WorldStorageError::InvalidFormat(
                "invalid chunk meta marker",
            ));
        }
        Ok(Self {
            dirty_mask: read_u64(bytes, 9),
            content_hash: read_u64(bytes, 17),
            save_count: read_u64(bytes, 25),
        })
    }

    pub(super) fn dirty_sections(self) -> BTreeSet<i32> {
        section_range()
            .enumerate()
            .filter_map(|(index, section)| {
                (self.dirty_mask & (1_u64 << index) != 0).then_some(section)
            })
            .collect()
    }
}

fn dirty_mask(sections: impl Iterator<Item = i32>) -> u64 {
    let min = *section_range().start();
    sections.fold(0, |mask, section| {
        let index = (section - min) as u64;
        mask | (1_u64 << index)
    })
}

fn content_hash(sections: &BTreeMap<i32, Vec<u8>>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for (section, bytes) in sections {
        hash = fnv(hash, &section.to_le_bytes());
        hash = fnv(hash, bytes);
    }
    hash
}

fn fnv(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}
