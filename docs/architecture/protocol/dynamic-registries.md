# Dynamic Registries

## Packet Shape

Configuration sends one `registry_data` packet per dynamic registry.
Each packet contains:

- registry ID string,
- VarInt entry count,
- entry key string,
- value-present boolean,
- anonymous compound NBT value when present.

The tags packet sends one tag group for each declared dynamic registry.
Registry order is stable and test-covered.

## Coverage Policy

Registry data is evidence-driven and minimal. It is not full vanilla registry
coverage unless a doc explicitly changes the milestone target.

Use client reports, first-party probes, and vanilla datapack or class evidence
to decide which entries belong in the first milestone. Fabric or modded reports
are valid only when they expose a vanilla protocol gap.

## Damage Type Bootstrap Set

`minecraft:damage_type` must be a grouped registry packet. The client constructs
its `DamageSources` while handling play login, so these vanilla damage source
keys must exist before the play login packet is sent:

`in_fire`, `campfire`, `lightning_bolt`, `on_fire`, `lava`, `hot_floor`,
`in_wall`, `cramming`, `drown`, `starve`, `cactus`, `fall`, `ender_pearl`,
`fly_into_wall`, `out_of_world`, `generic`, `magic`, `wither`,
`dragon_breath`, `dry_out`, `sweet_berry_bush`, `freeze`, `stalagmite`,
`outside_border`, and `generic_kill`.

The values are hand-authored from the vanilla `1.21.11` datapack JSON files
under `data/minecraft/damage_type/`. The set is confirmed by inspecting the
`DamageSources` constructor in the local `1.21.11` server jar cache.

## Damage Type Tags

The first milestone declares the `minecraft:damage_type` tag group with zero
tags. Add real damage tags only when gameplay behavior requires them.
