use crate::probe::ProbeError;
use crate::protocol::configuration::{self, KnownPack};
use crate::protocol::{codec, play};
use std::io::Cursor;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) struct PositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct LoginPacket {
    pub game_mode: i8,
    pub view_distance: i32,
}

pub(super) fn validate_known_packs(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let packs = configuration::decode_known_packs(data)?;
    if packs != vec![KnownPack::vanilla_core()] {
        return Err(Box::new(ProbeError::Phase("known packs payload")));
    }
    Ok(())
}

pub(super) fn validate_login_success(
    data: Vec<u8>,
    expected_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let _uuid = codec::read_uuid(&mut cursor)?;
    let username = codec::read_string(&mut cursor)?;
    let properties = codec::read_var_i32(&mut cursor)?;
    if username != expected_name || properties != 0 {
        return Err(Box::new(ProbeError::Phase("login success payload")));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("login success trailing bytes")));
    }
    Ok(())
}

pub(super) fn decode_login_success_uuid(data: Vec<u8>) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let uuid = codec::read_uuid(&mut cursor)?;
    Ok(uuid)
}

pub(super) fn decode_position_packet(
    data: Vec<u8>,
) -> Result<PositionPacket, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let teleport_id = codec::read_var_i32(&mut cursor)?;
    if teleport_id != play::Bootstrap::new(100).teleport_id() {
        return Err(Box::new(ProbeError::Phase("teleport id")));
    }
    let x = codec::read_f64(&mut cursor)?;
    let y = codec::read_f64(&mut cursor)?;
    let z = codec::read_f64(&mut cursor)?;
    for _ in 0..3 {
        codec::read_f64(&mut cursor)?;
    }
    let yaw = codec::read_f32(&mut cursor)?;
    let pitch = codec::read_f32(&mut cursor)?;
    Ok(PositionPacket {
        x,
        y,
        z,
        yaw,
        pitch,
    })
}

pub(super) fn decode_login_packet(
    data: Vec<u8>,
) -> Result<LoginPacket, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let _entity_id = codec::read_i32(&mut cursor)?;
    let _hardcore = codec::read_bool(&mut cursor)?;
    let dimension_count = codec::read_var_i32(&mut cursor)?;
    for _ in 0..dimension_count {
        let _dimension = codec::read_string(&mut cursor)?;
    }
    let _max_players = codec::read_var_i32(&mut cursor)?;
    let view_distance = codec::read_var_i32(&mut cursor)?;
    let _simulation_distance = codec::read_var_i32(&mut cursor)?;
    for _ in 0..3 {
        let _ = codec::read_bool(&mut cursor)?;
    }
    let _dimension_type = codec::read_var_i32(&mut cursor)?;
    let _dimension = codec::read_string(&mut cursor)?;
    let _seed = codec::read_i64(&mut cursor)?;
    let game_mode = codec::read_u8(&mut cursor)? as i8;
    Ok(LoginPacket {
        game_mode,
        view_distance,
    })
}

pub(super) fn validate_chunk_radius(data: Vec<u8>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let radius = codec::read_var_i32(&mut cursor)?;
    if radius != play::Bootstrap::new(100).view_distance {
        return Err(Box::new(ProbeError::Phase("chunk radius payload")));
    }
    Ok(play::chunk_count_for_radius(radius))
}

pub(super) fn validate_game_state_change(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    if data != play::encode_start_waiting_for_chunks() {
        return Err(Box::new(ProbeError::Phase("chunk readiness payload")));
    }
    Ok(())
}

pub(super) fn validate_chunk_batch_finished(
    data: Vec<u8>,
    expected: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    if codec::read_var_i32(&mut cursor)? != expected as i32 {
        return Err(Box::new(ProbeError::Phase("chunk batch size")));
    }
    Ok(())
}
