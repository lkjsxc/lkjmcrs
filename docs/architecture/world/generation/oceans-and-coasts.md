# Oceans And Coasts

## Goal

Define ocean basins, coast transitions, and beach-like terrain as generated
natural terrain.

## Owns

- Sea level for ocean terrain.
- Ocean basin classification.
- Coast width and slope transitions.
- Beach and shallow-water surface constraints.
- Ocean floor height hints.

## Inputs

- Macro landmass fields from [macro-fields.md](macro-fields.md).
- Drainage and wetness hints from [hydrology.md](hydrology.md).
- Absolute column coordinates.
- `world_seed`.

## Outputs

- Ocean, coast, beach, and inland classification hints.
- Static water-fill bounds for ocean columns.
- Shelf and low-bank height hints that keep water connected to nearby land.
- Surface constraints consumed by
  [surface-palettes.md](surface-palettes.md).
- Dry-land hints consumed by [spawn-resolution](../spawn-resolution.md).

## Rules

1. Ocean water is static generated terrain.
2. Sea level is `72` for natural terrain.
3. River and ocean water use the same top level at shared boundaries.
4. Coast transitions must be continuous across chunk borders.
5. Near-coast ocean floors stay shallow enough to read as shelves before deeper
   basins descend.
6. Dry coast columns blend through low banks before inland terrain rises.
7. Beaches are generated surface material choices, not separate structures.
8. Spawn resolution must avoid ocean, deep-water, and unsafe coast columns.
9. New block states for sand, gravel, kelp, or coral require separate protocol
   and storage ownership before use.

## Verification

- Golden tests cover fixed-seed ocean, beach, and inland columns.
- Golden tests include sea-level water and a nearby dry bank sample.
- Border property tests cover coast continuity.
- Live terrain smoke verifies decoded static water and dry spawn safety when
  ocean terrain is active.

## Out Of Scope

- Fluid simulation.
- Tides, waves, boats, swimming rules, fishing, and aquatic mobs.
- Coral reefs, shipwrecks, monuments, and other structures.
