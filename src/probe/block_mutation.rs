use crate::probe::ProbeError;
use crate::probe::item_entities;
use crate::probe::position::BlockPos;
use crate::probe::survival_expect;
use crate::protocol::{codec, ids};
use std::io::Cursor;
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

const DIRT_BREAK_WAIT: Duration = Duration::from_millis(850);

pub(super) async fn acquire_dirt_from(
    stream: &mut TcpStream,
    pos: BlockPos,
    current_state: i32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    mine_dirt_like_at(
        stream,
        1000 + pos.x.abs() + pos.z.abs(),
        pos,
        current_state,
        0,
    )
    .await?;
    item_entities::collect_drop_at(
        stream,
        28,
        phase,
        f64::from(pos.x) + 0.5,
        f64::from(pos.y) + 1.0,
        f64::from(pos.z) + 0.5,
    )
    .await?;
    Ok(())
}

pub(super) async fn mine_dirt_like_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
    current_state: i32,
    final_state: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    mine_block_at(
        stream,
        sequence,
        pos,
        current_state,
        final_state,
        DIRT_BREAK_WAIT,
    )
    .await
}

async fn mine_block_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
    current_state: i32,
    final_state: i32,
    wait: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    send_start_destroy_at(stream, sequence, pos).await?;
    expect_ack_and_update_at(stream, sequence, pos, current_state).await?;
    sleep(wait).await;
    send_stop_destroy_at(stream, sequence, pos).await?;
    expect_ack_and_update_at(stream, sequence, pos, final_state).await
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

async fn send_start_destroy_at(
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

async fn send_stop_destroy_at(
    stream: &mut TcpStream,
    sequence: i32,
    pos: BlockPos,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_var_i32(&mut payload, 2);
    codec::write_position(&mut payload, pos.x, pos.y, pos.z);
    codec::write_u8(&mut payload, 1);
    codec::write_var_i32(&mut payload, sequence);
    codec::write_packet(stream, ids::play::SERVERBOUND_PLAYER_ACTION, &payload).await?;
    Ok(())
}

pub(super) async fn read_next_non_time(
    stream: &mut TcpStream,
    phase: &'static str,
) -> Result<codec::Packet, Box<dyn std::error::Error>> {
    survival_expect::read_next_material_packet(stream, phase).await
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
    let ack = survival_expect::read_next_material_packet(stream, "block mutation ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("block mutation ack id")));
    }
    validate_ack(ack.data, sequence)?;
    let update =
        survival_expect::read_next_material_packet(stream, "block mutation update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("block mutation update id")));
    }
    validate_update_at(update.data, pos, block_state)
}
