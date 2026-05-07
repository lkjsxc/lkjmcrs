use crate::probe::ProbeError;
use crate::protocol::{codec, ids};
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;

pub(super) async fn send_position_look<S>(stream: &mut S) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsyncWrite + Unpin,
{
    send_position_look_at(stream, 0.5, 80.0, 0.5, 0.0, 0.0).await
}

pub(super) async fn send_position_look_at<S>(
    stream: &mut S,
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsyncWrite + Unpin,
{
    let mut payload = Vec::new();
    codec::write_f64(&mut payload, x);
    codec::write_f64(&mut payload, y);
    codec::write_f64(&mut payload, z);
    codec::write_f32(&mut payload, yaw);
    codec::write_f32(&mut payload, pitch);
    codec::write_u8(&mut payload, 0);
    codec::write_packet(stream, ids::play::SERVERBOUND_POSITION_LOOK, &payload).await?;
    Ok(())
}

pub(super) async fn expect_keepalive_after_time(
    stream: &mut TcpStream,
) -> Result<i64, Box<dyn std::error::Error>> {
    let mut saw_time = false;
    loop {
        let packet = codec::read_packet(stream).await?;
        match packet.id {
            ids::play::KEEPALIVE => {
                if !saw_time {
                    return Err(Box::new(ProbeError::Phase("post-bootstrap time")));
                }
                return Ok(codec::read_i64(&mut std::io::Cursor::new(packet.data))?);
            }
            ids::play::SET_TIME => {
                saw_time = true;
            }
            id if ignorable_live_packet(id) => {}
            _ => return Err(Box::new(ProbeError::Phase("periodic play packet"))),
        }
    }
}

fn ignorable_live_packet(id: i32) -> bool {
    matches!(
        id,
        ids::play::SPAWN_ENTITY
            | ids::play::ENTITY_DESTROY
            | ids::play::ENTITY_METADATA
            | ids::play::UPDATE_HEALTH
            | ids::play::HELD_ITEM_SLOT
            | ids::play::SET_PLAYER_INVENTORY
            | ids::play::SYSTEM_CHAT
            | ids::play::COLLECT
    )
}
