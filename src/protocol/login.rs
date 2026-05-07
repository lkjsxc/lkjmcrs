use crate::protocol::codec;
use std::io::Cursor;
use uuid::Uuid;

pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

pub fn encode_encryption_request(public_key: &[u8], verify_token: &[u8]) -> Vec<u8> {
    let mut payload = Vec::new();
    codec::write_string(&mut payload, "");
    codec::write_var_i32(&mut payload, public_key.len() as i32);
    payload.extend_from_slice(public_key);
    codec::write_var_i32(&mut payload, verify_token.len() as i32);
    payload.extend_from_slice(verify_token);
    codec::write_bool(&mut payload, true);
    payload
}

pub fn decode_encryption_response(data: Vec<u8>) -> Result<EncryptionResponse, codec::CodecError> {
    let mut cursor = Cursor::new(data);
    let shared_secret = read_byte_array(&mut cursor)?;
    let verify_token = read_byte_array(&mut cursor)?;
    Ok(EncryptionResponse {
        shared_secret,
        verify_token,
    })
}

pub fn encode_success(uuid: Uuid, name: &str) -> Vec<u8> {
    let mut payload = Vec::new();
    codec::write_uuid(&mut payload, uuid);
    codec::write_string(&mut payload, name);
    codec::write_var_i32(&mut payload, 0);
    payload
}

fn read_byte_array(cursor: &mut Cursor<Vec<u8>>) -> Result<Vec<u8>, codec::CodecError> {
    let length = codec::read_var_i32(cursor)?;
    if length < 0 {
        return Err(codec::CodecError::NegativeLength);
    }
    let mut bytes = vec![0; length as usize];
    std::io::Read::read_exact(cursor, &mut bytes).map_err(|_| codec::CodecError::Eof)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{decode_encryption_response, encode_encryption_request, encode_success};
    use crate::protocol::codec;
    use std::io::Cursor;
    use uuid::Uuid;

    #[test]
    fn login_finished_has_no_trailing_bool() {
        let uuid = Uuid::from_u128(0x00112233445566778899aabbccddeeff);
        let payload = encode_success(uuid, "Probe");
        let mut cursor = Cursor::new(payload);
        assert_eq!(codec::read_uuid(&mut cursor).unwrap(), uuid);
        assert_eq!(codec::read_string(&mut cursor).unwrap(), "Probe");
        assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 0);
        assert_eq!(cursor.position(), cursor.get_ref().len() as u64);
    }

    #[test]
    fn encryption_request_response_payloads_round_trip() {
        let request = encode_encryption_request(&[1, 2, 3], &[4, 5]);
        let mut cursor = Cursor::new(request);
        assert_eq!(codec::read_string(&mut cursor).unwrap(), "");
        assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 3);
        cursor.set_position(cursor.position() + 3);
        assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 2);
        cursor.set_position(cursor.position() + 2);
        assert!(codec::read_bool(&mut cursor).unwrap());

        let mut response = Vec::new();
        codec::write_var_i32(&mut response, 2);
        response.extend_from_slice(&[8, 9]);
        codec::write_var_i32(&mut response, 1);
        response.push(7);
        let decoded = decode_encryption_response(response).unwrap();
        assert_eq!(decoded.shared_secret, [8, 9]);
        assert_eq!(decoded.verify_token, [7]);
    }
}
