use crate::protocol::registry::{encode_registry_data, encode_tags};
use crate::protocol::registry_contract::{
    REQUIRED_NON_EMPTY_VARIANT_REGISTRIES, REQUIRED_ONE_ENTRY_REGISTRIES, REQUIRED_REGISTRY_IDS,
    TIMELINE_OVERWORLD_ENTRY_ID, TIMELINE_OVERWORLD_TAG, TIMELINE_REGISTRY,
};
use crate::protocol::registry_decode::{self, DecodedRegistry};
use std::collections::HashSet;

#[test]
fn registry_data_declares_required_registry_ids_in_order() {
    let registries = decoded_registries();
    assert_eq!(registry_ids(&registries), REQUIRED_REGISTRY_IDS);
}

#[test]
fn required_one_entry_registries_declare_literal_keys() {
    let registries = decoded_registries();
    for (registry, key) in REQUIRED_ONE_ENTRY_REGISTRIES {
        assert_eq!(registry_entries(&registries, registry), vec![*key]);
    }
}

#[test]
fn required_variant_registries_are_non_empty() {
    let registries = decoded_registries();
    for registry in REQUIRED_NON_EMPTY_VARIANT_REGISTRIES {
        assert!(
            !registry_entries(&registries, registry).is_empty(),
            "empty {registry}"
        );
    }
}

#[test]
fn required_registry_ids_are_unique() {
    let mut seen = HashSet::new();
    for id in registry_ids(&decoded_registries()) {
        assert!(seen.insert(id), "duplicate registry {id}");
    }
}

#[test]
fn timeline_tag_binds_in_overworld_to_day_entry() {
    let groups = registry_decode::decode_tags(encode_tags()).unwrap();
    let timeline = groups
        .iter()
        .find(|group| group.registry == TIMELINE_REGISTRY)
        .expect("missing timeline tag group");
    assert_eq!(timeline.tags.len(), 1);
    assert_eq!(timeline.tags[0].name, TIMELINE_OVERWORLD_TAG);
    assert_eq!(timeline.tags[0].entries, [TIMELINE_OVERWORLD_ENTRY_ID]);
}

#[test]
fn tags_declares_required_registry_ids_in_order() {
    let groups = registry_decode::decode_tags(encode_tags()).unwrap();
    let ids: Vec<&str> = groups.iter().map(|group| group.registry.as_str()).collect();
    assert_eq!(ids, REQUIRED_REGISTRY_IDS);
}

fn decoded_registries() -> Vec<DecodedRegistry> {
    encode_registry_data()
        .into_iter()
        .map(|packet| registry_decode::decode_registry_data(packet).unwrap())
        .collect()
}

fn registry_ids(registries: &[DecodedRegistry]) -> Vec<&str> {
    registries
        .iter()
        .map(|registry| registry.id.as_str())
        .collect()
}

fn registry_entries<'a>(registries: &'a [DecodedRegistry], id: &str) -> Vec<&'a str> {
    registries
        .iter()
        .find(|registry| registry.id == id)
        .unwrap_or_else(|| panic!("missing {id}"))
        .entries
        .iter()
        .map(String::as_str)
        .collect()
}
