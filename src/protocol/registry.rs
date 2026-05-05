use crate::protocol::codec;
use crate::protocol::nbt::Compound;
use crate::protocol::registry_values::{self, TIMELINE_REGISTRY};

pub fn encode_registry_data() -> Vec<Vec<u8>> {
    registry_values::required_registries()
        .into_iter()
        .map(|entry| encode_registry(entry.registry, entry.key, &entry.value))
        .collect()
}

pub fn encode_tags() -> Vec<u8> {
    let registries = registry_values::required_registries();
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, registries.len() as i32);
    for entry in registries {
        if entry.registry == TIMELINE_REGISTRY {
            write_tag_group(
                &mut out,
                entry.registry,
                &[("minecraft:in_overworld", &[0])],
            );
        } else {
            write_tag_group(&mut out, entry.registry, &[]);
        }
    }
    out
}

fn encode_registry(id: &str, key: &str, value: &Compound) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_string(&mut out, id);
    codec::write_var_i32(&mut out, 1);
    codec::write_string(&mut out, key);
    codec::write_bool(&mut out, true);
    crate::protocol::nbt::write_anonymous_compound(&mut out, value);
    out
}

fn write_tag_group(out: &mut Vec<u8>, registry: &str, tags: &[(&str, &[i32])]) {
    codec::write_string(out, registry);
    codec::write_var_i32(out, tags.len() as i32);
    for (name, entries) in tags {
        codec::write_string(out, name);
        codec::write_var_i32(out, entries.len() as i32);
        for entry in *entries {
            codec::write_var_i32(out, *entry);
        }
    }
}

pub fn registry_packet_count() -> usize {
    registry_values::required_registries().len()
}

#[cfg(test)]
mod tests {
    use super::{encode_registry_data, encode_tags};
    use crate::protocol::codec;
    use crate::protocol::registry_values::{TIMELINE_REGISTRY, required_registries};
    use std::io::Cursor;

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
}
