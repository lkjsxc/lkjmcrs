use crate::protocol::codec;
use crate::protocol::registry::{encode_registry_data, encode_tags};
use crate::protocol::registry_values::{TIMELINE_REGISTRY, required_registries};
use std::collections::HashSet;
use std::io::{Cursor, Read};

#[test]
fn registry_data_declares_required_non_empty_registries() {
    let packets = encode_registry_data();
    let required = required_registries();
    assert_eq!(packets.len(), required.len());
    for (packet, registry) in packets.into_iter().zip(required) {
        let mut cursor = Cursor::new(packet);
        assert_eq!(codec::read_string(&mut cursor).unwrap(), registry.id);
        assert_eq!(
            codec::read_var_i32(&mut cursor).unwrap(),
            registry.entries.len() as i32
        );
        for entry in registry.entries {
            assert_eq!(codec::read_string(&mut cursor).unwrap(), entry.key);
            let mut present = [0];
            cursor.read_exact(&mut present).unwrap();
            assert_eq!(present, [1]);
            skip_anonymous_compound(&mut cursor);
        }
    }
}

#[test]
fn required_registry_ids_are_unique() {
    let mut seen = HashSet::new();
    for registry in required_registries() {
        assert!(
            seen.insert(registry.id),
            "duplicate registry {}",
            registry.id
        );
    }
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
        skip_tags(&mut cursor, tag_count);
    }
    panic!("missing timeline tag group");
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

fn skip_anonymous_compound(cursor: &mut Cursor<Vec<u8>>) {
    assert_eq!(read_u8(cursor), 10);
    skip_compound_payload(cursor);
}

fn skip_compound_payload(cursor: &mut Cursor<Vec<u8>>) {
    loop {
        let tag = read_u8(cursor);
        if tag == 0 {
            return;
        }
        skip_nbt_string(cursor);
        skip_payload(cursor, tag);
    }
}

fn skip_payload(cursor: &mut Cursor<Vec<u8>>, tag: u8) {
    match tag {
        1 => skip(cursor, 1),
        3 | 5 => skip(cursor, 4),
        6 => skip(cursor, 8),
        8 => skip_nbt_string(cursor),
        10 => skip_compound_payload(cursor),
        other => panic!("unsupported tag {other}"),
    }
}

fn skip_nbt_string(cursor: &mut Cursor<Vec<u8>>) {
    let len = codec::read_u16(cursor).unwrap() as usize;
    skip(cursor, len);
}

fn skip(cursor: &mut Cursor<Vec<u8>>, len: usize) {
    let mut ignored = vec![0; len];
    cursor.read_exact(&mut ignored).unwrap();
}

fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> u8 {
    let mut byte = [0];
    cursor.read_exact(&mut byte).unwrap();
    byte[0]
}
