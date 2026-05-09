use crate::world::storage_section_codec::{StoredSection, section_y};
use crate::world::{BlockPos, BlockState, ChunkPos, ChunkSnapshot};

const STATE_CODE_OFFSET: usize = 26;

#[test]
fn binary_section_round_trips_multiple_overrides() {
    let pos = ChunkPos::new(-2, 3);
    let first = BlockPos::new(-32, 80, 48);
    let second = BlockPos::new(-17, 81, 63);
    let mut chunk = ChunkSnapshot::flat(pos);

    let bytes = section_bytes(
        pos,
        vec![(first, BlockState::Stone), (second, BlockState::Dirt)],
    );
    StoredSection::decode(&bytes)
        .unwrap()
        .apply_to(&mut chunk)
        .unwrap();

    assert_eq!(chunk.block_at_pos(first), BlockState::Stone);
    assert_eq!(chunk.block_at_pos(second), BlockState::Dirt);
}

#[test]
fn invalid_state_code_is_rejected() {
    let mut bytes = one_override_bytes();
    bytes[STATE_CODE_OFFSET..STATE_CODE_OFFSET + 2].copy_from_slice(&99_u16.to_le_bytes());

    let error = StoredSection::decode(&bytes).unwrap_err().to_string();

    assert!(error.contains("invalid block state"));
}

#[test]
fn invalid_local_coordinate_is_rejected() {
    let mut bytes = one_override_bytes();
    bytes[23] = 16;

    let error = StoredSection::decode(&bytes).unwrap_err().to_string();

    assert!(error.contains("invalid stored block"));
}

#[test]
fn truncated_data_is_rejected() {
    let mut bytes = one_override_bytes();
    bytes.pop();

    let error = StoredSection::decode(&bytes).unwrap_err().to_string();

    assert!(error.contains("invalid world storage format"));
}

#[test]
fn coordinate_mismatch_is_rejected() {
    let bytes = one_override_bytes();
    let mut chunk = ChunkSnapshot::flat(ChunkPos::new(1, 0));

    let error = StoredSection::decode(&bytes)
        .unwrap()
        .apply_to(&mut chunk)
        .unwrap_err()
        .to_string();

    assert!(error.contains("invalid stored chunk key"));
}

fn one_override_bytes() -> Vec<u8> {
    let pos = BlockPos::new(0, 80, 0);
    section_bytes(ChunkPos::new(0, 0), vec![(pos, BlockState::Stone)])
}

fn section_bytes(pos: ChunkPos, entries: Vec<(BlockPos, BlockState)>) -> Vec<u8> {
    let y = section_y(entries[0].0.y);
    StoredSection::from_entries(pos, y, entries)
        .encode()
        .unwrap()
}
