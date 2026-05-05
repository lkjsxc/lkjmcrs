use crate::protocol::nbt::{self, Compound, Tag};
use crate::protocol::registry_variants::{
    asset_variant, model_asset_variant, painting_variant, timeline_day, wolf_sound_variant,
    wolf_variant,
};

pub const DIMENSION_REGISTRY: &str = "minecraft:dimension_type";
pub const BIOME_REGISTRY: &str = "minecraft:worldgen/biome";
pub const TIMELINE_REGISTRY: &str = "minecraft:timeline";

pub struct RegistryEntry {
    pub registry: &'static str,
    pub key: &'static str,
    pub value: Compound,
}

pub fn required_registries() -> Vec<RegistryEntry> {
    vec![
        entry(
            DIMENSION_REGISTRY,
            "minecraft:overworld",
            overworld_dimension(),
        ),
        entry(BIOME_REGISTRY, "minecraft:plains", plains_biome()),
        entry(
            "minecraft:cat_variant",
            "minecraft:all_black",
            asset_variant("minecraft:entity/cat/all_black"),
        ),
        entry(
            "minecraft:chicken_variant",
            "minecraft:cold",
            model_asset_variant("cold", "minecraft:entity/chicken/cold_chicken"),
        ),
        entry(
            "minecraft:cow_variant",
            "minecraft:cold",
            model_asset_variant("cold", "minecraft:entity/cow/cold_cow"),
        ),
        entry(
            "minecraft:frog_variant",
            "minecraft:cold",
            asset_variant("minecraft:entity/frog/cold_frog"),
        ),
        entry(
            "minecraft:painting_variant",
            "minecraft:alban",
            painting_variant(),
        ),
        entry(
            "minecraft:pig_variant",
            "minecraft:cold",
            model_asset_variant("cold", "minecraft:entity/pig/cold_pig"),
        ),
        entry(TIMELINE_REGISTRY, "minecraft:day", timeline_day()),
        entry(
            "minecraft:wolf_sound_variant",
            "minecraft:angry",
            wolf_sound_variant(),
        ),
        entry("minecraft:wolf_variant", "minecraft:ashen", wolf_variant()),
        entry(
            "minecraft:zombie_nautilus_variant",
            "minecraft:temperate",
            asset_variant("minecraft:entity/nautilus/zombie_nautilus"),
        ),
    ]
}

fn entry(registry: &'static str, key: &'static str, value: Compound) -> RegistryEntry {
    RegistryEntry {
        registry,
        key,
        value,
    }
}

fn overworld_dimension() -> Compound {
    nbt::compound(vec![
        ("timelines", nbt::string("#minecraft:in_overworld")),
        ("ambient_light", Tag::Float(0.0)),
        ("monster_spawn_block_light_limit", Tag::Int(0)),
        ("coordinate_scale", Tag::Double(1.0)),
        ("logical_height", Tag::Int(384)),
        ("infiniburn", nbt::string("#minecraft:infiniburn_overworld")),
        ("attributes", visual_attributes()),
        ("min_y", Tag::Int(-64)),
        ("monster_spawn_light_level", uniform_light_level()),
        ("has_ceiling", Tag::Byte(0)),
        ("has_skylight", Tag::Byte(1)),
        ("height", Tag::Int(384)),
    ])
}

fn visual_attributes() -> Tag {
    Tag::Compound(nbt::compound(vec![
        ("minecraft:visual/fog_color", nbt::string("#c0d8ff")),
        ("minecraft:visual/cloud_height", Tag::Float(192.33)),
        ("minecraft:visual/sky_color", nbt::string("#78a7ff")),
        ("minecraft:visual/cloud_color", nbt::string("#ccffffff")),
    ]))
}

fn uniform_light_level() -> Tag {
    Tag::Compound(nbt::compound(vec![
        ("min_inclusive", Tag::Int(0)),
        ("max_inclusive", Tag::Int(7)),
        ("type", nbt::string("minecraft:uniform")),
    ]))
}

fn plains_biome() -> Compound {
    nbt::compound(vec![
        (
            "effects",
            Tag::Compound(nbt::compound(vec![("water_color", nbt::string("#3f76e4"))])),
        ),
        ("has_precipitation", Tag::Byte(1)),
        ("temperature", Tag::Float(0.8)),
        ("downfall", Tag::Float(0.4)),
        (
            "attributes",
            Tag::Compound(nbt::compound(vec![(
                "minecraft:visual/sky_color",
                nbt::string("#78a7ff"),
            )])),
        ),
    ])
}
