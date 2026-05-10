# Surface Palettes

## Goal

Define biome-aware block choices for the top and shallow subsurface of generated
natural terrain.

## Owns

- Top block selection for dry land, wet land, beaches, and shallow water.
- Shallow subsurface layering.
- Palette keys exposed to decorator stages.
- Fallback block choices when a biome has no specialized palette.

## Inputs

- Base terrain height and material from the natural generator.
- Biome labels from [biome-pipeline.md](biome-pipeline.md).
- River and wetness hints from [hydrology.md](hydrology.md).
- Ocean and coast hints from [oceans-and-coasts.md](oceans-and-coasts.md).
- Absolute column coordinates.

## Outputs

- Final generated surface and shallow subsurface block states.
- Decorator placement substrate hints for
  [surface-decorators.md](surface-decorators.md).
- Generated-origin hints for
  [generated-content-markers.md](generated-content-markers.md).

## Rules

1. Surface palettes may only emit block states owned by protocol and storage
   docs.
2. Dry surface columns must preserve spawn-safe solid ground and headroom.
3. Submerged columns must not place grass or other dry-only top blocks.
4. Palette decisions must be deterministic from the stage inputs.
5. Persisted player edits override generated palette output.

## Verification

- Golden tests include fixed-seed top-block and shallow-layer samples.
- Border property tests cover palette continuity at chunk edges.
- Storage tests cover any new block-state code before palette use.

## Out Of Scope

- Trees, plants, ore, structures, and loose items.
- Runtime grass spread or farmland changes.
- Client resource-pack appearance.
