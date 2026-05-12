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

## First Decorator Family

- Spruce-style wood terrain is the first promoted decorator family.
- Candidate trunks use `minecraft:spruce_log[axis=y]`.
- Candidate leaf blocks use
  `minecraft:spruce_leaves[distance=7,persistent=true,waterlogged=false]`.
- Candidate roots are deterministic from `world_seed` and absolute
  coordinates.
- Forest root density targets roughly one candidate per `6x6` blocks before
  slope and safety rejection.
- The promoted density target is about three times the compact spruce baseline
  density.
- Moist lowland plains may place sparse companion trees so forests spread
  across readable land instead of appearing as isolated patches.
- Placement requires dry grass surface, moderate local slope, and enough
  vertical headroom for a richer spruce form.
- Tree forms may vary trunk height, crown height, and crown radius while using
  only owned spruce log and spruce leaf states.
- Placement skips ocean, river, beach, static water, stone highlands, and a
  small exclusion area around the resolved spawn column.

## Rules

1. Decorators run after base terrain and before persisted overrides are applied.
2. Placement must be deterministic and chunk-neighbor safe.
3. Decorators must not overwrite static water, bedrock, or persisted edits.
4. Decorators must check support blocks and headroom before placement.
5. New decoration block states require protocol and storage ownership before
   use.
6. Broad tree, ore, plant, and structure systems need their own owner docs
   before they replace the first compact spruce-style decorator family.

## Verification

- Golden tests include fixed-seed decoration presence and absence samples.
- Density tests count generated spruce trunks over fixed seed areas.
- Border property tests cover decorations crossing chunk boundaries or near
  edges.
- Live chunk smoke verifies encoded decoration blocks only after their block
  states are owned.

## Out Of Scope

- Growth ticks, decay, drops, loot, and mob interactions.
- Structure generation.
- Runtime placement behavior by players.
