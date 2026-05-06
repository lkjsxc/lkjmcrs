use crate::probe::ProbeError;
use crate::probe::block_mutation;
use crate::probe::live_play;
use crate::probe::play_client::PlayClient;
use crate::protocol::{codec, ids};
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};

const NAME: &str = "ProfileA";
const X: f64 = 2.25;
const Y: f64 = 81.5;
const Z: f64 = 3.75;
const YAW: f32 = 45.0;
const PITCH: f32 = 10.0;

pub(super) async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut first = PlayClient::connect(host, NAME).await?;
    live_play::send_position_look_at(&mut first.stream, X, Y, Z, YAW, PITCH).await?;
    confirm_ordered_packet_processing(&mut first.stream).await?;
    first.stream.shutdown().await?;
    drop(first);
    sleep(Duration::from_millis(500)).await;

    let second = PlayClient::connect(host, NAME).await?;
    if !matches_saved_position(&second) {
        return Err(Box::new(ProbeError::Phase("profile reconnect position")));
    }
    Ok(())
}

async fn confirm_ordered_packet_processing(
    stream: &mut tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Vec::new();
    codec::write_string(&mut payload, "homes");
    codec::write_packet(stream, ids::play::SERVERBOUND_CHAT_COMMAND, &payload).await?;
    let packet = block_mutation::read_next_non_time(stream, "profile barrier").await?;
    if packet.id == ids::play::SYSTEM_CHAT
        && String::from_utf8_lossy(&packet.data).contains("Homes")
    {
        Ok(())
    } else {
        Err(Box::new(ProbeError::Phase("profile barrier")))
    }
}

fn matches_saved_position(client: &PlayClient) -> bool {
    approx(client.initial_position.x, X)
        && approx(client.initial_position.y, Y)
        && approx(client.initial_position.z, Z)
        && approx_f32(client.initial_position.yaw, YAW)
        && approx_f32(client.initial_position.pitch, PITCH)
}

fn approx(left: f64, right: f64) -> bool {
    (left - right).abs() < 0.0001
}

fn approx_f32(left: f32, right: f32) -> bool {
    (left - right).abs() < 0.0001
}
