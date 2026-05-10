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
- Surface constraints consumed by
  [surface-palettes.md](surface-palettes.md).
- Dry-land hints consumed by [spawn-resolution](../spawn-resolution.md).

## Rules

1. Ocean water is static generated terrain.
2. Coast transitions must be continuous across chunk borders.
3. Beaches are generated surface material choices, not separate structures.
4. Spawn resolution must avoid ocean, deep-water, and unsafe coast columns.
5. Ocean and river water must agree at shared sea-level boundaries.
6. New block states for sand, gravel, kelp, or coral require separate protocol
   and storage ownership before use.

## Verification

- Golden tests cover fixed-seed ocean, beach, and inland columns.
- Border property tests cover coast continuity.
- Live terrain smoke verifies decoded static water and dry spawn safety when
  ocean terrain is active.

## Out Of Scope

- Fluid simulation.
- Tides, waves, boats, swimming rules, fishing, and aquatic mobs.
- Coral reefs, shipwrecks, monuments, and other structures.
