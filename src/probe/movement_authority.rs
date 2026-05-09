use crate::probe::ProbeError;
use crate::probe::play_client::PlayClient;
use crate::probe::survival_expect;
use crate::probe::validation::{PositionPacket, decode_position_packet};
use crate::protocol::{codec, ids};
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};

const NAME: &str = "MoveAuthorityA";
const YAW: f32 = 35.0;
const PITCH: f32 = 8.0;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlayClient::connect(host, NAME).await?;
    let accepted = accepted_position(client.initial_position);
    send_position_look(&mut client.stream, accepted, YAW, PITCH).await?;
    expect_position_correction(&mut client, f64::NAN).await?;
    expect_position_correction(&mut client, 900.0).await?;
    confirm_ordered_packet_processing(&mut client.stream).await?;
    client.stream.shutdown().await?;
    drop(client);
    sleep(Duration::from_millis(2_000)).await;

    let reconnected = PlayClient::connect(host, NAME).await?;
    if !matches_position(reconnected.initial_position, accepted, YAW, PITCH) {
        return Err(Box::new(ProbeError::Phase(
            "movement authority persisted position",
        )));
    }
    Ok(())
}

async fn expect_position_correction(
    client: &mut PlayClient,
    y: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    send_position_look(&mut client.stream, (0.5, y, 0.5), 0.0, 0.0).await?;
    for _ in 0..20 {
        let packet = survival_expect::read_next_survival_packet(&mut client.stream).await?;
        if packet.id == ids::play::PLAYER_POSITION {
            let correction = decode_position_packet(packet.data)?;
            if matches_position(
                correction,
                accepted_position(client.initial_position),
                YAW,
                PITCH,
            ) {
                return Ok(());
            }
            return Err(Box::new(ProbeError::Phase("movement correction position")));
        }
    }
    Err(Box::new(ProbeError::Phase("movement correction packet")))
}

async fn send_position_look<S>(
    stream: &mut S,
    pos: (f64, f64, f64),
    yaw: f32,
    pitch: f32,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: tokio::io::AsyncWrite + Unpin,
{
    let mut payload = Vec::new();
    codec::write_f64(&mut payload, pos.0);
    codec::write_f64(&mut payload, pos.1);
    codec::write_f64(&mut payload, pos.2);
    codec::write_f32(&mut payload, yaw);
    codec::write_f32(&mut payload, pitch);
    codec::write_u8(&mut payload, 0x01);
    codec::write_packet(stream, ids::play::SERVERBOUND_POSITION_LOOK, &payload).await?;
    Ok(())
}

async fn confirm_ordered_packet_processing(
    stream: &mut tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_string(&mut payload, "homes");
    codec::write_packet(stream, ids::play::SERVERBOUND_CHAT_COMMAND, &payload).await?;
    for _ in 0..20 {
        let packet = survival_expect::read_next_material_packet(stream, "movement barrier").await?;
        if packet.id == ids::play::SYSTEM_CHAT
            && String::from_utf8_lossy(&packet.data).contains("Homes")
        {
            return Ok(());
        }
    }
    Err(Box::new(ProbeError::Phase("movement barrier")))
}

fn accepted_position(initial: PositionPacket) -> (f64, f64, f64) {
    (initial.x + 1.0, initial.y, initial.z + 1.0)
}

fn matches_position(
    actual: PositionPacket,
    expected: (f64, f64, f64),
    yaw: f32,
    pitch: f32,
) -> bool {
    approx(actual.x, expected.0)
        && approx(actual.y, expected.1)
        && approx(actual.z, expected.2)
        && approx_f32(actual.yaw, yaw)
        && approx_f32(actual.pitch, pitch)
}

fn approx(left: f64, right: f64) -> bool {
    (left - right).abs() < 0.0001
}

fn approx_f32(left: f32, right: f32) -> bool {
    (left - right).abs() < 0.0001
}
