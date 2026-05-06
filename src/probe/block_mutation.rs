use crate::probe::ProbeError;
use crate::probe::item_entities;
use crate::protocol::{codec, ids};
use crate::world::BlockPos;
use std::io::Cursor;
use tokio::net::TcpStream;

const PLACE_SEQUENCE: i32 = 10;
const BREAK_SEQUENCE: i32 = 11;

pub(super) async fn place_and_break(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    acquire_dirt(stream, BlockPos::new(0, 79, 0), "smoke dirt").await?;
    send_use_item_on(stream, PLACE_SEQUENCE).await?;
    expect_ack_and_update(stream, PLACE_SEQUENCE, 10).await?;
    send_start_destroy(stream, BREAK_SEQUENCE).await?;
    expect_ack_and_update(stream, BREAK_SEQUENCE, 0).await?;
    item_entities::collect_drop(stream, 28, "smoke dirt cleanup").await?;
    Ok(())
}

pub(super) async fn acquire_dirt(
    stream: &mut TcpStream,
    grass: BlockPos,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    send_start_destroy_at(stream, 1000 + grass.x.abs() + grass.z.abs(), grass).await?;
    expect_ack_and_update_at(stream, 1000 + grass.x.abs() + grass.z.abs(), grass, 0).await?;
    item_entities::collect_drop_at(
        stream,
        28,
        phase,
        f64::from(grass.x) + 0.5,
        f64::from(grass.y) + 1.0,
        f64::from(grass.z) + 0.5,
    )
    .await?;
    Ok(())
}

pub(super) async fn send_use_item_on(
    stream: &mut TcpStream,
    sequence: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    send_use_item_on_at(stream, sequence, BlockPos::new(0, 79, 0)).await
}

pub(super) async fn send_use_item_on_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_var_i32(&mut payload, 0);
    codec::write_position(&mut payload, pos.x, pos.y, pos.z);
    codec::write_var_i32(&mut payload, 1);
    codec::write_f32(&mut payload, 0.5);
    codec::write_f32(&mut payload, 1.0);
    codec::write_f32(&mut payload, 0.5);
    codec::write_bool(&mut payload, false);
    codec::write_bool(&mut payload, false);
    codec::write_var_i32(&mut payload, sequence);
    codec::write_packet(stream, ids::play::SERVERBOUND_USE_ITEM_ON, &payload).await?;
    Ok(())
}

pub(super) async fn send_start_destroy(
    stream: &mut TcpStream,
    sequence: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    send_start_destroy_at(stream, sequence, BlockPos::new(0, 80, 0)).await
}

pub(super) async fn send_start_destroy_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_var_i32(&mut payload, 0);
    codec::write_position(&mut payload, pos.x, pos.y, pos.z);
    codec::write_u8(&mut payload, 1);
    codec::write_var_i32(&mut payload, sequence);
    codec::write_packet(stream, ids::play::SERVERBOUND_PLAYER_ACTION, &payload).await?;
    Ok(())
}

pub(super) async fn expect_ack_and_update(
    stream: &mut TcpStream,
    sequence: i32,
    block_state: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let ack = read_next_non_time(stream, "block mutation ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("block mutation ack id")));
    }
    validate_ack(ack.data, sequence)?;
    let update = read_next_non_time(stream, "block mutation update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("block mutation update id")));
    }
    validate_update(update.data, block_state)
}

pub(super) async fn read_next_non_time(
    stream: &mut TcpStream,
    phase: &'static str,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    loop {
        let packet = codec::read_packet(stream).await?;
        if !matches!(
            packet.id,
            ids::play::SET_TIME | ids::play::SET_PLAYER_INVENTORY | ids::play::HELD_ITEM_SLOT
        ) {
            return Ok(packet);
        }
        tracing::debug!(phase, "periodic or inventory packet skipped");
    }
}

pub(super) fn validate_ack(data: Vec<u8>, sequence: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    if codec::read_var_i32(&mut cursor)? != sequence {
        return Err(Box::new(ProbeError::Phase("block mutation ack sequence")));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("block mutation ack trailing")));
    }
    Ok(())
}

pub(super) fn validate_update(data: Vec<u8>, state: i32) -> Result<(), Box<dyn std::error::Error>> {
    validate_update_at(data, BlockPos::new(0, 80, 0), state)
}

pub(super) fn validate_update_at(
    data: Vec<u8>,
    pos: BlockPos,
    state: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    if codec::read_position(&mut cursor)? != (pos.x, pos.y, pos.z) {
        return Err(Box::new(ProbeError::Phase("block mutation update pos")));
    }
    let actual = codec::read_var_i32(&mut cursor)?;
    if actual != state {
        return Err(Box::new(std::io::Error::other(format!(
            "block mutation update state: expected {state}, got {actual}"
        ))));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase(
            "block mutation update trailing",
        )));
    }
    Ok(())
}

pub(super) async fn expect_ack_and_update_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
    block_state: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let ack = read_next_non_time(stream, "block mutation ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("block mutation ack id")));
    }
    validate_ack(ack.data, sequence)?;
    let update = read_next_non_time(stream, "block mutation update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("block mutation update id")));
    }
    validate_update_at(update.data, pos, block_state)
}
