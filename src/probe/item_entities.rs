use crate::probe::ProbeError;
use crate::probe::inventory_packets::PlayerInventorySlot;
use crate::probe::{inventory_packets, live_play, survival_expect};
use crate::protocol::entity::ITEM_ENTITY_TYPE_ID;
use crate::protocol::{codec, ids};
use std::io::Cursor;
use tokio::net::TcpStream;

pub(super) async fn collect_drop(
    stream: &mut TcpStream,
    item_id: i32,
    phase: &'static str,
) -> Result<PlayerInventorySlot, Box<dyn std::error::Error>> {
    collect_drop_at(stream, item_id, phase, 0.5, 80.0, 0.5).await
}

pub(super) async fn collect_drop_at(
    stream: &mut TcpStream,
    item_id: i32,
    phase: &'static str,
    x: f64,
    y: f64,
    z: f64,
) -> Result<PlayerInventorySlot, Box<dyn std::error::Error>> {
    let entity_id = expect_spawn(stream, phase).await?;
    expect_metadata(stream, entity_id, item_id, phase).await?;
    live_play::send_position_look_at(stream, x, y, z, 0.0, 0.0).await?;
    expect_collect(stream, entity_id, phase).await?;
    expect_destroy(stream, entity_id, phase).await?;
    expect_inventory_delta(stream, item_id, phase).await
}

async fn expect_spawn(
    stream: &mut TcpStream,
    _phase: &'static str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let packet = survival_expect::read_next_live_packet(stream).await?;
    if packet.id != ids::play::SPAWN_ENTITY {
        return Err(Box::new(ProbeError::Phase("item spawn id")));
    }
    let mut cursor = Cursor::new(packet.data);
    let entity_id = codec::read_var_i32(&mut cursor)?;
    let _uuid = codec::read_uuid(&mut cursor)?;
    if codec::read_var_i32(&mut cursor)? != ITEM_ENTITY_TYPE_ID {
        return Err(Box::new(ProbeError::Phase("item entity type")));
    }
    for _ in 0..3 {
        let _ = codec::read_f64(&mut cursor)?;
    }
    if codec::read_u8(&mut cursor)? != 0 {
        return Err(Box::new(ProbeError::Phase("item zero velocity")));
    }
    for _ in 0..3 {
        let _ = codec::read_u8(&mut cursor)?;
    }
    if codec::read_var_i32(&mut cursor)? != 0 {
        return Err(Box::new(ProbeError::Phase("item object data")));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("item spawn trailing bytes")));
    }
    Ok(entity_id)
}

async fn expect_metadata(
    stream: &mut TcpStream,
    entity_id: i32,
    item_id: i32,
    _phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet = survival_expect::read_next_live_packet(stream).await?;
    if packet.id != ids::play::ENTITY_METADATA {
        return Err(Box::new(ProbeError::Phase("item metadata id")));
    }
    let mut cursor = Cursor::new(packet.data);
    if codec::read_var_i32(&mut cursor)? != entity_id || codec::read_u8(&mut cursor)? != 8 {
        return Err(Box::new(ProbeError::Phase("item metadata header")));
    }
    if codec::read_var_i32(&mut cursor)? != 7 || codec::read_var_i32(&mut cursor)? != 1 {
        return Err(Box::new(ProbeError::Phase("item metadata slot")));
    }
    if codec::read_var_i32(&mut cursor)? != item_id {
        return Err(Box::new(ProbeError::Phase("item metadata item")));
    }
    if codec::read_var_i32(&mut cursor)? != 0 || codec::read_var_i32(&mut cursor)? != 0 {
        return Err(Box::new(ProbeError::Phase("item metadata components")));
    }
    if codec::read_u8(&mut cursor)? != 0xff {
        return Err(Box::new(ProbeError::Phase("item metadata terminator")));
    }
    Ok(())
}

async fn expect_collect(
    stream: &mut TcpStream,
    entity_id: i32,
    _phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet = survival_expect::read_next_live_packet(stream).await?;
    if packet.id != ids::play::COLLECT {
        return Err(Box::new(ProbeError::Phase("collect id")));
    }
    let mut cursor = Cursor::new(packet.data);
    if codec::read_var_i32(&mut cursor)? != entity_id || codec::read_var_i32(&mut cursor)? != 1 {
        return Err(Box::new(ProbeError::Phase("collect payload")));
    }
    if codec::read_var_i32(&mut cursor)? != 1 {
        return Err(Box::new(ProbeError::Phase("collect count")));
    }
    Ok(())
}

async fn expect_destroy(
    stream: &mut TcpStream,
    entity_id: i32,
    _phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let packet = survival_expect::read_next_live_packet(stream).await?;
    if packet.id != ids::play::ENTITY_DESTROY {
        return Err(Box::new(ProbeError::Phase("destroy id")));
    }
    let mut cursor = Cursor::new(packet.data);
    if codec::read_var_i32(&mut cursor)? != 1 || codec::read_var_i32(&mut cursor)? != entity_id {
        return Err(Box::new(ProbeError::Phase("destroy payload")));
    }
    Ok(())
}

async fn expect_inventory_delta(
    stream: &mut TcpStream,
    item_id: i32,
    _phase: &'static str,
) -> Result<PlayerInventorySlot, Box<dyn std::error::Error>> {
    let packet = survival_expect::read_next_live_packet(stream).await?;
    if packet.id != ids::play::SET_PLAYER_INVENTORY {
        return Err(Box::new(ProbeError::Phase("pickup inventory id")));
    }
    let slot = inventory_packets::decode_player_inventory_slot(packet.data)?;
    if slot.item_id != Some(item_id) {
        return Err(Box::new(ProbeError::Phase("pickup inventory item")));
    }
    Ok(slot)
}
