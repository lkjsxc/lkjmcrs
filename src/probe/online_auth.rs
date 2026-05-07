use crate::probe::{ProbeError, expect, send_handshake, validation};
use crate::protocol::codec;
use crate::protocol::encryption::EncryptedStream;
use crate::protocol::ids;
use crate::protocol::types::NextState;
use rand::RngCore;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::io::Cursor;
use tokio::net::TcpStream;

pub async fn run(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(host).await?;
    send_handshake(&mut stream, host, NextState::Login).await?;
    let login = crate::protocol::types::LoginStart::encode("OnlineProbe", uuid::Uuid::nil());
    codec::write_packet(&mut stream, ids::login::START, &login).await?;
    let request = expect(
        &mut stream,
        ids::login::ENCRYPTION_REQUEST,
        "encryption request",
    )
    .await?;
    let (public_key, token) = decode_encryption_request(request.data)?;
    let mut secret = [0_u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut secret);
    let key = RsaPublicKey::from_public_key_der(&public_key)?;
    let encrypted_secret = key.encrypt(&mut rand::rngs::OsRng, Pkcs1v15Encrypt, &secret)?;
    let encrypted_token = key.encrypt(&mut rand::rngs::OsRng, Pkcs1v15Encrypt, &token)?;
    send_encryption_response(&mut stream, &encrypted_secret, &encrypted_token).await?;
    let mut stream = EncryptedStream::new(stream, &secret);
    let success = codec::read_packet(&mut stream).await?;
    if success.id != ids::login::SUCCESS {
        return Err(Box::new(ProbeError::Phase("online login success")));
    }
    let uuid = validation::decode_login_success_uuid(success.data)?;
    if uuid != uuid::Uuid::parse_str("00112233-4455-6677-8899-aabbccddeeff")? {
        return Err(Box::new(ProbeError::Phase("online login uuid")));
    }
    Ok(())
}

async fn send_encryption_response(
    stream: &mut TcpStream,
    secret: &[u8],
    token: &[u8],
) -> Result<(), codec::CodecError> {
    let mut response = Vec::new();
    codec::write_var_i32(&mut response, secret.len() as i32);
    response.extend_from_slice(secret);
    codec::write_var_i32(&mut response, token.len() as i32);
    response.extend_from_slice(token);
    codec::write_packet(stream, ids::login::ENCRYPTION_RESPONSE, &response).await
}

fn decode_encryption_request(data: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), codec::CodecError> {
    let mut cursor = Cursor::new(data);
    let server_id = codec::read_string(&mut cursor)?;
    if !server_id.is_empty() {
        return Err(codec::CodecError::Eof);
    }
    let public_key = read_bytes(&mut cursor)?;
    let verify_token = read_bytes(&mut cursor)?;
    if !codec::read_bool(&mut cursor)? {
        return Err(codec::CodecError::Eof);
    }
    Ok((public_key, verify_token))
}

fn read_bytes(cursor: &mut Cursor<Vec<u8>>) -> Result<Vec<u8>, codec::CodecError> {
    let length = codec::read_var_i32(cursor)?;
    if length < 0 {
        return Err(codec::CodecError::NegativeLength);
    }
    let mut bytes = vec![0; length as usize];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| codec::CodecError::Eof)?;
    Ok(bytes)
}
