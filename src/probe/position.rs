pub(super) const MIN_Y: i32 = -64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub(super) const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
