use crate::probe::ProbeError;
use crate::protocol::configuration::{self, KnownPack};
use crate::protocol::{MINECRAFT_VERSION, PROTOCOL_VERSION, codec, play};
use std::io::Cursor;

pub(super) fn validate_status_json(json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let version = &value["version"];
    if version["name"] != MINECRAFT_VERSION || version["protocol"] != PROTOCOL_VERSION {
        return Err(Box::new(ProbeError::Phase("status version")));
    }
    Ok(())
}

pub(super) fn validate_known_packs(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let packs = configuration::decode_known_packs(data)?;
    if packs != vec![KnownPack::vanilla_core()] {
        return Err(Box::new(ProbeError::Phase("known packs payload")));
    }
    Ok(())
}

pub(super) fn validate_login_success(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let _uuid = codec::read_uuid(&mut cursor)?;
    let username = codec::read_string(&mut cursor)?;
    let properties = codec::read_var_i32(&mut cursor)?;
    if username != "Probe" || properties != 0 {
        return Err(Box::new(ProbeError::Phase("login success payload")));
    }
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("login success trailing bytes")));
    }
    Ok(())
}

pub(super) fn validate_position_packet(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let teleport_id = codec::read_var_i32(&mut cursor)?;
    if teleport_id != play::Bootstrap::new(100).teleport_id() {
        return Err(Box::new(ProbeError::Phase("teleport id")));
    }
    Ok(())
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
