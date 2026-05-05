use crate::protocol::codec::CodecError;
use std::io::Cursor;
use uuid::Uuid;

pub fn read_var_i32(cursor: &mut Cursor<Vec<u8>>) -> Result<i32, CodecError> {
    let mut value = 0i32;
    for position in 0..5 {
        let byte = read_u8(cursor)?;
        value |= ((byte & 0x7f) as i32) << (position * 7);
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(CodecError::VarIntTooLarge)
}

pub fn write_var_i32(out: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value = ((value as u32) >> 7) as i32;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

pub fn read_string(cursor: &mut Cursor<Vec<u8>>) -> Result<String, CodecError> {
    let length = read_var_i32(cursor)?;
    if length < 0 {
        return Err(CodecError::NegativeLength);
    }
    let mut bytes = vec![0; length as usize];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    String::from_utf8(bytes).map_err(|_| CodecError::Utf8)
}

pub fn write_string(out: &mut Vec<u8>, value: &str) {
    write_var_i32(out, value.len() as i32);
    out.extend_from_slice(value.as_bytes());
}

pub fn read_uuid(cursor: &mut Cursor<Vec<u8>>) -> Result<Uuid, CodecError> {
    let mut bytes = [0; 16];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(Uuid::from_bytes(bytes))
}

pub fn write_uuid(out: &mut Vec<u8>, uuid: Uuid) {
    out.extend_from_slice(uuid.as_bytes());
}

pub fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> Result<u8, CodecError> {
    let mut bytes = [0; 1];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(bytes[0])
}

pub fn write_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

pub fn write_i8(out: &mut Vec<u8>, value: i8) {
    out.push(value as u8);
}

pub fn write_i32(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn write_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn read_i64(cursor: &mut Cursor<Vec<u8>>) -> Result<i64, CodecError> {
    let mut bytes = [0; 8];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(i64::from_be_bytes(bytes))
}

pub fn write_i64(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn read_u16(cursor: &mut Cursor<Vec<u8>>) -> Result<u16, CodecError> {
    let mut bytes = [0; 2];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(u16::from_be_bytes(bytes))
}

pub fn write_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn read_bool(cursor: &mut Cursor<Vec<u8>>) -> Result<bool, CodecError> {
    match read_u8(cursor)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(CodecError::Eof),
    }
}

pub fn write_bool(out: &mut Vec<u8>, value: bool) {
    out.push(u8::from(value));
}

pub fn read_f32(cursor: &mut Cursor<Vec<u8>>) -> Result<f32, CodecError> {
    let mut bytes = [0; 4];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(f32::from_be_bytes(bytes))
}

pub fn write_f32(out: &mut Vec<u8>, value: f32) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn read_f64(cursor: &mut Cursor<Vec<u8>>) -> Result<f64, CodecError> {
    let mut bytes = [0; 8];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(f64::from_be_bytes(bytes))
}

pub fn write_f64(out: &mut Vec<u8>, value: f64) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn write_position(out: &mut Vec<u8>, x: i32, y: i32, z: i32) {
    let value = ((i64::from(x) & 0x3ffffff) << 38)
        | ((i64::from(z) & 0x3ffffff) << 12)
        | (i64::from(y) & 0xfff);
    out.extend_from_slice(&(value as u64).to_be_bytes());
}
