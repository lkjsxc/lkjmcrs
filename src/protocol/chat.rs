use crate::protocol::codec::{self, CodecError};
use crate::protocol::ids;
use crate::protocol::nbt;
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayChat {
    Message(String),
    Command(String),
    HeldSlot(i16),
}

pub fn decode(id: i32, data: Vec<u8>) -> Result<Option<PlayChat>, CodecError> {
    let decoded = match id {
        ids::play::SERVERBOUND_CHAT_MESSAGE => Some(PlayChat::Message(decode_message(data)?)),
        ids::play::SERVERBOUND_CHAT_COMMAND => Some(PlayChat::Command(decode_command(data)?)),
        ids::play::SERVERBOUND_CHAT_COMMAND_SIGNED => {
            Some(PlayChat::Command(decode_signed_command(data)?))
        }
        ids::play::SERVERBOUND_HELD_ITEM_SLOT => Some(PlayChat::HeldSlot(decode_held_slot(data)?)),
        _ => None,
    };
    Ok(decoded)
}

pub fn encode_system_chat(message: &str) -> Vec<u8> {
    let mut out = Vec::new();
    write_text_component(&mut out, message);
    codec::write_bool(&mut out, false);
    out
}

pub fn encode_kick(reason: &str) -> Vec<u8> {
    let mut out = Vec::new();
    write_text_component(&mut out, reason);
    out
}

fn decode_command(data: Vec<u8>) -> Result<String, CodecError> {
    let mut cursor = Cursor::new(data);
    let command = codec::read_string(&mut cursor)?;
    reject_trailing(&mut cursor)?;
    Ok(command)
}

fn decode_signed_command(data: Vec<u8>) -> Result<String, CodecError> {
    let mut cursor = Cursor::new(data);
    let command = codec::read_string(&mut cursor)?;
    let _timestamp = codec::read_i64(&mut cursor)?;
    let _salt = codec::read_i64(&mut cursor)?;
    for _ in 0..codec::read_var_i32(&mut cursor)? {
        let _name = codec::read_string(&mut cursor)?;
        skip_bytes(&mut cursor, 256)?;
    }
    let _message_count = codec::read_var_i32(&mut cursor)?;
    skip_bytes(&mut cursor, 3)?;
    let _checksum = codec::read_u8(&mut cursor)?;
    reject_trailing(&mut cursor)?;
    Ok(command)
}

fn decode_message(data: Vec<u8>) -> Result<String, CodecError> {
    let mut cursor = Cursor::new(data);
    let message = codec::read_string(&mut cursor)?;
    let _timestamp = codec::read_i64(&mut cursor)?;
    let _salt = codec::read_i64(&mut cursor)?;
    if codec::read_bool(&mut cursor)? {
        skip_bytes(&mut cursor, 256)?;
    }
    let _offset = codec::read_var_i32(&mut cursor)?;
    skip_bytes(&mut cursor, 3)?;
    let _checksum = codec::read_u8(&mut cursor)?;
    reject_trailing(&mut cursor)?;
    Ok(message)
}

fn decode_held_slot(data: Vec<u8>) -> Result<i16, CodecError> {
    let mut cursor = Cursor::new(data);
    let slot = codec::read_i16(&mut cursor)?;
    reject_trailing(&mut cursor)?;
    Ok(slot)
}

fn write_text_component(out: &mut Vec<u8>, text: &str) {
    nbt::write_anonymous_compound(out, &nbt::compound(vec![("text", nbt::string(text))]));
}

fn skip_bytes(cursor: &mut Cursor<Vec<u8>>, len: usize) -> Result<(), CodecError> {
    let mut bytes = vec![0; len];
    cursor.read_exact(&mut bytes).map_err(|_| CodecError::Eof)
}

fn reject_trailing(cursor: &mut Cursor<Vec<u8>>) -> Result<(), CodecError> {
    if cursor.position() == cursor.get_ref().len() as u64 {
        Ok(())
    } else {
        Err(CodecError::Eof)
    }
}

#[cfg(test)]
mod tests {
    use super::{PlayChat, decode, encode_system_chat};
    use crate::protocol::{codec, ids};

    #[test]
    fn decodes_unsigned_chat_message() {
        let mut data = Vec::new();
        codec::write_string(&mut data, "hello");
        codec::write_i64(&mut data, 0);
        codec::write_i64(&mut data, 0);
        codec::write_bool(&mut data, false);
        codec::write_var_i32(&mut data, 0);
        data.extend_from_slice(&[0; 3]);
        codec::write_u8(&mut data, 0);

        assert_eq!(
            decode(ids::play::SERVERBOUND_CHAT_MESSAGE, data).unwrap(),
            Some(PlayChat::Message("hello".to_string()))
        );
    }

    #[test]
    fn system_chat_contains_text_nbt() {
        let payload = encode_system_chat("hello");
        assert_eq!(payload[0], 10);
        assert!(String::from_utf8_lossy(&payload).contains("hello"));
    }
}
