use crate::world::{ChunkPos, ChunkSnapshot, FlatWorld};

#[derive(Debug, Clone)]
pub enum TerrainGenerator {
    Flat(FlatWorld),
    Natural(NaturalWorld),
}

#[derive(Debug, Clone)]
pub struct NaturalWorld {
    flat: FlatWorld,
    seed: i64,
}

impl TerrainGenerator {
    pub fn flat() -> Self {
        Self::Flat(FlatWorld::default())
    }

    pub fn natural(seed: i64) -> Self {
        Self::Natural(NaturalWorld {
            flat: FlatWorld::default(),
            seed,
        })
    }

    pub fn chunk_snapshot(&self, pos: ChunkPos) -> ChunkSnapshot {
        match self {
            Self::Flat(world) => world.chunk_snapshot(pos),
            Self::Natural(world) => world.chunk_snapshot(pos),
        }
    }

    pub fn chunk_positions(&self, center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
        FlatWorld::default().chunk_positions(center, radius)
    }

    pub fn spawn(&self) -> (f64, f64, f64) {
        FlatWorld::default().spawn()
    }
}

impl NaturalWorld {
    pub fn chunk_snapshot(&self, pos: ChunkPos) -> ChunkSnapshot {
        if pos.x.abs().max(pos.z.abs()) <= 1 {
            return self.flat.chunk_snapshot(pos);
        }
        let mut heights = [79; 256];
        for z in 0..16 {
            for x in 0..16 {
                let gx = pos.x * 16 + x as i32;
                let gz = pos.z * 16 + z as i32;
                heights[z * 16 + x] = self.height_at(gx, gz);
            }
        }
        ChunkSnapshot::natural(pos, self.seed, heights)
    }

    fn height_at(&self, x: i32, z: i32) -> i32 {
        let broad = value_noise(self.seed, x, z, 32) * 18.0;
        let detail = value_noise(self.seed ^ 0x51f2_d36a, x, z, 8) * 5.0;
        (79.0 + broad + detail).round().clamp(62.0, 96.0) as i32
    }
}

fn value_noise(seed: i64, x: i32, z: i32, scale: i32) -> f64 {
    let x0 = floor_div(x, scale);
    let z0 = floor_div(z, scale);
    let xf = (x - x0 * scale) as f64 / scale as f64;
    let zf = (z - z0 * scale) as f64 / scale as f64;
    let a = hash_unit(seed, x0, z0);
    let b = hash_unit(seed, x0 + 1, z0);
    let c = hash_unit(seed, x0, z0 + 1);
    let d = hash_unit(seed, x0 + 1, z0 + 1);
    let sx = smooth(xf);
    let sz = smooth(zf);
    lerp(lerp(a, b, sx), lerp(c, d, sx), sz)
}

fn hash_unit(seed: i64, x: i32, z: i32) -> f64 {
    let mut n = seed as u64;
    n ^= (x as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    n ^= (z as i64 as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    n ^= n >> 30;
    n = n.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    n ^= n >> 27;
    n = n.wrapping_mul(0x94d0_49bb_1331_11eb);
    (((n ^ (n >> 31)) >> 11) as f64 / ((1_u64 << 53) as f64)) * 2.0 - 1.0
}

fn floor_div(value: i32, divisor: i32) -> i32 {
    value.div_euclid(divisor)
}

fn smooth(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::TerrainGenerator;
    use crate::world::{BlockState, ChunkPos};

    #[test]
    fn natural_terrain_is_deterministic() {
        let a = TerrainGenerator::natural(42).chunk_snapshot(ChunkPos::new(2, -1));
        let b = TerrainGenerator::natural(42).chunk_snapshot(ChunkPos::new(2, -1));
        assert_eq!(a.base_entries_for_tests(), b.base_entries_for_tests());
    }

    #[test]
    fn spawn_plateau_matches_flat_surface() {
        let chunk = TerrainGenerator::natural(7).chunk_snapshot(ChunkPos::new(1, -1));
        assert!(chunk.is_shared_flat_base());
        assert_eq!(chunk.block_at(79), BlockState::GrassBlock);
        assert_eq!(chunk.block_at(80), BlockState::Air);
    }

    #[test]
    fn outer_chunks_have_height_variation() {
        let chunk = TerrainGenerator::natural(7).chunk_snapshot(ChunkPos::new(2, 0));
        let entries = chunk.base_entries_for_tests();
        assert!(
            entries
                .iter()
                .any(|(_, y, _, state)| *y != 79 && *state == BlockState::GrassBlock)
        );
    }
}
