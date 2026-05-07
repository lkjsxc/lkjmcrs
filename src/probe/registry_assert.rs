use crate::probe::ProbeError;
use crate::protocol::registry_contract::{
    REQUIRED_NON_EMPTY_VARIANT_REGISTRIES, REQUIRED_ONE_ENTRY_REGISTRIES, REQUIRED_REGISTRY_IDS,
    TIMELINE_ENTRY, TIMELINE_OVERWORLD_ENTRY_ID, TIMELINE_OVERWORLD_TAG, TIMELINE_REGISTRY,
};
use crate::protocol::registry_decode::{DecodedRegistry, DecodedTagGroup};
use crate::protocol::{ids, registry_decode};
use tokio::io::AsyncRead;

pub(super) async fn expect_configuration_registries<S>(
    stream: &mut S,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: AsyncRead + Unpin,
{
    let mut registries = Vec::new();
    for _ in 0..REQUIRED_REGISTRY_IDS.len() {
        let packet = super::expect(stream, ids::config::REGISTRY_DATA, "registry data").await?;
        registries.push(registry_decode::decode_registry_data(packet.data)?);
    }
    let packet = super::expect(stream, ids::config::TAGS, "configuration tags").await?;
    let tags = registry_decode::decode_tags(packet.data)?;
    validate_registries(&registries)?;
    validate_tags(&tags)?;
    Ok(())
}

fn validate_registries(registries: &[DecodedRegistry]) -> Result<(), Box<dyn std::error::Error>> {
    let observed: Vec<&str> = registries
        .iter()
        .map(|registry| registry.id.as_str())
        .collect();
    if observed != REQUIRED_REGISTRY_IDS {
        return Err(Box::new(ProbeError::Phase("registry ids")));
    }
    for (registry, key) in REQUIRED_ONE_ENTRY_REGISTRIES {
        let entries = registry_entries(registries, registry)?;
        if entries != [*key] {
            return Err(Box::new(ProbeError::Phase("registry entry keys")));
        }
    }
    for registry in REQUIRED_NON_EMPTY_VARIANT_REGISTRIES {
        if registry_entries(registries, registry)?.is_empty() {
            return Err(Box::new(ProbeError::Phase("empty variant registry")));
        }
    }
    if registry_entries(registries, TIMELINE_REGISTRY)? != [TIMELINE_ENTRY] {
        return Err(Box::new(ProbeError::Phase("timeline registry")));
    }
    Ok(())
}

fn validate_tags(tags: &[DecodedTagGroup]) -> Result<(), Box<dyn std::error::Error>> {
    let observed: Vec<&str> = tags.iter().map(|group| group.registry.as_str()).collect();
    if observed != REQUIRED_REGISTRY_IDS {
        return Err(Box::new(ProbeError::Phase("tag registry ids")));
    }
    let timeline = tags
        .iter()
        .find(|group| group.registry == TIMELINE_REGISTRY)
        .ok_or(ProbeError::Phase("timeline tag group"))?;
    let tag = timeline
        .tags
        .iter()
        .find(|tag| tag.name == TIMELINE_OVERWORLD_TAG)
        .ok_or(ProbeError::Phase("timeline overworld tag"))?;
    if tag.entries != [TIMELINE_OVERWORLD_ENTRY_ID] {
        return Err(Box::new(ProbeError::Phase("timeline tag entries")));
    }
    Ok(())
}

fn registry_entries<'a>(
    registries: &'a [DecodedRegistry],
    id: &str,
) -> Result<Vec<&'a str>, Box<dyn std::error::Error>> {
    registries
        .iter()
        .find(|registry| registry.id == id)
        .map(|registry| registry.entries.iter().map(String::as_str).collect())
        .ok_or_else(|| {
            Box::new(ProbeError::Phase("missing registry")) as Box<dyn std::error::Error>
        })
}
