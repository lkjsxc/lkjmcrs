use crate::probe::ProbeError;
use crate::probe::live_play;
use crate::probe::scale_chunk_stream;
use crate::probe::scale_chunk_stream_packets as packets;
use std::collections::HashSet;

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
    packets::expect_cache_center(&mut stream, 1, 0).await?;
    let expected = window(1, 0, RADIUS);
    while seen.len() < TOTAL_CHUNKS {
        let batch = packets::read_next_batch(&mut stream).await?;
        reject_stale_chunks(&batch.positions, &expected)?;
        reject_duplicate_chunks(&seen, &batch.positions)?;
        seen.extend(batch.positions);
    }
    if seen != expected {
        return Err(Box::new(ProbeError::Phase("moving pending total")));
    }
    Ok(())
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
