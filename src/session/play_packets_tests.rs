use crate::protocol::codec;
use crate::session::play_packets::decode_keepalive;

#[test]
fn keepalive_decode_rejects_trailing_bytes() {
    let mut data = Vec::new();
    codec::write_i64(&mut data, 7);
    data.push(0);
    assert!(decode_keepalive(data).is_err());
}
