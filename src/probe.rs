use crate::protocol::codec;
mod block_mutation;
mod chunk;
mod chunk_stream;
mod commands;
mod inventory_packets;
mod inventory_sync;
mod item_entities;
mod item_pickup;
mod live_play;
mod movement_authority;
mod multiplayer_mutation;
#[path = "probe/online_auth.rs"]
mod online_auth_probe;
mod persistence;
mod play_bootstrap;
mod play_client;
mod position;
mod profile_reconnect;
mod registry_assert;
mod render_distance;
mod render_moving_pending;
mod scale_chunk_stream;
mod scale_chunk_stream_packets;
mod scale_load_metrics;
mod scale_moving_pending;
mod smoke;
mod smp_commands;
mod storage_section_persistence;
mod survival_expect;
mod survival_item;
mod survival_vitals;
mod terrain_chunk;
mod terrain_generation;
mod terrain_rivers;
mod validation;
mod vitals_packets;

pub use commands::{
    chunk_stream, inventory_sync, item_pickup, login_play, movement_authority,
    multiplayer_mutation, online_auth, persist_check, persist_place, profile_reconnect,
    render_distance, render_moving_pending, scale_chunk_stream, scale_load_metrics,
    scale_moving_pending, smp_commands, storage_section_persistence, survival_item,
    survival_vitals, terrain_generation, terrain_rivers,
};

use crate::protocol::PROTOCOL_VERSION;
use crate::protocol::ids;
use crate::protocol::types::{Handshake, NextState};
use std::future::Future;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio::net::TcpStream;
use tokio::time::{Duration, Instant, sleep};

#[derive(Debug, Error)]
enum ProbeError {
    #[error("probe phase failed: {0}")]
    Phase(&'static str),
}

const RETRY_TOTAL: Duration = Duration::from_secs(60);
const RETRY_INITIAL: Duration = Duration::from_millis(250);
const RETRY_MAX: Duration = Duration::from_secs(1);

pub(super) async fn retry_connect<T, F, Fut>(
    mut attempt: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    let start = Instant::now();
    let mut delay = RETRY_INITIAL;
    loop {
        match attempt().await {
            Ok(value) => return Ok(value),
            Err(err) if start.elapsed() >= RETRY_TOTAL => return Err(err),
            Err(_) => {
                sleep(delay).await;
                delay = (delay * 2).min(RETRY_MAX);
            }
        }
    }
}

pub(super) async fn send_handshake(
    stream: &mut TcpStream,
    host: &str,
    next_state: NextState,
) -> Result<(), codec::CodecError> {
    let handshake = Handshake {
        protocol: PROTOCOL_VERSION,
        address: host.to_string(),
        port: 25565,
        next_state,
    };
    codec::write_packet(stream, ids::HANDSHAKE, &handshake.encode()).await
}

pub(super) async fn expect<S>(
    stream: &mut S,
    id: i32,
    phase: &'static str,
) -> Result<codec::Packet, Box<dyn std::error::Error>>
where
    S: AsyncRead + Unpin,
{
    let packet = codec::read_packet(stream).await?;
    if packet.id == id {
        Ok(packet)
    } else {
        Err(Box::new(ProbeError::Phase(phase)))
    }
}
