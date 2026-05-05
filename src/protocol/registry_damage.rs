use crate::protocol::registry_values::{
    DAMAGE_TYPE_REGISTRY, RegistryData, damage_type, entry, registry,
};

const FALL_VARIANTS: &str = "fall_variants";

struct DamageTypeSpec {
    key: &'static str,
    message_id: &'static str,
    exhaustion: f32,
    effects: Option<&'static str>,
    death_message_type: Option<&'static str>,
}

pub fn damage_type_registry() -> RegistryData {
    registry(
        DAMAGE_TYPE_REGISTRY,
        damage_type_specs()
            .into_iter()
            .map(|spec| {
                entry(
                    spec.key,
                    damage_type(
                        spec.message_id,
                        spec.exhaustion,
                        spec.effects,
                        spec.death_message_type,
                    ),
                )
            })
            .collect(),
        vec![],
    )
}

fn damage_type_specs() -> Vec<DamageTypeSpec> {
    vec![
        damage("minecraft:in_fire", "inFire", 0.1, Some("burning"), None),
        damage("minecraft:campfire", "inFire", 0.1, Some("burning"), None),
        damage("minecraft:lightning_bolt", "lightningBolt", 0.1, None, None),
        damage("minecraft:on_fire", "onFire", 0.0, Some("burning"), None),
        damage("minecraft:lava", "lava", 0.1, Some("burning"), None),
        damage(
            "minecraft:hot_floor",
            "hotFloor",
            0.1,
            Some("burning"),
            None,
        ),
        damage("minecraft:in_wall", "inWall", 0.0, None, None),
        damage("minecraft:cramming", "cramming", 0.0, None, None),
        damage("minecraft:drown", "drown", 0.0, Some("drowning"), None),
        damage("minecraft:starve", "starve", 0.0, None, None),
        damage("minecraft:cactus", "cactus", 0.1, None, None),
        damage("minecraft:fall", "fall", 0.0, None, Some(FALL_VARIANTS)),
        damage(
            "minecraft:ender_pearl",
            "fall",
            0.0,
            None,
            Some(FALL_VARIANTS),
        ),
        damage("minecraft:fly_into_wall", "flyIntoWall", 0.0, None, None),
        damage("minecraft:out_of_world", "outOfWorld", 0.0, None, None),
        damage("minecraft:generic", "generic", 0.0, None, None),
        damage("minecraft:magic", "magic", 0.0, None, None),
        damage("minecraft:wither", "wither", 0.0, None, None),
        damage("minecraft:dragon_breath", "dragonBreath", 0.0, None, None),
        damage("minecraft:dry_out", "dryout", 0.1, None, None),
        damage(
            "minecraft:sweet_berry_bush",
            "sweetBerryBush",
            0.1,
            Some("poking"),
            None,
        ),
        damage("minecraft:freeze", "freeze", 0.0, Some("freezing"), None),
        damage("minecraft:stalagmite", "stalagmite", 0.0, None, None),
        damage("minecraft:outside_border", "outsideBorder", 0.0, None, None),
        damage("minecraft:generic_kill", "genericKill", 0.0, None, None),
    ]
}

fn damage(
    key: &'static str,
    message_id: &'static str,
    exhaustion: f32,
    effects: Option<&'static str>,
    death_message_type: Option<&'static str>,
) -> DamageTypeSpec {
    DamageTypeSpec {
        key,
        message_id,
        exhaustion,
        effects,
        death_message_type,
    }
}
