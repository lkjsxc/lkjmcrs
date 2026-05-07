use crate::probe::play_client::PlayClient;
use crate::probe::{ProbeError, block_mutation, item_entities, live_play, multiplayer_mutation};
use crate::probe::{position, retry_connect};
use crate::protocol::types::NextState;
use crate::protocol::{PROTOCOL_VERSION, codec, ids};
use serde::Deserialize;
use tokio::net::TcpStream;

pub async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    status_ping(host).await?;
    let mut actor = PlayClient::connect(host, "SmokeA").await?;
    block_mutation::acquire_dirt(
        &mut actor.stream,
        position::BlockPos::new(0, 79, 0),
        "smoke dirt",
    )
    .await?;
    let mut observer = PlayClient::connect_with_block(host, "SmokeB", Some(0)).await?;
    block_mutation::send_use_item_on(&mut actor.stream, 20).await?;
    block_mutation::expect_ack_and_update(&mut actor.stream, 20, 10).await?;
    multiplayer_mutation::expect_observer_update(&mut observer, 10).await?;
    block_mutation::send_start_destroy(&mut actor.stream, 21).await?;
    block_mutation::expect_ack_and_update(&mut actor.stream, 21, 0).await?;
    multiplayer_mutation::expect_observer_update(&mut observer, 0).await?;
    item_entities::collect_drop(&mut actor.stream, 28, "smoke dirt cleanup").await?;
    let next_keepalive = live_play::expect_keepalive_after_time(&mut actor.stream).await?;
    if next_keepalive != 2 {
        return Err(Box::new(ProbeError::Phase("periodic keepalive id")));
    }
    Ok(())
}

#[derive(Deserialize)]
struct StatusEnvelope {
    version: StatusVersion,
}

#[derive(Deserialize)]
struct StatusVersion {
    name: String,
    protocol: i32,
}

async fn status_ping(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = retry_connect(|| async move {
        Ok::<TcpStream, Box<dyn std::error::Error>>(TcpStream::connect(host).await?)
    })
    .await?;
    super::send_handshake(&mut stream, host, NextState::Status).await?;
    codec::write_packet(&mut stream, ids::status::REQUEST, &[]).await?;
    let response = super::expect(&mut stream, ids::status::RESPONSE, "status response").await?;
    let mut cursor = std::io::Cursor::new(response.data);
    let json = codec::read_string(&mut cursor)?;
    let status: StatusEnvelope = serde_json::from_str(&json)?;
    if status.version.protocol != PROTOCOL_VERSION || status.version.name != "1.21.11" {
        return Err(Box::new(ProbeError::Phase("status version")));
    }
    let payload = 123_i64.to_be_bytes();
    codec::write_packet(&mut stream, ids::status::PING, &payload).await?;
    let pong = super::expect(&mut stream, ids::status::PONG, "status pong").await?;
    if pong.data != payload {
        return Err(Box::new(ProbeError::Phase("status pong payload")));
    }
    Ok(())
}
