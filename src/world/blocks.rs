use crate::world::chunk_layers;
use crate::world::{BlockPos, ChunkPos};
use std::collections::HashMap;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 384;
pub const MIN_Y: i32 = -64;
pub const MAX_Y: i32 = MIN_Y + CHUNK_HEIGHT as i32 - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockState {
    Air,
    Bedrock,
    Stone,
    Dirt,
    GrassBlock,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainKind {
    Natural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratedChunkKey {
    pub kind: TerrainKind,
    pub world_seed: i64,
    pub pos: ChunkPos,
}

#[derive(Debug, Clone)]
pub struct ChunkSnapshot {
    pub pos: ChunkPos,
    palette: [BlockState; 6],
    layers: Vec<u8>,
    overrides: HashMap<u16, BlockState>,
    shared_flat_base: bool,
    generated_key: Option<GeneratedChunkKey>,
}

impl ChunkSnapshot {
    pub fn flat(pos: ChunkPos) -> Self {
        Self {
            pos,
            palette: chunk_layers::palette(),
            layers: chunk_layers::flat_layers(),
            overrides: HashMap::new(),
            shared_flat_base: true,
            generated_key: None,
        }
    }

    pub(super) fn natural(
        pos: ChunkPos,
        world_seed: i64,
        columns: [chunk_layers::TerrainColumn; 256],
    ) -> Self {
        Self {
            pos,
            palette: chunk_layers::palette(),
            layers: chunk_layers::terrain_layers(pos, world_seed, &columns),
            overrides: HashMap::new(),
            shared_flat_base: false,
            generated_key: Some(GeneratedChunkKey {
                kind: TerrainKind::Natural,
                world_seed,
                pos,
            }),
        }
    }

    pub fn block_at(&self, y: i32) -> BlockState {
        self.block_at_local(0, y, 0)
    }

    pub fn block_at_local(&self, x: usize, y: i32, z: usize) -> BlockState {
        if !valid_local(x, z) {
            return BlockState::Air;
        }
        if !in_world(y) {
            return BlockState::Air;
        }
        if let Some(state) = self.overrides.get(&local_key(x, y, z)) {
            return *state;
        }
        self.base_block_at_local(x, y, z)
    }

    pub fn block_at_pos(&self, pos: BlockPos) -> BlockState {
        if pos.chunk() != self.pos {
            return BlockState::Air;
        }
        self.block_at_local(pos.local_x(), pos.y, pos.local_z())
    }

    pub fn heightmap_at_local(&self, x: usize, z: usize) -> u16 {
        for y in (MIN_Y..=MAX_Y).rev() {
            if self.block_at_local(x, y, z) != BlockState::Air {
                return (y + 1) as u16;
            }
        }
        0
    }

    pub fn set_block(&mut self, pos: BlockPos, state: BlockState) -> Option<BlockState> {
        if pos.chunk() != self.pos || !in_world(pos.y) {
            return None;
        }
        let base = self.base_block_at_local(pos.local_x(), pos.y, pos.local_z());
        if base == BlockState::Bedrock {
            return Some(BlockState::Bedrock);
        }
        let key = local_key(pos.local_x(), pos.y, pos.local_z());
        if state == base {
            self.overrides.remove(&key);
        } else {
            self.overrides.insert(key, state);
        }
        Some(self.block_at_pos(pos))
    }

    pub fn is_shared_flat_base(&self) -> bool {
        self.shared_flat_base && self.overrides.is_empty()
    }

    pub fn generated_cache_key(&self) -> Option<GeneratedChunkKey> {
        self.overrides
            .is_empty()
            .then_some(self.generated_key)
            .flatten()
    }

    pub fn override_entries(&self) -> Vec<(BlockPos, BlockState)> {
        let mut entries = self
            .overrides
            .iter()
            .map(|(key, state)| (self.pos.global_block_pos(*key), *state))
            .collect::<Vec<_>>();
        entries.sort_by_key(|(pos, _)| (pos.x, pos.y, pos.z));
        entries
    }

    #[cfg(test)]
    pub fn base_entries_for_tests(&self) -> Vec<(usize, i32, usize, BlockState)> {
        let mut entries = Vec::new();
        for z in 0..CHUNK_WIDTH {
            for x in 0..CHUNK_WIDTH {
                for y in MIN_Y..=MAX_Y {
                    let state = self.base_block_at_local(x, y, z);
                    if state != BlockState::Air {
                        entries.push((x, y, z, state));
                    }
                }
            }
        }
        entries
    }

    fn base_block_at_local(&self, x: usize, y: i32, z: usize) -> BlockState {
        if !in_world(y) || !valid_local(x, z) {
            return BlockState::Air;
        }
        let index = chunk_layers::layer_index(x, y, z);
        self.palette[self.layers[index] as usize]
    }
}

fn in_world(y: i32) -> bool {
    (MIN_Y..=MAX_Y).contains(&y)
}

fn valid_local(x: usize, z: usize) -> bool {
    x < CHUNK_WIDTH && z < CHUNK_WIDTH
}

fn local_key(x: usize, y: i32, z: usize) -> u16 {
    let y_index = (y - MIN_Y) as u16;
    ((y_index & 0x01ff) << 8) | ((z as u16) << 4) | x as u16
}

impl ChunkPos {
    fn global_block_pos(self, key: u16) -> BlockPos {
        let x = (key & 0x000f) as i32;
        let z = ((key >> 4) & 0x000f) as i32;
        let y = ((key >> 8) as i32) + MIN_Y;
        BlockPos::new(
            self.x * CHUNK_WIDTH as i32 + x,
            y,
            self.z * CHUNK_WIDTH as i32 + z,
        )
    }
}

#[cfg(test)]
#[path = "blocks_tests.rs"]
mod tests;
