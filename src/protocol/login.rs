use crate::protocol::codec;
use uuid::Uuid;

pub fn encode_success(uuid: Uuid, name: &str) -> Vec<u8> {
    let mut payload = Vec::new();
    codec::write_uuid(&mut payload, uuid);
    codec::write_string(&mut payload, name);
    codec::write_var_i32(&mut payload, 0);
    payload
}

#[cfg(test)]
mod tests {
    use super::encode_success;
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
}
