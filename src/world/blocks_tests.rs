use super::{BlockState, ChunkSnapshot};
use crate::world::{BlockPos, ChunkPos};

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

#[test]
fn sparse_overrides_mutate_only_target_blocks() {
    let mut chunk = ChunkSnapshot::flat(ChunkPos::new(-1, 0));
    let pos = BlockPos::new(-1, 80, 0);
    assert_eq!(
        chunk.set_block(pos, BlockState::Stone),
        Some(BlockState::Stone)
    );
    assert_eq!(chunk.block_at_pos(pos), BlockState::Stone);
    assert_eq!(chunk.block_at_local(14, 80, 0), BlockState::Air);
    assert_eq!(chunk.set_block(pos, BlockState::Air), Some(BlockState::Air));
}

#[test]
fn override_entries_return_global_positions() {
    let mut chunk = ChunkSnapshot::flat(ChunkPos::new(-1, 0));
    let pos = BlockPos::new(-1, 80, 0);
    chunk.set_block(pos, BlockState::Stone);
    assert_eq!(chunk.override_entries(), vec![(pos, BlockState::Stone)]);
}

#[test]
fn bedrock_is_immutable() {
    let mut chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
    let pos = BlockPos::new(0, 0, 0);
    assert_eq!(
        chunk.set_block(pos, BlockState::Air),
        Some(BlockState::Bedrock)
    );
    assert_eq!(chunk.block_at_pos(pos), BlockState::Bedrock);
}
