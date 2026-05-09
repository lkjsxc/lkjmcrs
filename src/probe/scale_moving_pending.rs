use crate::probe::ProbeError;
use crate::probe::live_play;
use crate::probe::scale_chunk_stream;
use crate::probe::scale_chunk_stream_packets as packets;
use std::collections::HashSet;
use tokio::time::{Duration, timeout};

const RADIUS: i32 = 8;
const INITIAL_CHUNKS: usize = 25;
const TOTAL_CHUNKS: usize = 289;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = scale_chunk_stream::connect(host).await?;
    let mut seen = scale_chunk_stream::read_bootstrap(&mut stream, RADIUS).await?;
    if seen.len() != INITIAL_CHUNKS {
        return Err(Box::new(ProbeError::Phase("moving pending initial")));
    }
    live_play::send_position_look_at(&mut stream, 16.5, 80.0, 0.5, 0.0, 0.0).await?;
    let expected = window(1, 0, RADIUS);
    for batch in packets::expect_cache_center_collecting_batches(&mut stream, 1, 0).await? {
        accept_batch(&mut seen, &expected, batch.positions)?;
    }
    while seen.len() < TOTAL_CHUNKS {
        let batch = timeout(
            Duration::from_secs(5),
            packets::read_next_batch(&mut stream),
        )
        .await
        .map_err(|_| missing_chunks_error(seen.len()))??;
        accept_batch(&mut seen, &expected, batch.positions)?;
    }
    if seen != expected {
        return Err(Box::new(ProbeError::Phase("moving pending total")));
    }
    Ok(())
}

fn accept_batch(
    seen: &mut HashSet<(i32, i32)>,
    expected: &HashSet<(i32, i32)>,
    batch: HashSet<(i32, i32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    reject_stale_chunks(&batch, expected)?;
    reject_duplicate_chunks(seen, &batch)?;
    seen.extend(batch);
    Ok(())
}

fn missing_chunks_error(seen: usize) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::other(format!(
        "moving pending timed out after {seen} chunks"
    )))
}

fn reject_stale_chunks(
    batch: &HashSet<(i32, i32)>,
    expected: &HashSet<(i32, i32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    if batch.iter().any(|pos| !expected.contains(pos)) {
        return Err(Box::new(ProbeError::Phase("moving pending stale chunk")));
    }
    Ok(())
}

fn reject_duplicate_chunks(
    seen: &HashSet<(i32, i32)>,
    batch: &HashSet<(i32, i32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    if batch.iter().any(|pos| seen.contains(pos)) {
        return Err(Box::new(ProbeError::Phase("moving pending duplicate")));
    }
    Ok(())
}

fn window(center_x: i32, center_z: i32, radius: i32) -> HashSet<(i32, i32)> {
    let mut chunks = HashSet::new();
    for z in center_z - radius..=center_z + radius {
        for x in center_x - radius..=center_x + radius {
            chunks.insert((x, z));
        }
    }
    chunks
}
