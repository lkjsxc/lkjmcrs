use crate::protocol::MINECRAFT_VERSION;
use crate::protocol::codec::{self, CodecError};
pub use crate::protocol::registry::{encode_registry_data, encode_tags, registry_packet_count};
use std::io::Cursor;

pub const VANILLA_FEATURE: &str = "minecraft:vanilla";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl KnownPack {
    pub fn vanilla_core() -> Self {
        Self {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: MINECRAFT_VERSION.to_string(),
        }
    }

    fn encode(&self, out: &mut Vec<u8>) {
        codec::write_string(out, &self.namespace);
        codec::write_string(out, &self.id);
        codec::write_string(out, &self.version);
    }

    fn decode(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, CodecError> {
        let namespace = codec::read_string(cursor)?;
        let id = codec::read_string(cursor)?;
        let version = codec::read_string(cursor)?;
        Ok(Self {
            namespace,
            id,
            version,
        })
    }
}

pub fn encode_select_known_packs() -> Vec<u8> {
    encode_known_packs(&[KnownPack::vanilla_core()])
}

pub fn encode_known_packs(packs: &[KnownPack]) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, packs.len() as i32);
    for pack in packs {
        pack.encode(&mut out);
    }
    out
}

pub fn decode_known_packs(data: Vec<u8>) -> Result<Vec<KnownPack>, CodecError> {
    let mut cursor = Cursor::new(data);
    let count = codec::read_var_i32(&mut cursor)?;
    if count < 0 {
        return Err(CodecError::NegativeLength);
    }
    let mut packs = Vec::with_capacity(count as usize);
    for _ in 0..count {
        packs.push(KnownPack::decode(&mut cursor)?);
    }
    Ok(packs)
}

pub fn encode_enabled_features() -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, 1);
    codec::write_string(&mut out, VANILLA_FEATURE);
    out
}

#[cfg(test)]
mod tests {
    use super::{
        KnownPack, VANILLA_FEATURE, decode_known_packs, encode_enabled_features,
        encode_select_known_packs,
    };

    #[test]
    fn select_known_packs_encodes_vanilla_core() {
        let payload = encode_select_known_packs();
        assert_eq!(payload, b"\x01\x09minecraft\x04core\x071.21.11".to_vec());
    }

    #[test]
    fn known_packs_decode_round_trips_vanilla_core() {
        let packs = decode_known_packs(encode_select_known_packs()).unwrap();
        assert_eq!(packs, vec![KnownPack::vanilla_core()]);
    }

    #[test]
    fn enabled_features_encodes_vanilla_flag() {
        let payload = encode_enabled_features();
        let mut expected = vec![1, VANILLA_FEATURE.len() as u8];
        expected.extend_from_slice(VANILLA_FEATURE.as_bytes());
        assert_eq!(payload, expected);
    }
}
