use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::inventory_packets;
use crate::probe::inventory_packets::PlayerInventorySlot;
use crate::probe::item_entities;
use crate::probe::play_client::PlayClient;
use crate::protocol::{codec, ids};
use crate::world::BlockPos;
use tokio::net::TcpStream;

const NAME: &str = "InventorySync";

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlayClient::connect_with_block(host, NAME, Some(0)).await?;
    assert_bootstrap_inventory(&client)?;
    assert_invalid_held_slot_resends_authority(&mut client.stream).await?;
    place_stone_consumes_selected_stack(&mut client.stream).await?;
    break_stone_adds_matching_delta(&mut client.stream).await
}

fn assert_bootstrap_inventory(client: &PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    if client.selected_hotbar_slot != 0 {
        return Err(Box::new(ProbeError::Phase("bootstrap held slot")));
    }
    let Some(slot) = client.inventory_slots.first() else {
        return Err(Box::new(ProbeError::Phase("bootstrap inventory")));
    };
    expect_slot(slot, 0, 1, Some(1), "bootstrap starter stone")?;
    for slot in client.inventory_slots.iter().skip(1) {
        expect_slot(slot, slot.slot_id, 0, None, "bootstrap empty slot")?;
    }
    Ok(())
}

async fn assert_invalid_held_slot_resends_authority(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_i16(&mut payload, 9);
    codec::write_packet(stream, ids::play::SERVERBOUND_HELD_ITEM_SLOT, &payload).await?;
    let held = block_mutation::read_next_non_time(stream, "invalid held slot").await?;
    if held.id != ids::play::HELD_ITEM_SLOT {
        return Err(Box::new(ProbeError::Phase("invalid held slot id")));
    }
    if inventory_packets::decode_held_item_slot(held.data)? != 0 {
        return Err(Box::new(ProbeError::Phase("invalid held slot value")));
    }
    let chat = block_mutation::read_next_non_time(stream, "invalid held slot chat").await?;
    if chat.id != ids::play::SYSTEM_CHAT {
        return Err(Box::new(ProbeError::Phase("invalid held slot chat")));
    }
    Ok(())
}

async fn place_stone_consumes_selected_stack(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::send_use_item_on_at(stream, 60, BlockPos::new(0, 79, 0)).await?;
    expect_ack(stream, 60).await?;
    let slot = expect_inventory_delta(stream).await?;
    expect_slot(&slot, 0, 0, None, "placement inventory delta")?;
    expect_update(stream, 1).await
}

async fn break_stone_adds_matching_delta(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    block_mutation::send_start_destroy_at(stream, 61, BlockPos::new(0, 80, 0)).await?;
    expect_ack(stream, 61).await?;
    expect_update(stream, 0).await?;
    let slot = item_entities::collect_drop(stream, 1, "inventory pickup").await?;
    expect_slot(&slot, 0, 1, Some(1), "breaking inventory delta")?;
    Ok(())
}

async fn expect_ack(
    stream: &mut TcpStream,
    sequence: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let ack = block_mutation::read_next_non_time(stream, "inventory ack").await?;
    if ack.id != ids::play::BLOCK_CHANGED_ACK {
        return Err(Box::new(ProbeError::Phase("inventory ack id")));
    }
    block_mutation::validate_ack(ack.data, sequence)
}

async fn expect_inventory_delta(
    stream: &mut TcpStream,
) -> Result<PlayerInventorySlot, Box<dyn std::error::Error>> {
    let packet = block_mutation::read_next_non_time(stream, "inventory delta").await?;
    if packet.id != ids::play::SET_PLAYER_INVENTORY {
        return Err(Box::new(ProbeError::Phase("inventory delta id")));
    }
    inventory_packets::decode_player_inventory_slot(packet.data)
}

async fn expect_update(
    stream: &mut TcpStream,
    block_state: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let update = block_mutation::read_next_non_time(stream, "inventory update").await?;
    if update.id != ids::play::BLOCK_UPDATE {
        return Err(Box::new(ProbeError::Phase("inventory update id")));
    }
    block_mutation::validate_update(update.data, block_state)
}

fn expect_slot(
    slot: &PlayerInventorySlot,
    slot_id: i32,
    item_count: i32,
    item_id: Option<i32>,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    if slot.slot_id != slot_id || slot.item_count != item_count || slot.item_id != item_id {
        return Err(Box::new(ProbeError::Phase(phase)));
    }
    Ok(())
}
