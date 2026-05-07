use crate::probe::ProbeError;
use crate::protocol::{codec, ids};
use std::io::Cursor;
use tokio::io::AsyncRead;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PlayerInventorySlot {
    pub slot_id: i32,
    pub item_count: i32,
    pub item_id: Option<i32>,
}

pub(super) async fn expect_held_item_slot<S>(
    stream: &mut S,
) -> Result<i32, Box<dyn std::error::Error>>
where
    S: AsyncRead + Unpin,
{
    let packet = super::expect(stream, ids::play::HELD_ITEM_SLOT, "held item slot").await?;
    decode_held_item_slot(packet.data)
}

pub(super) async fn expect_player_inventory<S>(
    stream: &mut S,
) -> Result<Vec<PlayerInventorySlot>, Box<dyn std::error::Error>>
where
    S: AsyncRead + Unpin,
{
    let mut slots = Vec::new();
    for expected_slot in 0..36 {
        let packet = super::expect(
            stream,
            ids::play::SET_PLAYER_INVENTORY,
            "set player inventory",
        )
        .await?;
        let slot = decode_player_inventory_slot(packet.data)?;
        if slot.slot_id != expected_slot {
            return Err(Box::new(ProbeError::Phase("player inventory slot order")));
        }
        slots.push(slot);
    }
    Ok(slots)
}

pub(super) fn decode_held_item_slot(data: Vec<u8>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let slot = codec::read_var_i32(&mut cursor)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("held item slot trailing")));
    }
    Ok(slot)
}

pub(super) fn decode_player_inventory_slot(
    data: Vec<u8>,
) -> Result<PlayerInventorySlot, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);
    let slot_id = codec::read_var_i32(&mut cursor)?;
    let item_count = codec::read_var_i32(&mut cursor)?;
    let item_id = if item_count == 0 {
        None
    } else {
        let item_id = codec::read_var_i32(&mut cursor)?;
        if codec::read_var_i32(&mut cursor)? != 0 || codec::read_var_i32(&mut cursor)? != 0 {
            return Err(Box::new(ProbeError::Phase("slot components")));
        }
        Some(item_id)
    };
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("player inventory trailing")));
    }
    Ok(PlayerInventorySlot {
        slot_id,
        item_count,
        item_id,
    })
}
