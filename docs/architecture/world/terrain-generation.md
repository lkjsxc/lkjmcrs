# Terrain Generation

## Goal

Replace the deterministic flat world only after chunk storage and progressive
streaming can absorb realistic generation cost.

## Direction

- Keep flat terrain as the current implementation.
- Introduce a generator boundary before adding natural terrain.
- Keep generation deterministic by world seed and chunk coordinate.
- Keep hot-path generation bounded to local chunk neighborhoods.
- Store player mutations as overrides above generated terrain.

## Pipeline Target

1. Sample deterministic fields such as height, temperature, humidity, roughness,
   and cave density.
2. Select a biome from data-driven distribution rules.
3. Build vertical block palettes for the chunk.
4. Apply local features such as ores, trees, and caves.
5. Run expensive enrichment only as background or pregeneration work.

## Rules

1. Generated chunks must be cheap to discard and rebuild.
2. Generation must not block session packet I/O.
3. Terrain docs must define new block IDs before protocol tests depend on them.
4. Direct Bukkit, Paper, Folia, or worldgen-plugin compatibility is not a
   current target.
