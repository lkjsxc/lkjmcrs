# Water And Rivers

## Goal

Define static water and river generation for natural terrain.

## Current Behavior

- Generate `minecraft:water[level=0]` as a static natural-terrain block state.
- Protocol block-state IDs are owned by
  [../protocol/chunk-packets.md](../protocol/chunk-packets.md).
- Internal override storage codes are owned by
  [section-storage.md](section-storage.md).
- Keep water static: no spread, flow updates, buckets, swimming rules, boats,
  aquatic mobs, or underwater gameplay changes.
- Keep rivers deterministic from `world_seed` and absolute block coordinates.
- Preserve chunk-border continuity without reading persisted neighbor chunks.

## Terrain Rules

- River level is `63`.
- Natural terrain may carve shallow riverbeds below the river level.
- Dry surface columns keep grass on top.
- Submerged riverbed columns use dirt or stone at the top, then water above.
- Heightmaps may treat static water as the top non-air block for this slice.
- Spawn resolution must reject water columns and keep solid floor plus
  headroom.
- Caves must not carve through static water columns.

## Verification

- Unit tests guard water block-state mapping and section-storage encoding.
- Golden terrain coverage requires water for a fixed seed and nearby chunks.
- Border property coverage checks river and water continuity across adjacent
  generated chunks.
- Live river terrain smoke decodes chunk data and requires at least one water
  block near spawn.

## Relationship To Richer Terrain

- Rivers remain static generated blocks until fluid simulation is documented.
- River corridors should shape nearby terrain before surface palettes and
  decorators run.
- Ocean, coast, and beach terrain is owned by
  [generation/oceans-and-coasts.md](generation/oceans-and-coasts.md).

## Out Of Scope

- Fluid physics.
- Aquifers and underwater light behavior.
- Placement or breaking behavior specific to water.
