# Caves

## Goal

Define the first generated cave slice without adding gameplay systems or new
block identities.

## Current Target

- Generate deterministic underground cave pockets for `natural` terrain.
- Carve only generated `stone` or `dirt` into `air`.
- Use `world_seed` and absolute block coordinates for cave decisions.
- Keep `flat` terrain unchanged for controlled scale probes.
- Keep persisted overrides layered above generated cave terrain through
  `WorldStore`.

## Terrain Rules

- Caves are generated base terrain and are disposable.
- Caves must stay below the surface safety margin.
- Caves must not carve bedrock, grass surface blocks, static water columns, or
  terrain outside the world height.
- Heightmaps remain surface-driven because this slice must not open surface
  entrances.
- Spawn safety remains dry, solid, and headroom-safe.

## Verification

- Golden terrain coverage requires fixed-seed underground cave air.
- Border property coverage checks cave decisions across adjacent chunks.
- Live cave terrain smoke decodes chunk data and requires enclosed underground
  air below the surface.

## Out Of Scope

- New protocol block-state IDs.
- New section-storage block codes.
- Lava, aquifers, ores, cave biomes, structures, decorations, mobs, and light
  simulation.
- Cave-specific placement, breaking, item, swimming, or movement behavior.
