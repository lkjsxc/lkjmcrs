use crate::protocol::codec;
use crate::protocol::registry::{encode_registry_data, encode_tags};
use crate::protocol::registry_values::DAMAGE_TYPE_REGISTRY;
use std::io::{Cursor, Read};

#[test]
fn damage_type_registry_declares_bootstrap_entries_in_order() {
    let entries = damage_type_entries();
    assert_eq!(entry_keys(&entries), DAMAGE_TYPE_KEYS);
}

#[test]
fn campfire_damage_type_uses_vanilla_in_fire_payload() {
    let value = entry_value("minecraft:campfire");
    assert_contains(&value, b"message_id");
    assert_contains(&value, b"inFire");
    assert_contains(&value, b"effects");
    assert_contains(&value, b"burning");
    assert_contains(&value, &0.1f32.to_be_bytes());
}

#[test]
fn special_damage_type_fields_are_encoded() {
    assert_contains(&entry_value("minecraft:drown"), b"drowning");
    assert_contains(&entry_value("minecraft:freeze"), b"freezing");
    assert_contains(&entry_value("minecraft:sweet_berry_bush"), b"poking");
    assert_contains(&entry_value("minecraft:fall"), b"fall_variants");
    assert_contains(&entry_value("minecraft:ender_pearl"), b"fall_variants");
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

const DAMAGE_TYPE_KEYS: &[&str] = &[
    "minecraft:in_fire",
    "minecraft:campfire",
    "minecraft:lightning_bolt",
    "minecraft:on_fire",
    "minecraft:lava",
    "minecraft:hot_floor",
    "minecraft:in_wall",
    "minecraft:cramming",
    "minecraft:drown",
    "minecraft:starve",
    "minecraft:cactus",
    "minecraft:fall",
    "minecraft:ender_pearl",
    "minecraft:fly_into_wall",
    "minecraft:out_of_world",
    "minecraft:generic",
    "minecraft:magic",
    "minecraft:wither",
    "minecraft:dragon_breath",
    "minecraft:dry_out",
    "minecraft:sweet_berry_bush",
    "minecraft:freeze",
    "minecraft:stalagmite",
    "minecraft:outside_border",
    "minecraft:generic_kill",
];

fn damage_type_entries() -> Vec<(String, Vec<u8>)> {
    let packet = registry_packet(DAMAGE_TYPE_REGISTRY);
    let mut cursor = Cursor::new(packet);
    assert_eq!(
        codec::read_string(&mut cursor).unwrap(),
        DAMAGE_TYPE_REGISTRY
    );
    let count = codec::read_var_i32(&mut cursor).unwrap();
    let mut entries = Vec::new();
    for _ in 0..count {
        let key = codec::read_string(&mut cursor).unwrap();
        let mut present = [0];
        cursor.read_exact(&mut present).unwrap();
        assert_eq!(present, [1]);
        let start = cursor.position() as usize;
        skip_anonymous_compound(&mut cursor);
        let end = cursor.position() as usize;
        entries.push((key, cursor.get_ref()[start..end].to_vec()));
    }
    entries
}

fn entry_keys(entries: &[(String, Vec<u8>)]) -> Vec<&str> {
    entries.iter().map(|(key, _)| key.as_str()).collect()
}

fn entry_value(key: &str) -> Vec<u8> {
    damage_type_entries()
        .into_iter()
        .find(|(entry_key, _)| entry_key == key)
        .map(|(_, value)| value)
        .unwrap_or_else(|| panic!("missing {key}"))
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

fn skip_tags(cursor: &mut Cursor<Vec<u8>>, tag_count: i32) {
    for _ in 0..tag_count {
        let _ = codec::read_string(cursor).unwrap();
        let entries = codec::read_var_i32(cursor).unwrap();
        for _ in 0..entries {
            let _ = codec::read_var_i32(cursor).unwrap();
        }
    }
}

fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> u8 {
    let mut byte = [0];
    cursor.read_exact(&mut byte).unwrap();
    byte[0]
}

fn assert_contains(haystack: &[u8], needle: &[u8]) {
    assert!(
        haystack
            .windows(needle.len())
            .any(|window| window == needle),
        "missing {:?}",
        String::from_utf8_lossy(needle)
    );
}
