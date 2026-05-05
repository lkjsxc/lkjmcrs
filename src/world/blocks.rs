use crate::world::ChunkPos;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 384;
pub const MIN_Y: i32 = -64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockState {
    Air,
    Bedrock,
    Stone,
    Dirt,
    GrassBlock,
}

#[derive(Debug, Clone)]
pub struct ChunkSnapshot {
    pub pos: ChunkPos,
    palette: [BlockState; 5],
    layers: Vec<u8>,
}

impl ChunkSnapshot {
    pub fn flat(pos: ChunkPos) -> Self {
        Self {
            pos,
            palette: [
                BlockState::Air,
                BlockState::Bedrock,
                BlockState::Stone,
                BlockState::Dirt,
                BlockState::GrassBlock,
            ],
            layers: build_flat_layers(),
        }
    }

    pub fn block_at(&self, y: i32) -> BlockState {
        let index = y - MIN_Y;
        if !(0..CHUNK_HEIGHT as i32).contains(&index) {
            return BlockState::Air;
        }
        self.palette[self.layers[index as usize] as usize]
    }

    pub fn unique_palette_len(&self) -> usize {
        self.palette.len()
    }
}

fn build_flat_layers() -> Vec<u8> {
    let mut layers = vec![0; CHUNK_HEIGHT];
    for y in 0..=79 {
        let state = match y {
            0 => 1,
            1..=62 => 2,
            63..=78 => 3,
            79 => 4,
            _ => 0,
        };
        layers[(y - MIN_Y) as usize] = state;
    }
    layers
}

#[cfg(test)]
mod tests {
    use super::{BlockState, ChunkSnapshot};
    use crate::world::ChunkPos;

    #[test]
    fn flat_chunk_layers_match_contract() {
        let chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
        assert_eq!(chunk.block_at(-1), BlockState::Air);
        assert_eq!(chunk.block_at(0), BlockState::Bedrock);
        assert_eq!(chunk.block_at(62), BlockState::Stone);
        assert_eq!(chunk.block_at(78), BlockState::Dirt);
        assert_eq!(chunk.block_at(79), BlockState::GrassBlock);
        assert_eq!(chunk.block_at(80), BlockState::Air);
    }
}
