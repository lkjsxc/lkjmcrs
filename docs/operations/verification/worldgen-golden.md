# Worldgen Golden

## Goal

Lock deterministic terrain output for selected chunks without coupling tests to
private runtime state.

## Scope

- Verify `natural` terrain with a fixed `world_seed`.
- Verify `flat` terrain remains selectable for scale regression probes.
- Cover spawn-near chunks and representative outer chunks.
- Cover final generated blocks before sparse persisted overrides are applied.
- Cover static water, riverbeds, banks, and underground cave air when those
  features are active generated terrain.
- Cover generated spruce-style decorator density over a fixed seed area.

## Golden Inputs

- Terrain generator name.
- World seed.
- Chunk coordinate.
- Expected sampled block states or column heights.
- Expected result for at least one chunk adjacent to a sampled golden chunk.
- Expected enclosed underground `Air` for cave-enabled natural terrain.
- Expected water top height for at least one static water sample.
- Expected generated spruce trunk count over a fixed radius.

## Acceptance Rules

1. Golden data must be small enough to review.
2. Golden checks must fail when a deterministic terrain formula changes.
3. Formula changes must update this doc or a linked result note before code.
4. Golden checks do not prove client rendering; wire probes still own that.

## Gate Command

- Static gate: `cargo test world::terrain`.
- Live evidence stays with [terrain-generation-smoke.md](terrain-generation-smoke.md)
  [terrain-rivers-smoke.md](terrain-rivers-smoke.md),
  [terrain-caves-smoke.md](terrain-caves-smoke.md), and
  [render-distance-smoke.md](render-distance-smoke.md).
- Compose services may wrap the same golden assertions, but the required gate
  is active when the Rust golden test exists.
