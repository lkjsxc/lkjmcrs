# Terrain Generation

## Goal

Provide a first-party terrain lane inspired by Terra concepts without adopting
Terra config packs, plugins, or compatibility.

## Direction

- Keep flat terrain selectable for scale probes.
- Use an internal generator boundary with `flat` and `natural` providers.
- Keep generation deterministic by world seed and chunk coordinate.
- Keep hot-path generation bounded to local chunk neighborhoods.
- Store player mutations as overrides above generated terrain.
- Keep chunks within Chebyshev radius `1` of spawn equivalent to the flat
  spawn surface.

## Pipeline Target

1. Choose `terrain_generator` as `natural` or `flat`.
2. Seed generation from `world_seed`.
3. Return flat spawn plateau chunks near `0,0`.
4. Sample deterministic smoothed value noise for outer-column height.
5. Build bedrock, stone, dirt, grass, and air columns.
6. Apply sparse stored overrides above the generated base.

## Rules

1. Generated chunks must be cheap to discard and rebuild.
2. Generation must not block session packet I/O.
3. Terrain docs must define new block IDs before protocol tests depend on them.
4. Direct Bukkit, Paper, Folia, Terra, or worldgen-plugin compatibility is not
   a current target.
5. Caves, biomes, ores, trees, mobs, weather, structures, and Anvil files are
   out of scope for this slice.
