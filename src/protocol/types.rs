use crate::protocol::codec::{self, CodecError};
use std::io::{Cursor, Read};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextState {
    Status,
    Login,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake {
    pub protocol: i32,
    pub address: String,
    pub port: u16,
    pub next_state: NextState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginStart {
    pub name: String,
    pub profile_id: Option<Uuid>,
}

impl Handshake {
    pub fn decode(data: Vec<u8>) -> Result<Self, CodecError> {
        let mut cursor = Cursor::new(data);
        let protocol = codec::read_var_i32(&mut cursor)?;
        let address = codec::read_string(&mut cursor)?;
        let port = codec::read_u16(&mut cursor)?;
        let next_state = match codec::read_var_i32(&mut cursor)? {
            1 => NextState::Status,
            2 => NextState::Login,
            _ => return Err(CodecError::Eof),
        };
        Ok(Self {
            protocol,
            address,
            port,
            next_state,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        codec::write_var_i32(&mut out, self.protocol);
        codec::write_string(&mut out, &self.address);
        codec::write_u16(&mut out, self.port);
        codec::write_var_i32(&mut out, self.next_state.wire_value());
        out
    }
}

impl From<NextState> for i32 {
    fn from(value: NextState) -> Self {
        match value {
            NextState::Status => 1,
            NextState::Login => 2,
        }
    }
}

impl LoginStart {
    pub fn decode(data: Vec<u8>) -> Result<Self, CodecError> {
        let mut cursor = Cursor::new(data);
        let name = codec::read_string(&mut cursor)?;
        let profile_id = if cursor.position() < cursor.get_ref().len() as u64 {
            Some(codec::read_uuid(&mut cursor)?)
        } else {
            None
        };
        Ok(Self { name, profile_id })
    }

    pub fn encode(name: &str, profile_id: Uuid) -> Vec<u8> {
        let mut out = Vec::new();
        codec::write_string(&mut out, name);
        codec::write_uuid(&mut out, profile_id);
        out
    }
}

impl NextState {
    pub const fn wire_value(self) -> i32 {
        match self {
            Self::Status => 1,
            Self::Login => 2,
        }
    }
}

pub fn has_remaining(cursor: &mut Cursor<Vec<u8>>) -> bool {
    cursor.position() < cursor.get_ref().len() as u64
}

pub fn remaining_bytes(cursor: &mut Cursor<Vec<u8>>) -> Result<Vec<u8>, CodecError> {
    let mut data = Vec::new();
    cursor.read_to_end(&mut data)?;
    Ok(data)
}
