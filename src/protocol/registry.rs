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
