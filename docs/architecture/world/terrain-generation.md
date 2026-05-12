# Terrain Generation

## Goal

Provide a first-party terrain lane informed by external terrain systems without
adopting config packs, plugins, or external runtime contracts.

## Current Behavior

- Keep flat terrain selectable for scale probes.
- Use an internal generator boundary with `flat` and `natural` providers.
- Keep generation deterministic by world seed and chunk coordinate.
- Keep hot-path generation bounded to local chunk neighborhoods.
- Store player mutations as overrides above generated terrain.
- `natural` builds bedrock, stone, dirt, grass, static water, riverbeds, cave
  air, and surface survival features from deterministic generated fields.
- Static water uses a shared river and ocean level that blends into nearby
  banks and shelves instead of sitting far below ordinary land.
- Static water and rivers are retained generated terrain owned by
  [water-and-rivers.md](water-and-rivers.md).
- Generated caves are retained generated terrain owned by [caves.md](caves.md).
- Richer surface generation is owned by [generation/README.md](generation/README.md).
- Spruce-style surface decorators should provide roughly triple the compact
  baseline tree density while staying deterministic and spawn-safe.
- `flat` remains the controlled generator for scale and cache regression
  probes.
- Spawn selection is owned by [spawn-resolution.md](spawn-resolution.md).
- The staged target pipeline is owned by [terrain-pipeline.md](terrain-pipeline.md).

## Rules

1. Generated chunks must be cheap to discard and rebuild.
2. Generation must not block session packet I/O.
3. Terrain docs must define new block IDs before protocol tests depend on them.
4. Direct Bukkit, Paper, Folia, Terra, or worldgen-plugin support is not a
   current target.
5. Surface biomes, forests, coasts, rivers, and scenic spawn quality are in
   scope for the normal survival terrain target.
6. Formula marker changes are required when generated chunk cache semantics
   change.
7. Ores, mobs, weather, structures, fluid simulation, and Anvil files remain
   outside the current terrain target.
