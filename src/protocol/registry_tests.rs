use crate::protocol::codec;
use crate::protocol::registry::{encode_registry_data, encode_tags};
use crate::protocol::registry_values::{
    DAMAGE_TYPE_REGISTRY, TIMELINE_REGISTRY, required_registries,
};
use std::io::{Cursor, Read};

#[test]
fn registry_data_declares_required_non_empty_registries() {
    let packets = encode_registry_data();
    let required = required_registries();
    assert_eq!(packets.len(), required.len());
    for (packet, required) in packets.into_iter().zip(required) {
        let mut cursor = Cursor::new(packet);
        assert_eq!(codec::read_string(&mut cursor).unwrap(), required.registry);
        assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 1);
        assert_eq!(codec::read_string(&mut cursor).unwrap(), required.key);
    }
}

#[test]
fn required_registry_data_includes_damage_type() {
    assert!(
        required_registries()
            .iter()
            .any(|entry| entry.registry == DAMAGE_TYPE_REGISTRY)
    );
}

#[test]
fn damage_type_registry_packet_declares_in_fire() {
    let packet = registry_packet(DAMAGE_TYPE_REGISTRY);
    let mut cursor = Cursor::new(packet);
    assert_eq!(
        codec::read_string(&mut cursor).unwrap(),
        DAMAGE_TYPE_REGISTRY
    );
    assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 1);
    assert_eq!(
        codec::read_string(&mut cursor).unwrap(),
        "minecraft:in_fire"
    );
}

#[test]
fn damage_type_registry_value_encodes_required_fields() {
    let packet = registry_packet(DAMAGE_TYPE_REGISTRY);
    let mut cursor = Cursor::new(packet);
    assert_eq!(
        codec::read_string(&mut cursor).unwrap(),
        DAMAGE_TYPE_REGISTRY
    );
    assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 1);
    assert_eq!(
        codec::read_string(&mut cursor).unwrap(),
        "minecraft:in_fire"
    );
    let mut present = [0];
    cursor.read_exact(&mut present).unwrap();
    assert_eq!(present, [1]);
    let nbt = &cursor.get_ref()[cursor.position() as usize..];
    for value in [
        "message_id",
        "inFire",
        "scaling",
        "when_caused_by_living_non_player",
        "exhaustion",
        "effects",
        "burning",
    ] {
        assert!(contains_bytes(nbt, value.as_bytes()), "missing {value}");
    }
    assert!(contains_bytes(nbt, &0.1f32.to_be_bytes()));
}

#[test]
fn timeline_tag_binds_in_overworld_to_day_entry() {
    let mut cursor = Cursor::new(encode_tags());
    let groups = codec::read_var_i32(&mut cursor).unwrap();
    for _ in 0..groups {
        let registry = codec::read_string(&mut cursor).unwrap();
        let tag_count = codec::read_var_i32(&mut cursor).unwrap();
        if registry == TIMELINE_REGISTRY {
            assert_eq!(tag_count, 1);
            assert_eq!(
                codec::read_string(&mut cursor).unwrap(),
                "minecraft:in_overworld"
            );
            assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 1);
            assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 0);
            return;
        }
        assert_eq!(tag_count, 0);
    }
    panic!("missing timeline tag group");
}

#[test]
fn damage_type_tags_are_declared_empty() {
    let mut cursor = Cursor::new(encode_tags());
    let groups = codec::read_var_i32(&mut cursor).unwrap();
    for _ in 0..groups {
        let registry = codec::read_string(&mut cursor).unwrap();
        let tag_count = codec::read_var_i32(&mut cursor).unwrap();
        if registry == DAMAGE_TYPE_REGISTRY {
            assert_eq!(tag_count, 0);
            return;
        }
        skip_tags(&mut cursor, tag_count);
    }
    panic!("missing damage type tag group");
}

fn registry_packet(registry: &str) -> Vec<u8> {
    encode_registry_data()
        .into_iter()
        .find(|packet| {
            let mut cursor = Cursor::new(packet.clone());
            codec::read_string(&mut cursor).unwrap() == registry
        })
        .unwrap()
}

fn skip_tags(cursor: &mut Cursor<Vec<u8>>, tag_count: i32) {
    for _ in 0..tag_count {
        let _ = codec::read_string(cursor).unwrap();
        let entries = codec::read_var_i32(cursor).unwrap();
        for _ in 0..entries {
            let _ = codec::read_var_i32(cursor).unwrap();
        }
    }
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
