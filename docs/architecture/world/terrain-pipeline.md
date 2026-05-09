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
2. Resolve a deterministic world spawn from `world_seed`.
3. For `natural`, sample staged fields for continentalness, ridge/valley shape,
   erosion-like smoothing, temperature, and humidity.
4. Build the surface and river columns from staged fields with deterministic
   chunk-neighbor continuity.
5. Apply the cave stage after surface and river columns.
6. Apply tree, ore, and decorator stages only after their owner docs and
   verification exist.
7. Load sparse persisted sections through `WorldStore`.
8. Apply persisted sections above the generated base.
9. Encode the final chunk through the protocol chunk contract.

## River And Water Slice

- River terrain is owned by [water-and-rivers.md](water-and-rivers.md).
- Rivers are static generated terrain blocks, not simulated fluids.
- Water uses the block-state constant documented by the protocol chunk owner.
- Spawn scoring must choose dry columns even when rivers generate near origin.

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

## Boundaries

- Generated terrain is disposable and may be rebuilt from config inputs.
- Persisted block changes are section records in the active `redb` schema.
- `WorldStore` is the persistence boundary for override reads and writes.
- Protocol chunk encoding must not know whether a block came from generation
  or persistence.

## Out of Scope

- Anvil import or export.
- Terra config-pack compatibility.
- Biomes beyond minimal registry-safe values.
- Mobs and weather-driven terrain changes.
