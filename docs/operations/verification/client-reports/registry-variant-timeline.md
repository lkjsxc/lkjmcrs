# Registry Variant And Timeline Report

## Source

- Raw file: `tmp/disconnect-2026-05-05_12.55.20-client.txt`.
- Report time: `2026-05-05 12:55:20`.
- Client: Minecraft Java Edition `1.21.11`.
- State: configuration registry loading.

## Client Failure

The client failed registry loading because these root registries were empty:

- `minecraft:cat_variant`
- `minecraft:chicken_variant`
- `minecraft:cow_variant`
- `minecraft:frog_variant`
- `minecraft:painting_variant`
- `minecraft:pig_variant`
- `minecraft:wolf_sound_variant`
- `minecraft:wolf_variant`
- `minecraft:zombie_nautilus_variant`

The report also showed `minecraft:timeline` had unbound tag
`minecraft:in_overworld`.

## Interpretation

This is historical evidence. The vanilla client requires non-empty variant
registries even when gameplay does not spawn those entities yet. The overworld
dimension also references `#minecraft:in_overworld`, so the timeline registry
must declare a real entry and bind that tag to it.

## Fixed State

The protocol contract now requires one valid entry for each listed variant
registry, `minecraft:timeline` entry `minecraft:day`, and timeline tag
`minecraft:in_overworld` bound to entry ID `0`.

Regression coverage lives in registry encode tests and the compose `verify`
gate.
