use crate::protocol::codec::{self, CodecError};
use crate::protocol::ids;
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Movement {
    Position {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        horizontal_collision: bool,
    },
    PositionLook {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        horizontal_collision: bool,
    },
    Look {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        horizontal_collision: bool,
    },
    Flying {
        on_ground: bool,
        horizontal_collision: bool,
    },
}

impl Movement {
    pub fn decode(packet_id: i32, data: Vec<u8>) -> Result<Option<Self>, CodecError> {
        let mut cursor = Cursor::new(data);
        let movement = match packet_id {
            ids::play::SERVERBOUND_POSITION => Some(Self::decode_position(&mut cursor)?),
            ids::play::SERVERBOUND_POSITION_LOOK => Some(Self::decode_position_look(&mut cursor)?),
            ids::play::SERVERBOUND_LOOK => Some(Self::decode_look(&mut cursor)?),
            ids::play::SERVERBOUND_FLYING => Some(Self::decode_flying(&mut cursor)?),
            _ => None,
        };
        if movement.is_some() && cursor.position() != cursor.get_ref().len() as u64 {
            return Err(CodecError::Eof);
        }
        Ok(movement)
    }

    fn decode_position(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        Ok(Self::Position {
            x: codec::read_f64(cursor)?,
            y: codec::read_f64(cursor)?,
            z: codec::read_f64(cursor)?,
            on_ground: codec::read_bool(cursor)?,
            horizontal_collision: codec::read_bool(cursor)?,
        })
    }

    fn decode_position_look(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        Ok(Self::PositionLook {
            x: codec::read_f64(cursor)?,
            y: codec::read_f64(cursor)?,
            z: codec::read_f64(cursor)?,
            yaw: codec::read_f32(cursor)?,
            pitch: codec::read_f32(cursor)?,
            on_ground: codec::read_bool(cursor)?,
            horizontal_collision: codec::read_bool(cursor)?,
        })
    }

    fn decode_look(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        Ok(Self::Look {
            yaw: codec::read_f32(cursor)?,
            pitch: codec::read_f32(cursor)?,
            on_ground: codec::read_bool(cursor)?,
            horizontal_collision: codec::read_bool(cursor)?,
        })
    }

    fn decode_flying(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        Ok(Self::Flying {
            on_ground: codec::read_bool(cursor)?,
            horizontal_collision: codec::read_bool(cursor)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Movement;
    use crate::protocol::{codec, ids};

    #[test]
    fn decodes_position_look_packet() {
        let mut data = Vec::new();
        codec::write_f64(&mut data, 1.25);
        codec::write_f64(&mut data, 80.0);
        codec::write_f64(&mut data, -2.5);
        codec::write_f32(&mut data, 90.0);
        codec::write_f32(&mut data, 30.0);
        codec::write_bool(&mut data, true);
        codec::write_bool(&mut data, false);

        assert_eq!(
            Movement::decode(ids::play::SERVERBOUND_POSITION_LOOK, data).unwrap(),
            Some(Movement::PositionLook {
                x: 1.25,
                y: 80.0,
                z: -2.5,
                yaw: 90.0,
                pitch: 30.0,
                on_ground: true,
                horizontal_collision: false,
            })
        );
    }

    #[test]
    fn rejects_movement_trailing_bytes() {
        let data = vec![0, 0, 0];
        assert!(Movement::decode(ids::play::SERVERBOUND_FLYING, data).is_err());
    }
}
