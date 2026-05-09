use crate::world::terrain;
use crate::world::{ChunkPos, ChunkSnapshot, FlatWorld};

#[derive(Debug, Clone)]
pub enum TerrainGenerator {
    Flat(FlatWorld),
    Natural(NaturalWorld),
}

#[derive(Debug, Clone)]
pub struct NaturalWorld {
    seed: i64,
}

impl TerrainGenerator {
    pub fn flat() -> Self {
        Self::Flat(FlatWorld::default())
    }

    pub fn natural(seed: i64) -> Self {
        Self::Natural(NaturalWorld { seed })
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
        match self {
            Self::Flat(_) => (0.5, 80.0, 0.5),
            Self::Natural(world) => world.spawn(),
        }
    }
}

impl NaturalWorld {
    pub fn chunk_snapshot(&self, pos: ChunkPos) -> ChunkSnapshot {
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

    pub fn spawn(&self) -> (f64, f64, f64) {
        terrain::spawn_position(self.seed)
    }

    fn height_at(&self, x: i32, z: i32) -> i32 {
        self.natural_height_at(x, z)
    }

    fn natural_height_at(&self, x: i32, z: i32) -> i32 {
        terrain::surface_height(self.seed, x, z)
    }
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
    fn natural_spawn_uses_scored_column() {
        let spawn = TerrainGenerator::natural(7).spawn();
        assert_ne!(spawn, (0.5, 80.0, 0.5));
        assert_eq!(spawn.0.fract(), 0.5);
        assert_eq!(spawn.2.fract(), 0.5);
    }

    #[test]
    fn blended_outer_chunks_have_height_variation() {
        let chunk = TerrainGenerator::natural(7).chunk_snapshot(ChunkPos::new(2, 0));
        let entries = chunk.base_entries_for_tests();
        assert!(
            entries
                .iter()
                .any(|(_, y, _, state)| *y != 79 && *state == BlockState::GrassBlock)
        );
    }

    #[test]
    fn far_chunks_are_fully_natural() {
        let blended = TerrainGenerator::natural(7).chunk_snapshot(ChunkPos::new(2, 0));
        let far = TerrainGenerator::natural(7).chunk_snapshot(ChunkPos::new(7, 0));
        assert_ne!(
            blended.base_entries_for_tests(),
            far.base_entries_for_tests()
        );
    }
}
