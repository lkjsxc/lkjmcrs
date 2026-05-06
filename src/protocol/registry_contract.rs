pub const REQUIRED_REGISTRY_IDS: &[&str] = &[
    "minecraft:dimension_type",
    "minecraft:worldgen/biome",
    "minecraft:damage_type",
    "minecraft:cat_variant",
    "minecraft:chicken_variant",
    "minecraft:cow_variant",
    "minecraft:frog_variant",
    "minecraft:painting_variant",
    "minecraft:pig_variant",
    "minecraft:timeline",
    "minecraft:wolf_sound_variant",
    "minecraft:wolf_variant",
    "minecraft:zombie_nautilus_variant",
];

pub const REQUIRED_ONE_ENTRY_REGISTRIES: &[(&str, &str)] = &[
    ("minecraft:dimension_type", "minecraft:overworld"),
    ("minecraft:worldgen/biome", "minecraft:plains"),
    ("minecraft:cat_variant", "minecraft:all_black"),
    ("minecraft:chicken_variant", "minecraft:cold"),
    ("minecraft:cow_variant", "minecraft:cold"),
    ("minecraft:frog_variant", "minecraft:cold"),
    ("minecraft:painting_variant", "minecraft:alban"),
    ("minecraft:pig_variant", "minecraft:cold"),
    ("minecraft:timeline", "minecraft:day"),
    ("minecraft:wolf_sound_variant", "minecraft:angry"),
    ("minecraft:wolf_variant", "minecraft:ashen"),
    ("minecraft:zombie_nautilus_variant", "minecraft:temperate"),
];

pub const REQUIRED_NON_EMPTY_VARIANT_REGISTRIES: &[&str] = &[
    "minecraft:cat_variant",
    "minecraft:chicken_variant",
    "minecraft:cow_variant",
    "minecraft:frog_variant",
    "minecraft:painting_variant",
    "minecraft:pig_variant",
    "minecraft:wolf_sound_variant",
    "minecraft:wolf_variant",
    "minecraft:zombie_nautilus_variant",
];

pub const TIMELINE_REGISTRY: &str = "minecraft:timeline";
pub const TIMELINE_ENTRY: &str = "minecraft:day";
pub const TIMELINE_OVERWORLD_TAG: &str = "minecraft:in_overworld";
pub const TIMELINE_OVERWORLD_ENTRY_ID: i32 = 0;
