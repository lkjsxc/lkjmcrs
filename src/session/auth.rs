use serde::Deserialize;
use sha1::{Digest, Sha1};
use thiserror::Error;
use tokio::time::{Duration, timeout};
use uuid::Uuid;

const AUTH_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthProfile {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("session verifier timed out")]
    Timeout,
    #[error("session verifier request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("session verifier rejected profile")]
    Rejected,
    #[error("session verifier returned invalid uuid")]
    Uuid(#[from] uuid::Error),
}

#[derive(Debug, Deserialize)]
struct HasJoinedProfile {
    id: String,
    name: String,
}

pub async fn verify_has_joined(
    base_url: &str,
    name: &str,
    server_hash: &str,
) -> Result<AuthProfile, AuthError> {
    let url = format!(
        "{}/session/minecraft/hasJoined",
        base_url.trim_end_matches('/')
    );
    let client = reqwest::Client::new();
    let response = timeout(
        AUTH_TIMEOUT,
        client
            .get(url)
            .query(&[("username", name), ("serverId", server_hash)])
            .send(),
    )
    .await
    .map_err(|_| AuthError::Timeout)??;
    if !response.status().is_success() {
        return Err(AuthError::Rejected);
    }
    parse_has_joined_profile(response.text().await?.as_bytes())
}

pub fn server_hash(shared_secret: &[u8], public_key: &[u8]) -> String {
    let mut sha1 = Sha1::new();
    sha1.update([]);
    sha1.update(shared_secret);
    sha1.update(public_key);
    signed_hex(&sha1.finalize())
}

fn signed_hex(bytes: &[u8]) -> String {
    if bytes.iter().all(|byte| *byte == 0) {
        return "0".to_string();
    }
    if bytes[0] & 0x80 == 0 {
        return trim_hex(bytes);
    }
    let mut magnitude: Vec<u8> = bytes.iter().map(|byte| !byte).collect();
    for byte in magnitude.iter_mut().rev() {
        let (value, carry) = byte.overflowing_add(1);
        *byte = value;
        if !carry {
            break;
        }
    }
    format!("-{}", trim_hex(&magnitude))
}

fn trim_hex(bytes: &[u8]) -> String {
    let first = bytes
        .iter()
        .position(|byte| *byte != 0)
        .unwrap_or(bytes.len() - 1);
    let mut out = format!("{:x}", bytes[first]);
    for byte in &bytes[first + 1..] {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn parse_mojang_uuid(value: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(value)
}

fn parse_has_joined_profile(data: &[u8]) -> Result<AuthProfile, AuthError> {
    let profile: HasJoinedProfile =
        serde_json::from_slice(data).map_err(|_| AuthError::Rejected)?;
    if profile.id.is_empty() || profile.name.is_empty() {
        return Err(AuthError::Rejected);
    }
    Ok(AuthProfile {
        uuid: parse_mojang_uuid(&profile.id)?,
        name: profile.name,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_has_joined_profile, parse_mojang_uuid, server_hash, signed_hex};

    #[test]
    fn server_hash_formatting_is_deterministic() {
        assert_eq!(signed_hex(&[0, 0, 0, 0, 1]), "1");
        assert_eq!(signed_hex(&[0xff; 20]), "-1");
        assert_eq!(
            server_hash(b"secret", b"key"),
            server_hash(b"secret", b"key")
        );
    }

    #[test]
    fn mojang_uuid_parser_accepts_compact_uuid() {
        let uuid = parse_mojang_uuid("00112233445566778899aabbccddeeff").unwrap();
        assert_eq!(uuid.to_string(), "00112233-4455-6677-8899-aabbccddeeff");
    }

    #[test]
    fn has_joined_profile_parser_accepts_success_and_rejects_missing() {
        let profile =
            parse_has_joined_profile(br#"{"id":"00112233445566778899aabbccddeeff","name":"P"}"#)
                .unwrap();
        assert_eq!(
            profile.uuid.to_string(),
            "00112233-4455-6677-8899-aabbccddeeff"
        );
        assert!(parse_has_joined_profile(br#"{"name":"P"}"#).is_err());
        assert!(parse_has_joined_profile(br#"{"error":"Forbidden"}"#).is_err());
    }
}
