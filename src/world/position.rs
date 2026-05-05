#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockFace {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionSection {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub const fn packed(self) -> u64 {
        pack_i32_pair(self.x, self.z)
    }

    pub const fn section(self, shift: u8) -> RegionSection {
        RegionSection {
            x: self.x >> shift,
            z: self.z >> shift,
        }
    }
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn chunk(self) -> ChunkPos {
        ChunkPos {
            x: self.x.div_euclid(16),
            z: self.z.div_euclid(16),
        }
    }

    pub fn local_x(self) -> usize {
        self.x.rem_euclid(16) as usize
    }

    pub fn local_z(self) -> usize {
        self.z.rem_euclid(16) as usize
    }

    pub const fn offset(self, face: BlockFace) -> Self {
        match face {
            BlockFace::Down => Self::new(self.x, self.y - 1, self.z),
            BlockFace::Up => Self::new(self.x, self.y + 1, self.z),
            BlockFace::North => Self::new(self.x, self.y, self.z - 1),
            BlockFace::South => Self::new(self.x, self.y, self.z + 1),
            BlockFace::West => Self::new(self.x - 1, self.y, self.z),
            BlockFace::East => Self::new(self.x + 1, self.y, self.z),
        }
    }
}

impl RegionSection {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub const fn packed(self) -> u64 {
        pack_i32_pair(self.x, self.z)
    }
}

pub const fn pack_i32_pair(x: i32, z: i32) -> u64 {
    ((x as u32 as u64) << 32) | (z as u32 as u64)
}

pub const fn unpack_i32_pair(value: u64) -> (i32, i32) {
    ((value >> 32) as u32 as i32, value as u32 as i32)
}

#[cfg(test)]
mod tests {
    use super::{BlockFace, BlockPos, ChunkPos, RegionSection, unpack_i32_pair};

    #[test]
    fn packs_negative_coordinates() {
        let pos = ChunkPos::new(-17, 33);
        assert_eq!(unpack_i32_pair(pos.packed()), (-17, 33));
    }

    #[test]
    fn shifts_to_region_sections() {
        assert_eq!(ChunkPos::new(31, -1).section(4), RegionSection::new(1, -1));
        assert_eq!(ChunkPos::new(-17, 0).section(4), RegionSection::new(-2, 0));
    }

    #[test]
    fn block_positions_use_euclidean_chunk_mapping() {
        let pos = BlockPos::new(-1, 80, -17);
        assert_eq!(pos.chunk(), ChunkPos::new(-1, -2));
        assert_eq!((pos.local_x(), pos.local_z()), (15, 15));
    }

    #[test]
    fn block_face_offsets_target_neighbors() {
        assert_eq!(
            BlockPos::new(0, 79, 0).offset(BlockFace::Up),
            BlockPos::new(0, 80, 0)
        );
    }
}
