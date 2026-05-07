use crate::protocol::codec;
use crate::protocol::ids;
use crate::session::bootstrap::send_chunk_batch;
use crate::session::play_packets::decode_keepalive;
use crate::world::{ChunkPos, ChunkSnapshot};
use tokio::io::AsyncWriteExt;

#[test]
fn keepalive_decode_rejects_trailing_bytes() {
    let mut data = Vec::new();
    codec::write_i64(&mut data, 7);
    data.push(0);
    assert!(decode_keepalive(data).is_err());
}

#[tokio::test]
async fn chunk_batch_sends_embedded_light_without_update_light() {
    let (mut reader, mut writer) = tokio::io::duplex(128 * 1024);
    let chunks = vec![ChunkSnapshot::flat(ChunkPos::new(0, 0))];
    send_chunk_batch(&mut writer, &chunks).await.unwrap();
    writer.shutdown().await.unwrap();

    assert_eq!(
        codec::read_packet(&mut reader).await.unwrap().id,
        ids::play::CHUNK_BATCH_START
    );
    assert_eq!(
        codec::read_packet(&mut reader).await.unwrap().id,
        ids::play::LEVEL_CHUNK_WITH_LIGHT
    );
    assert_eq!(
        codec::read_packet(&mut reader).await.unwrap().id,
        ids::play::CHUNK_BATCH_FINISHED
    );
}
