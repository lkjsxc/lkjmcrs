# Water And Rivers

## Goal

Define the first static water and river generation slice for natural terrain.

## Current Target

- Add `minecraft:water[level=0]` as a generated block state.
- Use protocol block-state ID `86` for the current Minecraft `1.21.11` target.
- Use internal section-storage code `5` for water.
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

## Verification

- Unit tests guard water block-state mapping and section-storage encoding.
- Golden terrain coverage requires water for a fixed seed and nearby chunks.
- Border property coverage checks river and water continuity across adjacent
  generated chunks.
- Live river terrain smoke decodes chunk data and requires at least one water
  block near spawn.

## Out Of Scope

- Fluid physics.
- River graph drainage or erosion simulation.
- Aquifers, oceans, beaches, and underwater light behavior.
- Placement or breaking behavior specific to water.
