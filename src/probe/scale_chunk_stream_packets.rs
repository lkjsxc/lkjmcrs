use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::terrain_chunk::DecodedChunk;
use crate::probe::validation::validate_chunk_batch_finished;
use crate::protocol::{codec, ids, play};
use crate::session::stream_budget::{MAX_FOLLOWUP_CHUNKS, MAX_FOLLOWUP_PAYLOAD_BYTES};
use std::collections::HashSet;
use std::io::Cursor;
use tokio::net::TcpStream;

pub const RADIUS: i32 = 4;

#[derive(Debug)]
pub struct ChunkBatch {
    pub positions: HashSet<(i32, i32)>,
    pub payload_bytes: usize,
    pub has_non_flat_surface: bool,
}

pub async fn read_next_batch(
    stream: &mut TcpStream,
) -> Result<ChunkBatch, Box<dyn std::error::Error>> {
    loop {
        let packet = read_non_periodic(stream).await?;
        match packet.id {
            ids::play::CHUNK_BATCH_START => return read_batch_after_start(stream).await,
            id if ignorable_live_packet(id) => {}
            _ => return Err(packet_error("scale stream packet", packet.id)),
        }
    }
}

pub async fn expect_batch(
    stream: &mut TcpStream,
    expected: usize,
) -> Result<ChunkBatch, Box<dyn std::error::Error>> {
    super::expect(stream, ids::play::CHUNK_BATCH_START, "scale batch start").await?;
    let batch = read_chunks(stream, expected).await?;
    let finished =
        super::expect(stream, ids::play::CHUNK_BATCH_FINISHED, "scale batch end").await?;
    validate_chunk_batch_finished(finished.data, expected)?;
    Ok(ChunkBatch {
        positions: batch,
        payload_bytes: 0,
        has_non_flat_surface: false,
    })
}

async fn read_batch_after_start(
    stream: &mut TcpStream,
) -> Result<ChunkBatch, Box<dyn std::error::Error>> {
    let mut batch = HashSet::new();
    let mut payload_bytes = 0;
    let mut has_non_flat_surface = false;
    loop {
        let packet = read_non_periodic(stream).await?;
        if packet.id == ids::play::CHUNK_BATCH_FINISHED {
            validate_chunk_batch_finished(packet.data, batch.len())?;
            if batch.is_empty() || batch.len() > MAX_FOLLOWUP_CHUNKS {
                return Err(Box::new(ProbeError::Phase("scale batch size")));
            }
            if payload_bytes > MAX_FOLLOWUP_PAYLOAD_BYTES && batch.len() > 1 {
                return Err(Box::new(ProbeError::Phase("scale batch payload bytes")));
            }
            return Ok(ChunkBatch {
                positions: batch,
                payload_bytes,
                has_non_flat_surface,
            });
        }
        if packet.id != ids::play::LEVEL_CHUNK_WITH_LIGHT {
            return Err(Box::new(ProbeError::Phase("scale chunk id")));
        }
        payload_bytes += packet.data.len();
        batch.insert(chunk::level_chunk_pos(&packet.data)?);
        let decoded = DecodedChunk::from_packet(packet.data.clone())?;
        has_non_flat_surface |= decoded.has_non_flat_surface();
        chunk::validate_level_chunk_with_light(packet.data)?;
    }
}

async fn read_chunks(
    stream: &mut TcpStream,
    count: usize,
) -> Result<HashSet<(i32, i32)>, Box<dyn std::error::Error>> {
    let mut batch = HashSet::new();
    for _ in 0..count {
        let packet =
            super::expect(stream, ids::play::LEVEL_CHUNK_WITH_LIGHT, "scale chunk").await?;
        batch.insert(chunk::level_chunk_pos(&packet.data)?);
        chunk::validate_level_chunk_with_light(packet.data)?;
    }
    Ok(batch)
}

pub fn expect_radius_value(data: Vec<u8>, expected: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    if codec::read_var_i32(&mut cursor)? != expected {
        return Err(Box::new(ProbeError::Phase("scale chunk radius")));
    }
    Ok(())
}

pub async fn confirm_and_ack_keepalive(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut confirm = Vec::new();
    codec::write_var_i32(&mut confirm, play::Bootstrap::new(100).teleport_id());
    codec::write_packet(stream, ids::play::SERVERBOUND_TELEPORT_CONFIRM, &confirm).await?;
    let keepalive = super::expect(stream, ids::play::KEEPALIVE, "scale keepalive").await?;
    ack_keepalive(stream, keepalive.data).await
}

async fn ack_keepalive(
    stream: &mut TcpStream,
    data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    codec::write_packet(stream, ids::play::SERVERBOUND_KEEPALIVE, &data).await?;
    Ok(())
}

pub async fn expect_cache_center(
    stream: &mut TcpStream,
    x: i32,
    z: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = expect_cache_center_collecting_batches(stream, x, z).await?;
    Ok(())
}

pub async fn expect_cache_center_collecting_batches(
    stream: &mut TcpStream,
    x: i32,
    z: i32,
) -> Result<Vec<ChunkBatch>, Box<dyn std::error::Error>> {
    let mut skipped = Vec::new();
    loop {
        let packet = read_non_periodic(stream).await?;
        match packet.id {
            ids::play::CHUNK_CACHE_CENTER => {
                let mut cursor = Cursor::new(packet.data);
                if codec::read_var_i32(&mut cursor)? == x && codec::read_var_i32(&mut cursor)? == z
                {
                    return Ok(skipped);
                }
                return Err(Box::new(ProbeError::Phase("scale cache center payload")));
            }
            ids::play::CHUNK_BATCH_START => {
                skipped.push(read_batch_after_start(stream).await?);
            }
            id if ignorable_live_packet(id) => {}
            _ => return Err(packet_error("scale cache center", packet.id)),
        }
    }
}

pub async fn read_unload(stream: &mut TcpStream) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let packet = read_non_periodic(stream).await?;
    if packet.id != ids::play::UNLOAD_CHUNK {
        return Err(Box::new(ProbeError::Phase("scale unload id")));
    }
    let mut cursor = Cursor::new(packet.data);
    let z = codec::read_i32(&mut cursor)?;
    let x = codec::read_i32(&mut cursor)?;
    Ok((x, z))
}

async fn read_non_periodic(
    stream: &mut TcpStream,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    loop {
        let packet = codec::read_packet(stream).await?;
        match packet.id {
            ids::play::SET_TIME => {}
            ids::play::KEEPALIVE => ack_keepalive(stream, packet.data).await?,
            _ => return Ok(packet),
        }
    }
}

fn ignorable_live_packet(id: i32) -> bool {
    matches!(
        id,
        ids::play::SET_PLAYER_INVENTORY
            | ids::play::HELD_ITEM_SLOT
            | ids::play::UPDATE_HEALTH
            | ids::play::DEATH_COMBAT_EVENT
            | ids::play::UNLOAD_CHUNK
            | ids::play::CHUNK_CACHE_CENTER
            | ids::play::CHUNK_CACHE_RADIUS
    )
}

fn packet_error(phase: &'static str, id: i32) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::other(format!("{phase}: got 0x{id:x}")))
}
