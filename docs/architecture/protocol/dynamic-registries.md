# Dynamic Registries

## Packet Shape

Configuration sends one `registry_data` packet per dynamic registry.
Each packet contains:

- registry ID string,
- VarInt entry count,
- entry key string,
- value-present boolean,
- anonymous compound NBT value when present.

The tags packet sends one tag group for each required dynamic registry.
Registry order is stable and test-covered.

## Coverage Policy

Registry data is evidence-driven and minimal. It is not full vanilla registry
coverage unless a doc explicitly changes the current compatibility target.

Use client reports, first-party probes, and vanilla datapack or class evidence
to decide which entries belong in the current compatibility slice. Fabric or
modded reports are valid only when they expose a vanilla protocol gap.

## Required Registry Set

The current compatibility slice sends one `registry_data` packet for each
registry in this order:

- `minecraft:dimension_type` with `minecraft:overworld` at registry ID `0`.
- `minecraft:worldgen/biome` with `minecraft:plains` at registry ID `0`.
- `minecraft:damage_type` with the bootstrap keys below.
- `minecraft:cat_variant` with `minecraft:all_black` at registry ID `0`.
- `minecraft:chicken_variant` with `minecraft:cold` at registry ID `0`.
- `minecraft:cow_variant` with `minecraft:cold` at registry ID `0`.
- `minecraft:frog_variant` with `minecraft:cold` at registry ID `0`.
- `minecraft:painting_variant` with `minecraft:alban` at registry ID `0`.
- `minecraft:pig_variant` with `minecraft:cold` at registry ID `0`.
- `minecraft:timeline` with `minecraft:day` at registry ID `0`.
- `minecraft:wolf_sound_variant` with `minecraft:angry` at registry ID `0`.
- `minecraft:wolf_variant` with `minecraft:ashen` at registry ID `0`.
- `minecraft:zombie_nautilus_variant` with `minecraft:temperate` at registry
  ID `0`.

The `minecraft:timeline` tag group must bind `minecraft:in_overworld` to entry
ID `0`. Required variant registries must be non-empty even when gameplay does
not spawn those entities yet.

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

The current compatibility slice declares the `minecraft:damage_type` tag group
with zero tags. Add real damage tags only when gameplay behavior requires them.
