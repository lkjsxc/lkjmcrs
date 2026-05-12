# Terrain Pipeline

## Goal

Define how generated terrain, persisted overrides, and protocol chunk encoding
fit together without making Anvil or plugin-pack compatibility promises.

## Inputs

- `terrain_generator`: `natural` or `flat`.
- `world_seed`: signed integer seed for deterministic generated terrain.
- Chunk coordinate: signed `x,z`.
- Persisted sparse overrides loaded through `WorldStore`.

## Target Pipeline

1. Select `flat` or `natural` at runtime construction.
2. For `natural`, sample macro fields for land, coast, uplift, erosion,
   temperature, moisture, and river potential.
3. Shape terrain from macro fields so landforms, coasts, river corridors, and
   mountain belts are decided before local block painting.
4. Blend river terraces and ocean shelves to the shared water level before
   surface palette decisions.
5. Assign biome and surface palette from elevation, slope, moisture,
   temperature, coast influence, and water adjacency.
6. Write solid terrain, static water, riverbeds, beaches, and surface blocks.
7. Apply deterministic decorators such as trees only through owner-documented
   density, slope, and headroom stages.
8. Apply retained cave carving below surface and away from static water.
9. Resolve a deterministic scenic spawn from generated terrain.
10. Load sparse persisted sections through `WorldStore`.
11. Apply persisted sections above the generated base.
12. Encode the final chunk through the protocol chunk contract.

## River And Water Slice

- River terrain is owned by [water-and-rivers.md](water-and-rivers.md).
- Rivers are static generated terrain blocks, not simulated fluids.
- Water uses the block-state constant documented by the protocol chunk owner.
- River and ocean water share the documented top level.
- Spawn scoring must choose dry columns even when rivers generate near origin.
- Ocean, coast, and beach rules are owned by
  [generation/oceans-and-coasts.md](generation/oceans-and-coasts.md).

## Cave Slice

- Generated caves are owned by [caves.md](caves.md).
- Caves carve only generated solid terrain into `Air`.
- Cave output must be deterministic from `world_seed` and absolute block
  coordinates.
- Caves remain below the surface and must not modify static water columns.

## Spawn Resolution

- New profiles, `/spawn`, and respawn use the owner rules in
  [spawn-resolution.md](spawn-resolution.md).
- The natural generator must not create a visible flat plateau around spawn.
- Spawn scoring must favor dry footing, modest slope, headroom, nearby wood,
  nearby water access, and enough open ground for orientation.

## Boundaries

- Generated terrain is disposable and may be rebuilt from config inputs.
- Persisted block changes are section records in the active `redb` schema.
- `WorldStore` is the persistence boundary for override reads and writes.
- Protocol chunk encoding must not know whether a block came from generation
  or persistence.

## Out of Scope

- Anvil import or export.
- Terra config-pack compatibility.
- Mobs and weather-driven terrain changes.
