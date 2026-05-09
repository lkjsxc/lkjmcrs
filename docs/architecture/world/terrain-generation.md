# Terrain Generation

## Goal

Provide a first-party terrain lane inspired by Terra concepts without adopting
Terra config packs, plugins, or compatibility.

## Current Behavior

- Keep flat terrain selectable for scale probes.
- Use an internal generator boundary with `flat` and `natural` providers.
- Keep generation deterministic by world seed and chunk coordinate.
- Keep hot-path generation bounded to local chunk neighborhoods.
- Store player mutations as overrides above generated terrain.
- `natural` currently builds bedrock, stone, dirt, grass, and air columns from
  deterministic smoothed value-noise surface heights.
- Static water and rivers are the next natural-terrain slice and are owned by
  [water-and-rivers.md](water-and-rivers.md).
- `flat` remains the controlled generator for scale and cache regression
  probes.
- Spawn selection is owned by [spawn-resolution.md](spawn-resolution.md).
- The staged target pipeline is owned by [terrain-pipeline.md](terrain-pipeline.md).

## Rules

1. Generated chunks must be cheap to discard and rebuild.
2. Generation must not block session packet I/O.
3. Terrain docs must define new block IDs before protocol tests depend on them.
4. Direct Bukkit, Paper, Folia, Terra, or worldgen-plugin compatibility is not
   a current target.
5. Caves, biomes, ores, trees, mobs, weather, structures, fluid simulation, and
   Anvil files are out of scope for this slice.
