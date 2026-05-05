use std::io::Cursor;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("varint is too large")]
    VarIntTooLarge,
    #[error("utf8 string is invalid")]
    Utf8,
    #[error("packet ended early")]
    Eof,
    #[error("negative packet length")]
    NegativeLength,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

pub async fn read_packet<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Packet, CodecError> {
    let length = read_var_i32_async(reader).await?;
    if length < 0 {
        return Err(CodecError::NegativeLength);
    }
    let mut frame = vec![0; length as usize];
    reader.read_exact(&mut frame).await?;
    let mut cursor = Cursor::new(frame);
    let id = read_var_i32(&mut cursor)?;
    let mut data = Vec::new();
    std::io::Read::read_to_end(&mut cursor, &mut data)?;
    Ok(Packet { id, data })
}

pub async fn write_packet<W: AsyncWrite + Unpin>(
    writer: &mut W,
    id: i32,
    payload: &[u8],
) -> Result<(), CodecError> {
    let mut body = Vec::new();
    write_var_i32(&mut body, id);
    body.extend_from_slice(payload);
    let mut frame = Vec::new();
    write_var_i32(&mut frame, body.len() as i32);
    frame.extend_from_slice(&body);
    writer.write_all(&frame).await?;
    Ok(())
}

pub async fn read_var_i32_async<R: AsyncRead + Unpin>(reader: &mut R) -> Result<i32, CodecError> {
    let mut value = 0i32;
    for position in 0..5 {
        let byte = reader.read_u8().await?;
        value |= ((byte & 0x7f) as i32) << (position * 7);
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(CodecError::VarIntTooLarge)
}

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

pub fn write_i64(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn read_i64(cursor: &mut Cursor<Vec<u8>>) -> Result<i64, CodecError> {
    let mut bytes = [0; 8];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(i64::from_be_bytes(bytes))
}

pub fn write_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn read_u16(cursor: &mut Cursor<Vec<u8>>) -> Result<u16, CodecError> {
    let mut bytes = [0; 2];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(u16::from_be_bytes(bytes))
}

pub fn write_bool(out: &mut Vec<u8>, value: bool) {
    out.push(u8::from(value));
}

fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> Result<u8, CodecError> {
    let mut bytes = [0; 1];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| CodecError::Eof)?;
    Ok(bytes[0])
}

#[cfg(test)]
mod tests {
    use super::{read_var_i32, write_var_i32};
    use std::io::Cursor;

    #[test]
    fn varint_round_trip_boundaries() {
        for value in [0, 1, 127, 128, 255, 2_147_483_647] {
            let mut bytes = Vec::new();
            write_var_i32(&mut bytes, value);
            assert_eq!(read_var_i32(&mut Cursor::new(bytes)).unwrap(), value);
        }
    }
}
