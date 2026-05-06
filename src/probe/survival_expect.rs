use crate::probe::ProbeError;
use crate::protocol::{codec, ids};
use crate::world::BlockPos;
use std::io::Cursor;
use tokio::net::TcpStream;

pub(super) async fn read_next_material_packet(
    stream: &mut TcpStream,
    phase: &'static str,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    loop {
        let packet = codec::read_packet(stream).await?;
        if packet.id == ids::play::KEEPALIVE {
            respond_keepalive(stream, packet.data).await?;
            continue;
        }
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

pub(super) fn validate_update_state(
    data: Vec<u8>,
    pos: BlockPos,
    state: i32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    if codec::read_position(&mut cursor)? != (pos.x, pos.y, pos.z) {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    if codec::read_var_i32(&mut cursor)? != state {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    Ok(())
}
