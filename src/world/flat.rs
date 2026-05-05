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
        let mut chunks = Vec::new();
        for z in -radius..=radius {
            for x in -radius..=radius {
                chunks.push(self.chunk_snapshot(ChunkPos::new(x, z)));
            }
        }
        chunks
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
    }
}
