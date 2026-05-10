# Generation

Use this subtree for staged natural-terrain generation ownership.

## Read This Section When

- You need to add or change generated terrain fields.
- You need to know which stage owns a generated block choice.
- You need deterministic boundaries between terrain, water, biomes, palettes,
  decorators, and generated markers.

## Owner Docs

- [macro-fields.md](macro-fields.md): low-frequency terrain fields shared by
  later generation stages.
- [hydrology.md](hydrology.md): rivers, drainage hints, and static inland water
  decisions.
- [oceans-and-coasts.md](oceans-and-coasts.md): sea level, ocean basins,
  beaches, and coast transitions.
- [biome-pipeline.md](biome-pipeline.md): biome selection from terrain and
  climate fields.
- [surface-palettes.md](surface-palettes.md): biome-aware topsoil and shallow
  subsurface block choices.
- [surface-decorators.md](surface-decorators.md): deterministic surface
  decoration placement after base terrain exists.
- [generated-content-markers.md](generated-content-markers.md): internal
  markers that describe generated-origin content without changing persistence
  ownership.

## Existing Boundaries

- The broader generated terrain pipeline is owned by
  [../terrain-pipeline.md](../terrain-pipeline.md).
- Current generator behavior is owned by
  [../terrain-generation.md](../terrain-generation.md).
- Current static rivers are owned by
  [../water-and-rivers.md](../water-and-rivers.md).
- Generated caves are owned by [../caves.md](../caves.md).
- Spawn safety is owned by [../spawn-resolution.md](../spawn-resolution.md).
- Protocol biome and block-state IDs are owned by
  [../../protocol/chunk-packets.md](../../protocol/chunk-packets.md) and
  [../../protocol/dynamic-registries.md](../../protocol/dynamic-registries.md).

## Shared Rules

1. All stages are deterministic from `world_seed` and absolute coordinates.
2. Hot-path chunk generation must remain bounded to local chunk neighborhoods.
3. Generated output is disposable and must be rebuilt without persisted state.
4. Persisted player edits are sparse overrides layered after generation.
5. Protocol encoding must not know which stage produced a block.
6. New block states require protocol and section-storage ownership before use.
7. New behavior needs golden or property coverage before becoming a merge gate.

## Out Of Scope

- Anvil import or export.
- Terra config-pack compatibility.
- Runtime fluid simulation.
- Mob spawning, weather systems, structures, and gameplay loot.
