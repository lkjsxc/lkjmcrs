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
