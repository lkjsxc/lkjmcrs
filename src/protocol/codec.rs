use std::io::Cursor;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

mod primitives;
pub use primitives::*;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("connection closed")]
    ConnectionClosed,
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
    reader
        .read_exact(&mut frame)
        .await
        .map_err(read_error_to_codec)?;
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
        let byte = reader.read_u8().await.map_err(read_error_to_codec)?;
        value |= ((byte & 0x7f) as i32) << (position * 7);
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(CodecError::VarIntTooLarge)
}

pub(crate) fn read_error_to_codec(error: std::io::Error) -> CodecError {
    match error.kind() {
        std::io::ErrorKind::UnexpectedEof | std::io::ErrorKind::ConnectionReset => {
            CodecError::ConnectionClosed
        }
        _ => CodecError::Io(error),
    }
}
