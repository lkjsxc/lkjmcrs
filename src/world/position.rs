#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
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
    use super::{ChunkPos, RegionSection, unpack_i32_pair};

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
}
