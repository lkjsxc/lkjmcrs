use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::play_client::PlayClient;
use crate::protocol::{codec, ids};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut admin = PlayClient::connect(host, "Admin").await?;
    let mut guest = PlayClient::connect(host, "Guest").await?;
    if !admin.declared_commands || !guest.declared_commands {
        return Err(Box::new(ProbeError::Phase("declare commands")));
    }
    send_chat(&mut guest.stream, "hello").await?;
    expect_system_chat(&mut admin.stream, "Guest").await?;
    send_command(&mut guest.stream, "say denied").await?;
    expect_system_chat(&mut guest.stream, "Permission denied").await?;
    send_command(&mut admin.stream, "gamemode survival Guest").await?;
    expect_game_mode_change(&mut guest.stream).await?;
    guest.stream.shutdown().await?;
    drop(guest);
    sleep(Duration::from_millis(500)).await;

    let mut guest = PlayClient::connect(host, "Guest").await?;
    if guest.login.game_mode != 0 {
        return Err(Box::new(ProbeError::Phase("persisted gamemode")));
    }
    send_command(&mut admin.stream, "kick Guest done").await?;
    expect_kick(&mut guest.stream, "done").await
}

async fn send_chat(
    stream: &mut TcpStream,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_string(&mut payload, message);
    codec::write_i64(&mut payload, 0);
    codec::write_i64(&mut payload, 0);
    codec::write_bool(&mut payload, false);
    codec::write_var_i32(&mut payload, 0);
    payload.extend_from_slice(&[0; 3]);
    codec::write_u8(&mut payload, 0);
    codec::write_packet(stream, ids::play::SERVERBOUND_CHAT_MESSAGE, &payload).await?;
    Ok(())
}

async fn send_command(
    stream: &mut TcpStream,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_string(&mut payload, command);
    codec::write_packet(stream, ids::play::SERVERBOUND_CHAT_COMMAND, &payload).await?;
    Ok(())
}

async fn expect_system_chat(
    stream: &mut TcpStream,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..20 {
        let packet = block_mutation::read_next_non_time(stream, "system chat").await?;
        if packet.id != ids::play::SYSTEM_CHAT {
            continue;
        }
        if String::from_utf8_lossy(&packet.data).contains(text) {
            return Ok(());
        }
    }
    Err(Box::new(ProbeError::Phase("system chat text")))
}

async fn expect_game_mode_change(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..20 {
        let packet = block_mutation::read_next_non_time(stream, "gamemode change").await?;
        if packet.id == ids::play::GAME_STATE_CHANGE {
            return Ok(());
        }
    }
    Err(Box::new(ProbeError::Phase("gamemode change")))
}

async fn expect_kick(
    stream: &mut TcpStream,
    reason: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..20 {
        let packet = block_mutation::read_next_non_time(stream, "kick").await?;
        if packet.id != ids::play::KICK_DISCONNECT {
            continue;
        }
        if String::from_utf8_lossy(&packet.data).contains(reason) {
            return Ok(());
        }
    }
    Err(Box::new(ProbeError::Phase("kick reason")))
}
