use crate::probe::ProbeError;
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
        return Err(Box::new(ProbeError::Phase("scale load initial chunks")));
    }
    let mut followup_batches = 0;
    let mut max_followup_batch = 0;
    let mut max_followup_payload_bytes = 0;
    while seen.len() < TOTAL_CHUNKS {
        let batch = packets::read_next_batch(&mut stream).await?;
        followup_batches += 1;
        max_followup_batch = max_followup_batch.max(batch.positions.len());
        max_followup_payload_bytes = max_followup_payload_bytes.max(batch.payload_bytes);
        reject_duplicate_chunks(&seen, &batch.positions)?;
        seen.extend(batch.positions);
    }
    if seen.len() != TOTAL_CHUNKS {
        return Err(Box::new(ProbeError::Phase("scale load total chunks")));
    }
    if max_followup_batch > packets::MAX_BATCH {
        return Err(Box::new(ProbeError::Phase("scale load max batch")));
    }
    if max_followup_payload_bytes > packets::MAX_PAYLOAD_BYTES {
        return Err(Box::new(ProbeError::Phase("scale load max payload bytes")));
    }
    if followup_batches == 0 {
        return Err(Box::new(ProbeError::Phase("scale load counters")));
    }
    println!(
        "scale-load-metrics counters radius={RADIUS} initial={INITIAL_CHUNKS} total={} followup_batches={followup_batches} max_followup_batch={max_followup_batch} max_followup_payload_bytes={max_followup_payload_bytes}",
        seen.len()
    );
    Ok(())
}

fn reject_duplicate_chunks(
    seen: &HashSet<(i32, i32)>,
    batch: &HashSet<(i32, i32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    if batch.iter().any(|pos| seen.contains(pos)) {
        return Err(Box::new(ProbeError::Phase("scale load duplicate chunk")));
    }
    Ok(())
}
