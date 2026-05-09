use super::storage_meta::ChunkMeta;
use std::collections::BTreeMap;

#[test]
fn chunk_meta_tracks_dirty_sections_and_save_count() {
    let sections = BTreeMap::from([(-1, vec![1, 2]), (5, vec![3, 4])]);
    let first = ChunkMeta::from_sections(&sections, None);
    let second = ChunkMeta::from_sections(&sections, Some(first));

    assert_eq!(
        first.dirty_sections().into_iter().collect::<Vec<_>>(),
        [-1, 5]
    );
    assert_eq!(ChunkMeta::decode(&second.encode()).unwrap(), second);
    assert_ne!(first.encode(), second.encode());
}
