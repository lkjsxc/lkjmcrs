use crate::protocol::codec::{self, CodecError};
use crate::protocol::ids;
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockFace {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockInteraction {
    PlayerAction {
        action: PlayerAction,
        pos: BlockPos,
        face: BlockFace,
        sequence: i32,
    },
    UseItemOn {
        hand: i32,
        pos: BlockPos,
        face: BlockFace,
        sequence: i32,
    },
    Swing {
        hand: i32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    StartDestroyBlock,
    AbortDestroyBlock,
    StopDestroyBlock,
    Other(i32),
}

impl BlockInteraction {
    pub fn decode(packet_id: i32, data: Vec<u8>) -> Result<Option<Self>, CodecError> {
        let mut cursor = Cursor::new(data);
        let interaction = match packet_id {
            ids::play::SERVERBOUND_PLAYER_ACTION => Some(decode_player_action(&mut cursor)?),
            ids::play::SERVERBOUND_USE_ITEM_ON => Some(decode_use_item_on(&mut cursor)?),
            ids::play::SERVERBOUND_SWING => Some(Self::Swing {
                hand: codec::read_var_i32(&mut cursor)?,
            }),
            _ => None,
        };
        if interaction.is_some() && cursor.position() != cursor.get_ref().len() as u64 {
            return Err(CodecError::Eof);
        }
        Ok(interaction)
    }
}

pub fn encode_block_changed_ack(sequence: i32) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, sequence);
    out
}

pub fn encode_block_update(pos: BlockPos, block_state_id: i32) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_position(&mut out, pos.x, pos.y, pos.z);
    codec::write_var_i32(&mut out, block_state_id);
    out
}

fn decode_player_action(cursor: &mut Cursor<Vec<u8>>) -> Result<BlockInteraction, CodecError> {
    let action = match codec::read_var_i32(cursor)? {
        0 => PlayerAction::StartDestroyBlock,
        1 => PlayerAction::AbortDestroyBlock,
        2 => PlayerAction::StopDestroyBlock,
        other => PlayerAction::Other(other),
    };
    let pos = read_block_pos(cursor)?;
    let face = face_from_id(i32::from(codec::read_u8(cursor)?))?;
    let sequence = codec::read_var_i32(cursor)?;
    Ok(BlockInteraction::PlayerAction {
        action,
        pos,
        face,
        sequence,
    })
}

fn decode_use_item_on(cursor: &mut Cursor<Vec<u8>>) -> Result<BlockInteraction, CodecError> {
    let hand = codec::read_var_i32(cursor)?;
    let pos = read_block_pos(cursor)?;
    let face = face_from_id(codec::read_var_i32(cursor)?)?;
    let _hit_x = codec::read_f32(cursor)?;
    let _hit_y = codec::read_f32(cursor)?;
    let _hit_z = codec::read_f32(cursor)?;
    let _inside_block = codec::read_bool(cursor)?;
    let _world_border_hit = codec::read_bool(cursor)?;
    let sequence = codec::read_var_i32(cursor)?;
    Ok(BlockInteraction::UseItemOn {
        hand,
        pos,
        face,
        sequence,
    })
}

fn read_block_pos(cursor: &mut Cursor<Vec<u8>>) -> Result<BlockPos, CodecError> {
    let (x, y, z) = codec::read_position(cursor)?;
    Ok(BlockPos { x, y, z })
}

fn face_from_id(id: i32) -> Result<BlockFace, CodecError> {
    match id {
        0 => Ok(BlockFace::Down),
        1 => Ok(BlockFace::Up),
        2 => Ok(BlockFace::North),
        3 => Ok(BlockFace::South),
        4 => Ok(BlockFace::West),
        5 => Ok(BlockFace::East),
        _ => Err(CodecError::Eof),
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockInteraction, PlayerAction};
    use crate::protocol::{block_interaction, codec, ids};
    use block_interaction::{BlockFace, BlockPos};

    #[test]
    fn decodes_player_action() {
        let mut data = Vec::new();
        codec::write_var_i32(&mut data, 0);
        codec::write_position(&mut data, 0, 80, 0);
        codec::write_u8(&mut data, 1);
        codec::write_var_i32(&mut data, 9);

        assert_eq!(
            BlockInteraction::decode(ids::play::SERVERBOUND_PLAYER_ACTION, data).unwrap(),
            Some(BlockInteraction::PlayerAction {
                action: PlayerAction::StartDestroyBlock,
                pos: BlockPos { x: 0, y: 80, z: 0 },
                face: BlockFace::Up,
                sequence: 9,
            })
        );
    }

    #[test]
    fn decodes_use_item_on_and_rejects_trailing_bytes() {
        let mut data = use_item_on_payload(7);
        assert!(matches!(
            BlockInteraction::decode(ids::play::SERVERBOUND_USE_ITEM_ON, data.clone()).unwrap(),
            Some(BlockInteraction::UseItemOn { sequence: 7, .. })
        ));
        data.push(0);
        assert!(BlockInteraction::decode(ids::play::SERVERBOUND_USE_ITEM_ON, data).is_err());
    }

    #[test]
    fn block_update_encodes_position_and_state() {
        let payload = block_interaction::encode_block_update(BlockPos { x: 0, y: 80, z: 0 }, 1);
        assert_eq!(payload, vec![0, 0, 0, 0, 0, 0, 0, 80, 1]);
    }

    fn use_item_on_payload(sequence: i32) -> Vec<u8> {
        let mut data = Vec::new();
        codec::write_var_i32(&mut data, 0);
        codec::write_position(&mut data, 0, 79, 0);
        codec::write_var_i32(&mut data, 1);
        codec::write_f32(&mut data, 0.5);
        codec::write_f32(&mut data, 1.0);
        codec::write_f32(&mut data, 0.5);
        codec::write_bool(&mut data, false);
        codec::write_bool(&mut data, false);
        codec::write_var_i32(&mut data, sequence);
        data
    }
}
