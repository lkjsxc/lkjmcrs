use crate::probe::ProbeError;
use crate::probe::position::BlockPos;
use crate::protocol::{codec, ids};
use std::io::Cursor;
use tokio::net::TcpStream;

pub(super) async fn read_next_material_packet(
    stream: &mut TcpStream,
    phase: &'static str,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    loop {
        let packet = read_next_survival_packet(stream).await?;
        if !matches!(
            packet.id,
            ids::play::SET_TIME | ids::play::SET_PLAYER_INVENTORY | ids::play::HELD_ITEM_SLOT
        ) {
            return Ok(packet);
        }
        tracing::debug!(
            phase,
            "inventory or time packet skipped during survival probe"
        );
    }
}

pub(super) async fn read_next_survival_packet(
    stream: &mut TcpStream,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    loop {
        let packet = read_next_live_packet(stream).await?;
        if !is_chunk_stream_packet(packet.id) {
            return Ok(packet);
        }
    }
}

pub(super) async fn read_next_live_packet(
    stream: &mut TcpStream,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    loop {
        let packet = codec::read_packet(stream).await?;
        if packet.id == ids::play::KEEPALIVE {
            respond_keepalive(stream, packet.data).await?;
            continue;
        }
        if packet.id != ids::play::SET_TIME {
            return Ok(packet);
        }
    }
}

async fn respond_keepalive(
    stream: &mut TcpStream,
    data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = codec::read_i64(&mut Cursor::new(data))?;
    let mut response = Vec::new();
    codec::write_i64(&mut response, id);
    codec::write_packet(stream, ids::play::SERVERBOUND_KEEPALIVE, &response).await?;
    Ok(())
}

fn is_chunk_stream_packet(id: i32) -> bool {
    matches!(
        id,
        ids::play::CHUNK_BATCH_START
            | ids::play::CHUNK_BATCH_FINISHED
            | ids::play::LEVEL_CHUNK_WITH_LIGHT
            | ids::play::UNLOAD_CHUNK
            | ids::play::CHUNK_CACHE_CENTER
            | ids::play::CHUNK_CACHE_RADIUS
    )
}

pub(super) fn validate_update_state(
    data: Vec<u8>,
    pos: BlockPos,
    state: i32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let actual_pos = codec::read_position(&mut cursor)?;
    if actual_pos != (pos.x, pos.y, pos.z) {
        return Err(Box::new(std::io::Error::other(format!(
            "{phase}: expected position {},{},{} got {},{},{}",
            pos.x, pos.y, pos.z, actual_pos.0, actual_pos.1, actual_pos.2
        ))));
    }
    let actual_state = codec::read_var_i32(&mut cursor)?;
    if actual_state != state {
        return Err(Box::new(std::io::Error::other(format!(
            "{phase}: expected state {state} got {actual_state}"
        ))));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    Ok(())
}
