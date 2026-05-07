use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::play_client::PlayClient;
use crate::probe::smp_commands;
use crate::probe::validation::decode_position_packet;
use crate::probe::vitals_packets::{self, validate_health};
use crate::protocol::{codec, ids};

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut admin = PlayClient::connect(host, "Admin").await?;
    let mut target = PlayClient::connect(host, "VitalsTarget").await?;
    validate_health(target.health, 20.0, "bootstrap health")?;

    smp_commands::send_command(&mut admin.stream, "damage VitalsTarget 7.5").await?;
    smp_commands::expect_system_chat(&mut admin.stream, "Damage applied").await?;
    let damaged = vitals_packets::expect_update_health(&mut target.stream).await?;
    validate_health(damaged, 12.5, "damaged health")?;

    smp_commands::send_command(&mut admin.stream, "damage VitalsTarget 20").await?;
    smp_commands::expect_system_chat(&mut admin.stream, "Damage applied").await?;
    expect_death(&mut target).await?;
    send_respawn(&mut target).await?;
    expect_respawn(&mut target).await?;
    expect_regeneration(&mut admin, &mut target).await?;
    expect_starvation(&mut admin, &mut target).await
}

async fn expect_death(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    let zero = vitals_packets::expect_update_health(&mut client.stream).await?;
    validate_health(zero, 0.0, "lethal health")?;
    let packet = block_mutation::read_next_non_time(&mut client.stream, "death event").await?;
    if packet.id != ids::play::DEATH_COMBAT_EVENT {
        return Err(Box::new(ProbeError::Phase("death event id")));
    }
    Ok(())
}

async fn send_respawn(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_var_i32(&mut payload, 0);
    codec::write_packet(
        &mut client.stream,
        ids::play::SERVERBOUND_CLIENT_COMMAND,
        &payload,
    )
    .await?;
    Ok(())
}

async fn expect_respawn(client: &mut PlayClient) -> Result<(), Box<dyn std::error::Error>> {
    let respawn = block_mutation::read_next_non_time(&mut client.stream, "respawn").await?;
    if respawn.id != ids::play::RESPAWN {
        return Err(Box::new(ProbeError::Phase("respawn id")));
    }
    let position =
        block_mutation::read_next_non_time(&mut client.stream, "respawn position").await?;
    if position.id != ids::play::PLAYER_POSITION {
        return Err(Box::new(ProbeError::Phase("respawn position id")));
    }
    let position = decode_position_packet(position.data)?;
    if !approx(position.x, 0.5) || !approx(position.y, 80.0) || !approx(position.z, 0.5) {
        return Err(Box::new(ProbeError::Phase("respawn position")));
    }
    let restored = vitals_packets::expect_update_health(&mut client.stream).await?;
    validate_health(restored, 20.0, "respawn health")
}

fn approx(left: f64, right: f64) -> bool {
    (left - right).abs() < 0.001
}

async fn expect_regeneration(
    admin: &mut PlayClient,
    target: &mut PlayClient,
) -> Result<(), Box<dyn std::error::Error>> {
    smp_commands::send_command(&mut admin.stream, "vitals VitalsTarget 19 20 1").await?;
    smp_commands::expect_system_chat(&mut admin.stream, "Vitals updated").await?;
    let set = vitals_packets::expect_update_health(&mut target.stream).await?;
    vitals_packets::validate_state(set, 19.0, 20, 1.0, "set regen vitals")?;
    let regen = expect_later_health(target).await?;
    vitals_packets::validate_state(regen, 20.0, 20, 0.0, "regenerated health")
}

async fn expect_starvation(
    admin: &mut PlayClient,
    target: &mut PlayClient,
) -> Result<(), Box<dyn std::error::Error>> {
    smp_commands::send_command(&mut admin.stream, "vitals VitalsTarget 20 0 0").await?;
    smp_commands::expect_system_chat(&mut admin.stream, "Vitals updated").await?;
    let set = vitals_packets::expect_update_health(&mut target.stream).await?;
    vitals_packets::validate_state(set, 20.0, 0, 0.0, "set starvation vitals")?;
    let starved = expect_later_health(target).await?;
    vitals_packets::validate_state(starved, 19.0, 0, 0.0, "starvation health")
}

async fn expect_later_health(
    client: &mut PlayClient,
) -> Result<vitals_packets::HealthState, Box<dyn std::error::Error>> {
    loop {
        let packet = block_mutation::read_next_non_time(&mut client.stream, "later health").await?;
        if packet.id == ids::play::UPDATE_HEALTH {
            return vitals_packets::decode_update_health(packet.data);
        }
    }
}
