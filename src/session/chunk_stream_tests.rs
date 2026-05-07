use crate::session::chunk_stream::{ChunkStream, chunk_center, visible_chunks};
use crate::world::ChunkPos;
use std::collections::HashSet;

#[test]
fn chunk_center_uses_floored_euclidean_coordinates() {
    assert_eq!(chunk_center(0.0, 15.999), ChunkPos::new(0, 0));
    assert_eq!(chunk_center(16.0, 31.0), ChunkPos::new(1, 1));
    assert_eq!(chunk_center(-0.1, -1.0), ChunkPos::new(-1, -1));
    assert_eq!(chunk_center(-16.0, -16.1), ChunkPos::new(-1, -2));
}

#[test]
fn visible_diff_from_origin_to_east_is_new_column() {
    let mut stream = ChunkStream::new(ChunkPos::new(0, 0), 2);
    let leaving: HashSet<_> = stream
        .advance(ChunkPos::new(1, 0))
        .unwrap()
        .into_iter()
        .collect();
    assert_eq!(leaving, (-2..=2).map(|z| ChunkPos::new(-2, z)).collect());
    assert_eq!(stream.pending_len(), 5);
}

#[test]
fn same_center_produces_no_delta() {
    let mut stream = ChunkStream::new(ChunkPos::new(0, 0), 2);
    assert_eq!(stream.advance(ChunkPos::new(0, 0)), None);
}

#[test]
fn visible_chunks_are_square() {
    assert_eq!(visible_chunks(ChunkPos::new(0, 0), 2).len(), 25);
}

#[test]
fn larger_radius_bootstraps_near_chunks_and_queues_far_chunks() {
    let stream = ChunkStream::new(ChunkPos::new(0, 0), 4);
    assert_eq!(stream.initial_chunks().len(), 25);
    assert_eq!(stream.pending_len(), 56);
}

#[test]
fn movement_replaces_stale_pending_chunks() {
    let mut stream = ChunkStream::new(ChunkPos::new(0, 0), 4);
    assert!(stream.advance(ChunkPos::new(1, 0)).unwrap().is_empty());
    assert_eq!(stream.pending_len(), 56);
}
