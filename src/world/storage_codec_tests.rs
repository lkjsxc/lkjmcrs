use crate::world::storage_codec::StoredChunk;
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot};

const STATE_CODE_OFFSET: usize = 26;

#[test]
fn binary_chunk_round_trips_multiple_overrides() {
    let pos = ChunkPos::new(-2, 3);
    let first = BlockPos::new(-32, 80, 48);
    let second = BlockPos::new(-17, 81, 63);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(first, BlockState::Stone);
    chunk.set_block(second, BlockState::Dirt);

    let bytes = StoredChunk::from_snapshot(&chunk).encode().unwrap();
    let loaded = StoredChunk::decode(&bytes)
        .unwrap()
        .apply_to(ChunkSnapshot::flat(pos))
        .unwrap();

    assert_eq!(loaded.block_at_pos(first), BlockState::Stone);
    assert_eq!(loaded.block_at_pos(second), BlockState::Dirt);
}

#[test]
fn binary_chunk_round_trips_water_override() {
    let pos = ChunkPos::new(0, 0);
    let water = BlockPos::new(2, 63, 2);
    let mut chunk = ChunkSnapshot::flat(pos);
    chunk.set_block(water, BlockState::Water);

    let loaded = StoredChunk::decode(&StoredChunk::from_snapshot(&chunk).encode().unwrap())
        .unwrap()
        .apply_to(ChunkSnapshot::flat(pos))
        .unwrap();

    assert_eq!(loaded.block_at_pos(water), BlockState::Water);
}

#[test]
fn invalid_state_code_is_rejected() {
    let mut bytes = one_override_bytes();
    bytes[STATE_CODE_OFFSET..STATE_CODE_OFFSET + 2].copy_from_slice(&99_u16.to_le_bytes());

    let error = StoredChunk::decode(&bytes).unwrap_err().to_string();

    assert!(error.contains("invalid block state"));
}

#[test]
fn invalid_local_coordinate_is_rejected() {
    let mut bytes = one_override_bytes();
    bytes[20] = 16;

    let error = StoredChunk::decode(&bytes).unwrap_err().to_string();

    assert!(error.contains("invalid stored block"));
}

#[test]
fn truncated_data_is_rejected() {
    let mut bytes = one_override_bytes();
    bytes.pop();

    let error = StoredChunk::decode(&bytes).unwrap_err().to_string();

    assert!(error.contains("invalid world storage format"));
}

#[test]
fn coordinate_mismatch_is_rejected() {
    let bytes = one_override_bytes();

    let error = StoredChunk::decode(&bytes)
        .unwrap()
        .apply_to(ChunkSnapshot::flat(ChunkPos::new(1, 0)))
        .unwrap_err()
        .to_string();

    assert!(error.contains("invalid stored chunk key"));
}

#[test]
fn duplicate_override_position_is_rejected() {
    let mut bytes = one_override_bytes();
    bytes[18..20].copy_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_within(20..28);

    let error = StoredChunk::decode(&bytes).unwrap_err().to_string();

    assert!(error.contains("unsorted overrides"));
}

fn one_override_bytes() -> Vec<u8> {
    let mut chunk = ChunkSnapshot::flat(ChunkPos::new(0, 0));
    chunk.set_block(BlockPos::new(0, 80, 0), BlockState::Stone);
    StoredChunk::from_snapshot(&chunk).encode().unwrap()
}
