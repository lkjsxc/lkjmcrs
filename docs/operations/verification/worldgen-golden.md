# Worldgen Golden

## Goal

Lock deterministic terrain output for selected chunks without coupling tests to
private runtime state.

## Scope

- Verify `natural` terrain with a fixed `world_seed`.
- Verify `flat` terrain remains selectable for scale regression probes.
- Cover spawn safety-core chunks and representative outer chunks.
- Cover final generated blocks before sparse persisted overrides are applied.

## Golden Inputs

- Terrain generator name.
- World seed.
- Chunk coordinate.
- Expected sampled block states or column heights.
- Expected result for at least one boundary chunk near the spawn safety core.

## Acceptance Rules

1. Golden data must be small enough to review.
2. Golden checks must fail when a deterministic terrain formula changes.
3. Formula changes must update this doc or a linked result note before code.
4. Golden checks do not prove client rendering; wire probes still own that.

## Deferred Command

No dedicated compose command is active yet. Until one exists, terrain coverage
comes from [terrain-generation-smoke.md](terrain-generation-smoke.md) and
[render-distance-smoke.md](render-distance-smoke.md).
