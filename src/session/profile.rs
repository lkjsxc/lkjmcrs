use md5::{Digest, Md5};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("invalid player name")]
    InvalidName,
}

pub fn validate_name(name: &str) -> Result<(), ProfileError> {
    let valid_len = (3..=16).contains(&name.len());
    let valid_chars = name
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
    if valid_len && valid_chars {
        Ok(())
    } else {
        Err(ProfileError::InvalidName)
    }
}

pub fn offline_uuid(name: &str) -> Uuid {
    let mut digest = Md5::new();
    digest.update(format!("OfflinePlayer:{name}").as_bytes());
    let mut bytes: [u8; 16] = digest.finalize().into();
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::{offline_uuid, validate_name};

    #[test]
    fn rejects_invalid_names() {
        assert!(validate_name("abc").is_ok());
        assert!(validate_name("ab").is_err());
        assert!(validate_name("bad-name").is_err());
    }

    #[test]
    fn offline_uuid_is_deterministic() {
        assert_eq!(offline_uuid("Probe"), offline_uuid("Probe"));
        assert_ne!(offline_uuid("Probe"), offline_uuid("Other"));
    }
}
