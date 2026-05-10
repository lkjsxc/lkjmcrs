# Macro Fields

## Goal

Define the deterministic low-frequency fields that shape natural terrain before
specific water, biome, surface, cave, or decorator stages run.

## Owns

- Continentalness-like landmass tendency.
- Ridge and valley shape.
- Erosion-like smoothing.
- Temperature and humidity base fields.
- Height influence fields sampled by later stages.

## Inputs

- `world_seed`.
- Absolute block or column coordinates.
- Generator profile name selected by runtime construction.

## Outputs

- Stable scalar field values for each sampled column.
- Derived terrain-height hints for the base natural generator.
- Climate hints consumed by [biome-pipeline.md](biome-pipeline.md).
- Shape hints consumed by [hydrology.md](hydrology.md) and
  [oceans-and-coasts.md](oceans-and-coasts.md).

## Rules

1. Field sampling must not read persisted chunks or player overrides.
2. Field values must be continuous across chunk borders.
3. Column decisions may sample a bounded local neighborhood only.
4. Field names describe behavior, not third-party pack compatibility.
5. Field changes are deterministic formula changes and require golden updates.

## Verification

- Golden terrain tests cover fixed-seed representative chunks.
- Border property tests compare columns generated from adjacent chunks.
- Spawn-near coverage checks that macro fields do not force an artificial flat
  plateau around origin.

## Out Of Scope

- Final block-state selection.
- Biome registry IDs.
- Rivers, oceans, caves, decorators, ores, structures, and mobs.
