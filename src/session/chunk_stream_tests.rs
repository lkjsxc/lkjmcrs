use crate::session::chunk_payload_cache::ChunkPayloadCache;
use crate::session::chunk_stream::{ChunkStream, chunk_center, visible_chunks};
use crate::session::chunk_stream_load::encode_loaded_with_budget;
use crate::session::chunk_stream_send::{ChunkSendBudget, EncodedChunk};
use crate::world::{ChunkPos, ChunkSnapshot};
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
    let stream = ChunkStream::new(ChunkPos::new(0, 0), 32);
    assert_eq!(stream.initial_chunks().len(), 25);
    assert_eq!(stream.pending_len(), 4200);
}

#[test]
fn movement_replaces_stale_pending_chunks() {
    let mut stream = ChunkStream::new(ChunkPos::new(0, 0), 32);
    assert!(stream.advance(ChunkPos::new(1, 0)).unwrap().is_empty());
    assert_eq!(stream.pending_len(), 4200);
}

#[test]
fn pending_drain_selects_budget_order() {
    let mut stream = ChunkStream::new(ChunkPos::new(0, 0), 4);
    let drained = stream.drain_pending_positions(8);
    assert_eq!(drained.len(), 8);
    assert_eq!(
        drained,
        vec![
            ChunkPos::new(-3, -3),
            ChunkPos::new(-3, -2),
            ChunkPos::new(-3, -1),
            ChunkPos::new(-3, 0),
            ChunkPos::new(-3, 1),
            ChunkPos::new(-3, 2),
            ChunkPos::new(-3, 3),
            ChunkPos::new(-2, -3),
        ]
    );
    assert_eq!(stream.pending_len(), 48);
}

#[test]
fn byte_budget_allows_one_oversized_chunk() {
    let mut cache = ChunkPayloadCache::default();
    let positions = vec![ChunkPos::new(0, 0), ChunkPos::new(1, 0)];
    let snapshots = positions
        .iter()
        .copied()
        .map(ChunkSnapshot::flat)
        .collect::<Vec<_>>();
    let (chunks, unsent) = encode_loaded_with_budget(
        positions,
        snapshots,
        &mut cache,
        ChunkSendBudget {
            max_chunks: 8,
            max_payload_bytes: 1,
        },
    );
    assert_eq!(chunks.len(), 1);
    assert_eq!(unsent, vec![ChunkPos::new(1, 0)]);
}

#[test]
fn encoded_chunk_test_payload_reports_len() {
    let chunk = EncodedChunk::from_payload_for_tests(ChunkPos::new(0, 0), vec![1, 2, 3]);
    assert_eq!(chunk.len(), 3);
}
