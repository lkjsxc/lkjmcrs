use crate::protocol::nbt::{self, Compound, Tag};

pub fn overworld_dimension() -> Compound {
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

pub fn plains_biome() -> Compound {
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
