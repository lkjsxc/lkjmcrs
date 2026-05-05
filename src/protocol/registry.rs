use crate::protocol::codec;
use crate::protocol::registry_values::{self, RegistryData, TagGroup};

pub fn encode_registry_data() -> Vec<Vec<u8>> {
    registry_values::required_registries()
        .iter()
        .map(encode_registry)
        .collect()
}

pub fn encode_tags() -> Vec<u8> {
    let registries = registry_values::required_registries();
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, registries.len() as i32);
    for registry in registries {
        write_tag_group(&mut out, registry.id, &registry.tags);
    }
    out
}

fn encode_registry(registry: &RegistryData) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_string(&mut out, registry.id);
    codec::write_var_i32(&mut out, registry.entries.len() as i32);
    for entry in &registry.entries {
        codec::write_string(&mut out, entry.key);
        codec::write_bool(&mut out, true);
        crate::protocol::nbt::write_anonymous_compound(&mut out, &entry.value);
    }
    out
}

fn write_tag_group(out: &mut Vec<u8>, registry: &str, tags: &[TagGroup]) {
    codec::write_string(out, registry);
    codec::write_var_i32(out, tags.len() as i32);
    for tag in tags {
        codec::write_string(out, tag.name);
        codec::write_var_i32(out, tag.entries.len() as i32);
        for entry in tag.entries {
            codec::write_var_i32(out, *entry);
        }
    }
}

pub fn registry_packet_count() -> usize {
    registry_values::required_registries().len()
}
