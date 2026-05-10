# Biome Pipeline

## Goal

Define how generated terrain fields resolve to biome identities and biome hints
without coupling gameplay systems to private generator state.

## Owns

- Biome classification from climate, height, hydrology, and coast hints.
- Biome stability across chunk borders.
- Biome hints used by surface and decorator stages.
- Mapping from internal biome names to protocol registry IDs once those IDs are
  owned by protocol docs.

## Inputs

- Temperature and humidity from [macro-fields.md](macro-fields.md).
- Wetness and river hints from [hydrology.md](hydrology.md).
- Ocean and coast hints from [oceans-and-coasts.md](oceans-and-coasts.md).
- Absolute column coordinates.
- `world_seed`.

## Outputs

- Internal biome label per column or biome cell.
- Surface palette key consumed by
  [surface-palettes.md](surface-palettes.md).
- Decoration profile key consumed by
  [surface-decorators.md](surface-decorators.md).
- Protocol biome ID when dynamic registry support owns the mapping.

## Rules

1. Biome decisions are generated terrain metadata and are disposable.
2. Biome selection must not read persisted block overrides.
3. Biome borders must be deterministic and chunk-neighbor safe.
4. Protocol biome IDs remain owned by
   [../../protocol/dynamic-registries.md](../../protocol/dynamic-registries.md).
5. Until more registry entries are owned, generated chunks may still encode the
   minimal registry-safe biome value.

## Verification

- Golden tests include fixed-seed biome labels for representative columns.
- Border property tests cover biome continuity across adjacent chunks.
- Protocol tests verify biome registry IDs only after mappings are documented.

## Out Of Scope

- Mob spawn tables.
- Weather rules.
- Music, particles, sky color, and client visual effects.
- Structure selection.
