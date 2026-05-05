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
        let x = codec::read_f64(cursor)?;
        let y = codec::read_f64(cursor)?;
        let z = codec::read_f64(cursor)?;
        let (on_ground, horizontal_collision) = read_flags(cursor)?;
        Ok(Self::Position {
            x,
            y,
            z,
            on_ground,
            horizontal_collision,
        })
    }

    fn decode_position_look(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        let x = codec::read_f64(cursor)?;
        let y = codec::read_f64(cursor)?;
        let z = codec::read_f64(cursor)?;
        let yaw = codec::read_f32(cursor)?;
        let pitch = codec::read_f32(cursor)?;
        let (on_ground, horizontal_collision) = read_flags(cursor)?;
        Ok(Self::PositionLook {
            x,
            y,
            z,
            yaw,
            pitch,
            on_ground,
            horizontal_collision,
        })
    }

    fn decode_look(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        let yaw = codec::read_f32(cursor)?;
        let pitch = codec::read_f32(cursor)?;
        let (on_ground, horizontal_collision) = read_flags(cursor)?;
        Ok(Self::Look {
            yaw,
            pitch,
            on_ground,
            horizontal_collision,
        })
    }

    fn decode_flying(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        let (on_ground, horizontal_collision) = read_flags(cursor)?;
        Ok(Self::Flying {
            on_ground,
            horizontal_collision,
        })
    }
}

fn read_flags(cursor: &mut Cursor<Vec<u8>>) -> Result<(bool, bool), CodecError> {
    let flags = codec::read_u8(cursor)?;
    Ok((flags & 0x01 != 0, flags & 0x02 != 0))
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
        codec::write_u8(&mut data, 0x01);

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
        let data = vec![0, 0];
        assert!(Movement::decode(ids::play::SERVERBOUND_FLYING, data).is_err());
    }

    #[test]
    fn decodes_status_only_flags_byte() {
        assert_eq!(
            Movement::decode(ids::play::SERVERBOUND_FLYING, vec![0x03]).unwrap(),
            Some(Movement::Flying {
                on_ground: true,
                horizontal_collision: true,
            })
        );
    }
}
