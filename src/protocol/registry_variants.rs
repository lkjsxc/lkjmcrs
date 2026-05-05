use crate::protocol::nbt::{self, Compound, Tag};

pub fn asset_variant(asset_id: &str) -> Compound {
    nbt::compound(vec![("asset_id", nbt::string(asset_id))])
}

pub fn model_asset_variant(model: &str, asset_id: &str) -> Compound {
    nbt::compound(vec![
        ("model", nbt::string(model)),
        ("asset_id", nbt::string(asset_id)),
    ])
}

pub fn painting_variant() -> Compound {
    nbt::compound(vec![
        (
            "author",
            text_component("gray", "painting.minecraft.alban.author"),
        ),
        ("width", Tag::Int(1)),
        ("asset_id", nbt::string("minecraft:alban")),
        (
            "title",
            text_component("yellow", "painting.minecraft.alban.title"),
        ),
        ("height", Tag::Int(1)),
    ])
}

pub fn timeline_day() -> Compound {
    nbt::compound(vec![
        ("period_ticks", Tag::Int(24000)),
        ("tracks", Tag::Compound(nbt::compound(vec![]))),
    ])
}

pub fn wolf_sound_variant() -> Compound {
    nbt::compound(vec![
        (
            "ambient_sound",
            nbt::string("minecraft:entity.wolf_angry.ambient"),
        ),
        (
            "hurt_sound",
            nbt::string("minecraft:entity.wolf_angry.hurt"),
        ),
        (
            "death_sound",
            nbt::string("minecraft:entity.wolf_angry.death"),
        ),
        (
            "whine_sound",
            nbt::string("minecraft:entity.wolf_angry.whine"),
        ),
        (
            "growl_sound",
            nbt::string("minecraft:entity.wolf_angry.growl"),
        ),
        (
            "pant_sound",
            nbt::string("minecraft:entity.wolf_angry.pant"),
        ),
    ])
}

pub fn wolf_variant() -> Compound {
    nbt::compound(vec![(
        "assets",
        Tag::Compound(nbt::compound(vec![
            ("tame", nbt::string("minecraft:entity/wolf/wolf_ashen_tame")),
            (
                "angry",
                nbt::string("minecraft:entity/wolf/wolf_ashen_angry"),
            ),
            ("wild", nbt::string("minecraft:entity/wolf/wolf_ashen")),
        ])),
    )])
}

fn text_component(color: &str, translate: &str) -> Tag {
    Tag::Compound(nbt::compound(vec![
        ("color", nbt::string(color)),
        ("translate", nbt::string(translate)),
    ]))
}
