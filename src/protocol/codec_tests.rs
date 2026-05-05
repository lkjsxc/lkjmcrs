use crate::protocol::codec::{
    CodecError, read_bool, read_error_to_codec, read_f32, read_f64, read_packet, read_position,
    read_var_i32, write_position, write_var_i32,
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
    assert_eq!(read_position(&mut Cursor::new(bytes)).unwrap(), (0, 80, 0));
}

#[test]
fn position_decoding_sign_extends_negative_axes() {
    let mut bytes = Vec::new();
    write_position(&mut bytes, -1, -64, -2);
    assert_eq!(
        read_position(&mut Cursor::new(bytes)).unwrap(),
        (-1, -64, -2)
    );
}

#[test]
fn scalar_readers_decode_big_endian_payloads() {
    let mut bytes = Vec::new();
    bytes.push(1);
    bytes.extend_from_slice(&90.0f32.to_be_bytes());
    bytes.extend_from_slice(&80.5f64.to_be_bytes());
    let mut cursor = Cursor::new(bytes);
    assert!(read_bool(&mut cursor).unwrap());
    assert_eq!(read_f32(&mut cursor).unwrap(), 90.0);
    assert_eq!(read_f64(&mut cursor).unwrap(), 80.5);
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
