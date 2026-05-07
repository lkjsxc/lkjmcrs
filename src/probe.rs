use crate::protocol::codec;
mod block_mutation;
mod chunk;
mod chunk_stream;
mod inventory_packets;
mod inventory_sync;
mod item_entities;
mod item_pickup;
mod live_play;
mod multiplayer_mutation;
#[path = "probe/online_auth.rs"]
mod online_auth_probe;
mod persistence;
mod play_bootstrap;
mod play_client;
mod position;
mod profile_reconnect;
mod registry_assert;
mod scale_chunk_stream;
mod scale_chunk_stream_packets;
mod smoke;
mod smp_commands;
mod survival_expect;
mod survival_item;
mod survival_vitals;
mod validation;
mod vitals_packets;

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

pub async fn login_play(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    smoke::run(host).await?;
    println!("login-play probe ok");
    Ok(())
}

pub async fn multiplayer_mutation(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    multiplayer_mutation::run(host).await?;
    println!("multiplayer-mutation probe ok");
    Ok(())
}

pub async fn persist_place(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    persistence::place(host).await?;
    println!("persist-place probe ok");
    Ok(())
}

pub async fn persist_check(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    persistence::check(host).await?;
    println!("persist-check probe ok");
    Ok(())
}

pub async fn profile_reconnect(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    profile_reconnect::run(host).await?;
    println!("profile-reconnect probe ok");
    Ok(())
}

pub async fn chunk_stream(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    chunk_stream::run(host).await?;
    println!("chunk-stream probe ok");
    Ok(())
}

pub async fn scale_chunk_stream(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    scale_chunk_stream::run(host).await?;
    println!("scale-chunk-stream probe ok");
    Ok(())
}

pub async fn survival_item(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    survival_item::run(host).await?;
    println!("survival-item probe ok");
    Ok(())
}

pub async fn survival_vitals(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    survival_vitals::run(host).await?;
    println!("survival-vitals probe ok");
    Ok(())
}

pub async fn inventory_sync(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    inventory_sync::run(host).await?;
    println!("inventory-sync probe ok");
    Ok(())
}

pub async fn item_pickup(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    item_pickup::run(host).await?;
    println!("item-pickup probe ok");
    Ok(())
}

pub async fn smp_commands(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    smp_commands::run(host).await?;
    println!("smp-commands probe ok");
    Ok(())
}

pub async fn online_auth(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    online_auth_probe::run(host).await?;
    println!("online-auth probe ok");
    Ok(())
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
