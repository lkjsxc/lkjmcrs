use crate::protocol::codec::{self, CodecError};
use std::io::Cursor;

pub fn decode_action(data: Vec<u8>) -> Result<i32, CodecError> {
    let mut cursor = Cursor::new(data);
    let action = codec::read_var_i32(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(CodecError::Eof);
    }
    Ok(action)
}
