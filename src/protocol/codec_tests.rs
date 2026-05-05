use crate::protocol::codec::{
    CodecError, read_error_to_codec, read_packet, read_var_i32, write_position, write_var_i32,
};
use std::io::Cursor;
use tokio::io::{AsyncWriteExt, duplex};

#[test]
fn varint_round_trip_boundaries() {
    for value in [0, 1, 127, 128, 255, 2_147_483_647] {
        let mut bytes = Vec::new();
        write_var_i32(&mut bytes, value);
        assert_eq!(read_var_i32(&mut Cursor::new(bytes)).unwrap(), value);
    }
}

#[test]
fn position_encoding_matches_protocol_layout() {
    let mut bytes = Vec::new();
    write_position(&mut bytes, 0, 80, 0);
    assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0, 80]);
}

#[tokio::test]
async fn empty_async_read_is_connection_closed() {
    let (mut client, server) = duplex(8);
    drop(server);
    let error = read_packet(&mut client).await.unwrap_err();
    assert!(matches!(error, CodecError::ConnectionClosed));
}

#[tokio::test]
async fn partial_frame_read_is_connection_closed() {
    let (mut client, mut server) = duplex(8);
    server.write_all(&[2, 0]).await.unwrap();
    drop(server);
    let error = read_packet(&mut client).await.unwrap_err();
    assert!(matches!(error, CodecError::ConnectionClosed));
}

#[test]
fn connection_reset_is_connection_closed() {
    let error = std::io::Error::from(std::io::ErrorKind::ConnectionReset);
    assert!(matches!(
        read_error_to_codec(error),
        CodecError::ConnectionClosed
    ));
}
