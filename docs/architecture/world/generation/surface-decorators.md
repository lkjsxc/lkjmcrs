# Surface Decorators

## Goal

Define deterministic placement for generated surface details after base terrain,
water, biome, and surface palette stages have completed.

## Owns

- Decoration candidate selection.
- Placement density and spacing rules.
- Substrate and headroom checks for surface decorations.
- Generated marker emission for decoration-origin blocks.

## Inputs

- Final generated surface columns from
  [surface-palettes.md](surface-palettes.md).
- Biome decoration profile from
  [biome-pipeline.md](biome-pipeline.md).
- Hydrology and coast constraints from [hydrology.md](hydrology.md) and
  [oceans-and-coasts.md](oceans-and-coasts.md).
- Absolute block coordinates.
- `world_seed`.

## Outputs

- Additional generated block states above or near the surface.
- Optional generated-content markers for later inspection.

## Rules

1. Decorators run after base terrain and before persisted overrides are applied.
2. Placement must be deterministic and chunk-neighbor safe.
3. Decorators must not overwrite static water, bedrock, or persisted edits.
4. Decorators must check support blocks and headroom before placement.
5. New decoration block states require protocol and storage ownership before
   use.
6. Tree, ore, plant, and structure families need their own owner docs before
   they become broad systems.

## Verification

- Golden tests include fixed-seed decoration presence and absence samples.
- Border property tests cover decorations crossing chunk boundaries or near
  edges.
- Live chunk smoke verifies encoded decoration blocks only after their block
  states are owned.

## Out Of Scope

- Growth ticks, decay, drops, loot, and mob interactions.
- Structure generation.
- Runtime placement behavior by players.
