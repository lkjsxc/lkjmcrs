use crate::world::ChunkPos;
use std::collections::{HashSet, VecDeque};

pub const EAGER_RADIUS: i32 = 2;

pub fn chunk_center(x: f64, z: f64) -> ChunkPos {
    ChunkPos::new(block_coord(x).div_euclid(16), block_coord(z).div_euclid(16))
}

pub fn visible_chunks(center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
    assert!(radius >= 0, "chunk radius must be non-negative");
    let mut chunks = Vec::new();
    for z in center.z - radius..=center.z + radius {
        for x in center.x - radius..=center.x + radius {
            chunks.push(ChunkPos::new(x, z));
        }
    }
    chunks
}

pub fn eager_chunks(center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
    visible_chunks(center, radius.min(EAGER_RADIUS))
}

pub fn ordered_pending(
    center: ChunkPos,
    radius: i32,
    sent: &HashSet<ChunkPos>,
) -> VecDeque<ChunkPos> {
    let mut pending = visible_chunks(center, radius)
        .into_iter()
        .filter(|pos| !sent.contains(pos))
        .collect::<Vec<_>>();
    pending.sort_by_key(|pos| {
        let ring = (pos.x - center.x).abs().max((pos.z - center.z).abs());
        (ring, pos.x, pos.z)
    });
    pending.into()
}

fn block_coord(value: f64) -> i32 {
    value.floor() as i32
}
