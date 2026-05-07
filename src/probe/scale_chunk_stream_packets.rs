use crate::probe::ProbeError;
use crate::probe::chunk;
use crate::probe::validation::validate_chunk_batch_finished;
use crate::protocol::{codec, ids, play};
use std::collections::HashSet;
use std::io::Cursor;
use tokio::net::TcpStream;

pub const RADIUS: i32 = 4;
pub const MAX_BATCH: usize = 8;

pub async fn read_next_batch(
    stream: &mut TcpStream,
) -> Result<HashSet<(i32, i32)>, Box<dyn std::error::Error>> {
    loop {
        let packet = codec::read_packet(stream).await?;
        match packet.id {
            ids::play::CHUNK_BATCH_START => return read_batch_after_start(stream).await,
            ids::play::SET_TIME => {}
            ids::play::KEEPALIVE => ack_keepalive(stream, packet.data).await?,
            _ => return Err(Box::new(ProbeError::Phase("scale stream packet"))),
        }
    }
}

pub async fn expect_batch(
    stream: &mut TcpStream,
    expected: usize,
) -> Result<HashSet<(i32, i32)>, Box<dyn std::error::Error>> {
    super::expect(stream, ids::play::CHUNK_BATCH_START, "scale batch start").await?;
    let batch = read_chunks(stream, expected).await?;
    let finished =
        super::expect(stream, ids::play::CHUNK_BATCH_FINISHED, "scale batch end").await?;
    validate_chunk_batch_finished(finished.data, expected)?;
    Ok(batch)
}

async fn read_batch_after_start(
    stream: &mut TcpStream,
) -> Result<HashSet<(i32, i32)>, Box<dyn std::error::Error>> {
    let mut batch = HashSet::new();
    loop {
        let packet = codec::read_packet(stream).await?;
        if packet.id == ids::play::CHUNK_BATCH_FINISHED {
            validate_chunk_batch_finished(packet.data, batch.len())?;
            if batch.is_empty() || batch.len() > MAX_BATCH {
                return Err(Box::new(ProbeError::Phase("scale batch size")));
            }
            return Ok(batch);
        }
        if packet.id != ids::play::LEVEL_CHUNK_WITH_LIGHT {
            return Err(Box::new(ProbeError::Phase("scale chunk id")));
        }
        batch.insert(chunk::level_chunk_pos(&packet.data)?);
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

pub fn expect_radius(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    if codec::read_var_i32(&mut cursor)? != RADIUS {
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
    let packet = codec::read_packet(stream).await?;
    if packet.id != ids::play::CHUNK_CACHE_CENTER {
        return Err(Box::new(ProbeError::Phase("scale cache center")));
    }
    let mut cursor = Cursor::new(packet.data);
    if codec::read_var_i32(&mut cursor)? != x || codec::read_var_i32(&mut cursor)? != z {
        return Err(Box::new(ProbeError::Phase("scale cache center payload")));
    }
    Ok(())
}

pub async fn read_unload(stream: &mut TcpStream) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let packet = codec::read_packet(stream).await?;
    if packet.id != ids::play::UNLOAD_CHUNK {
        return Err(Box::new(ProbeError::Phase("scale unload id")));
    }
    let mut cursor = Cursor::new(packet.data);
    let z = codec::read_i32(&mut cursor)?;
    let x = codec::read_i32(&mut cursor)?;
    Ok((x, z))
}
