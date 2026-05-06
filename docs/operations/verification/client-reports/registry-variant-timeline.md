# Registry Variant And Timeline Report

## Sources

- Raw files:
  - `tmp/disconnect-2026-05-05_12.55.20-client.txt`.
  - `tmp/disconnect-2026-05-05_12.56.05-client.txt`.
- Report times: `2026-05-05 12:55:20` and `2026-05-05 12:56:05`.
- Client: Minecraft Java Edition `1.21.11`.
- Client type: Fabric/modded.
- State: configuration registry loading.
- Tested server commit: unknown.
- Evidence class: historical modded evidence that exposed a vanilla registry
  shape issue.

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

These are historical reports. The vanilla client requires non-empty variant
registries even when gameplay does not spawn those entities yet. The overworld
dimension also references `#minecraft:in_overworld`, so the timeline registry
must declare a real entry and bind that tag to it.

## Fixed State

The dynamic registry contract now requires one valid entry for each listed
variant registry, `minecraft:timeline` entry `minecraft:day`, and timeline tag
`minecraft:in_overworld` bound to entry ID `0`.

Regression coverage lives in registry encode tests and the compose `verify`
gate.
