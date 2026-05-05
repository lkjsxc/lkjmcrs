use crate::protocol::nbt::{self, Compound, Tag};
use crate::protocol::registry_damage;
use crate::protocol::registry_variants::{
    asset_variant, model_asset_variant, painting_variant, timeline_day, wolf_sound_variant,
    wolf_variant,
};
use crate::protocol::registry_world::{overworld_dimension, plains_biome};

pub const DIMENSION_REGISTRY: &str = "minecraft:dimension_type";
pub const BIOME_REGISTRY: &str = "minecraft:worldgen/biome";
pub const DAMAGE_TYPE_REGISTRY: &str = "minecraft:damage_type";
pub const TIMELINE_REGISTRY: &str = "minecraft:timeline";

pub struct RegistryData {
    pub id: &'static str,
    pub entries: Vec<RegistryEntry>,
    pub tags: Vec<TagGroup>,
}

pub struct RegistryEntry {
    pub key: &'static str,
    pub value: Compound,
}

pub struct TagGroup {
    pub name: &'static str,
    pub entries: &'static [i32],
}

pub fn required_registries() -> Vec<RegistryData> {
    vec![
        one_entry(
            DIMENSION_REGISTRY,
            "minecraft:overworld",
            overworld_dimension(),
            vec![],
        ),
        one_entry(BIOME_REGISTRY, "minecraft:plains", plains_biome(), vec![]),
        registry_damage::damage_type_registry(),
        one_entry(
            "minecraft:cat_variant",
            "minecraft:all_black",
            asset_variant("minecraft:entity/cat/all_black"),
            vec![],
        ),
        one_entry(
            "minecraft:chicken_variant",
            "minecraft:cold",
            model_asset_variant("cold", "minecraft:entity/chicken/cold_chicken"),
            vec![],
        ),
        one_entry(
            "minecraft:cow_variant",
            "minecraft:cold",
            model_asset_variant("cold", "minecraft:entity/cow/cold_cow"),
            vec![],
        ),
        one_entry(
            "minecraft:frog_variant",
            "minecraft:cold",
            asset_variant("minecraft:entity/frog/cold_frog"),
            vec![],
        ),
        one_entry(
            "minecraft:painting_variant",
            "minecraft:alban",
            painting_variant(),
            vec![],
        ),
        one_entry(
            "minecraft:pig_variant",
            "minecraft:cold",
            model_asset_variant("cold", "minecraft:entity/pig/cold_pig"),
            vec![],
        ),
        one_entry(
            TIMELINE_REGISTRY,
            "minecraft:day",
            timeline_day(),
            vec![tag("minecraft:in_overworld", &[0])],
        ),
        one_entry(
            "minecraft:wolf_sound_variant",
            "minecraft:angry",
            wolf_sound_variant(),
            vec![],
        ),
        one_entry(
            "minecraft:wolf_variant",
            "minecraft:ashen",
            wolf_variant(),
            vec![],
        ),
        one_entry(
            "minecraft:zombie_nautilus_variant",
            "minecraft:temperate",
            asset_variant("minecraft:entity/nautilus/zombie_nautilus"),
            vec![],
        ),
    ]
}

pub fn registry(
    id: &'static str,
    entries: Vec<RegistryEntry>,
    tags: Vec<TagGroup>,
) -> RegistryData {
    RegistryData { id, entries, tags }
}

pub fn entry(key: &'static str, value: Compound) -> RegistryEntry {
    RegistryEntry { key, value }
}

pub fn tag(name: &'static str, entries: &'static [i32]) -> TagGroup {
    TagGroup { name, entries }
}

fn one_entry(
    id: &'static str,
    key: &'static str,
    value: Compound,
    tags: Vec<TagGroup>,
) -> RegistryData {
    registry(id, vec![entry(key, value)], tags)
}

pub fn damage_type(
    message_id: &'static str,
    exhaustion: f32,
    effects: Option<&'static str>,
    death_message_type: Option<&'static str>,
) -> Compound {
    let mut values = vec![
        ("message_id", nbt::string(message_id)),
        ("scaling", nbt::string("when_caused_by_living_non_player")),
        ("exhaustion", Tag::Float(exhaustion)),
    ];
    if let Some(effects) = effects {
        values.push(("effects", nbt::string(effects)));
    }
    if let Some(message_type) = death_message_type {
        values.push(("death_message_type", nbt::string(message_type)));
    }
    nbt::compound(values)
}
