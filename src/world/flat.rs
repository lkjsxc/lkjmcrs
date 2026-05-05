use crate::world::{ChunkPos, ChunkSnapshot};

#[derive(Debug, Clone)]
pub struct FlatWorld {
    section_shift: u8,
    spawn: (f64, f64, f64),
}

impl Default for FlatWorld {
    fn default() -> Self {
        Self {
            section_shift: 4,
            spawn: (0.0, 80.0, 0.0),
        }
    }
}

impl FlatWorld {
    pub const fn section_shift(&self) -> u8 {
        self.section_shift
    }

    pub const fn spawn(&self) -> (f64, f64, f64) {
        self.spawn
    }

    pub fn chunk_snapshot(&self, pos: ChunkPos) -> ChunkSnapshot {
        ChunkSnapshot::flat(pos)
    }

    pub fn spawn_chunks(&self, radius: i32) -> Vec<ChunkSnapshot> {
        self.spawn_chunk_positions(radius)
            .into_iter()
            .map(|pos| self.chunk_snapshot(pos))
            .collect()
    }

    pub fn spawn_chunk_positions(&self, radius: i32) -> Vec<ChunkPos> {
        self.chunk_positions(ChunkPos::new(0, 0), radius)
    }

    pub fn chunk_positions(&self, center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
        let mut positions = Vec::new();
        for z in center.z - radius..=center.z + radius {
            for x in center.x - radius..=center.x + radius {
                positions.push(ChunkPos::new(x, z));
            }
        }
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::FlatWorld;

    #[test]
    fn spawn_chunks_are_square() {
        let world = FlatWorld::default();
        assert_eq!(world.spawn(), (0.0, 80.0, 0.0));
        assert_eq!(world.spawn_chunks(1).len(), 9);
        assert_eq!(world.spawn_chunks(2).len(), 25);
        assert_eq!(
            world
                .chunk_positions(crate::world::ChunkPos::new(2, -1), 0)
                .len(),
            1
        );
    }
}
