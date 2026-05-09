# Spawn Resolution

## Goal

Own server-chosen spawn positions for new profiles, respawn, `/spawn`, and
teleport safety checks.

## Current Behavior

- `flat` worlds spawn at the deterministic flat surface.
- `natural` worlds must use the deterministic spawn scorer for the configured
  `world_seed`.
- Spawn coordinates are converted into player position, default-spawn packet,
  respawn packet, and `/spawn` teleport state through one server-owned path.

## Natural Spawn Rules

1. Candidate columns are scored near the configured seed's origin.
2. Solid floor, two-block headroom, modest local slope, and distance from the
   origin affect the score.
3. The resolver may make small deterministic safety adjustments to the selected
   column.
4. Safety adjustment must not stamp a visible flat plateau into generated
   natural terrain.
5. The same seed and generator version must resolve the same spawn.

## Teleport Safety

- `/spawn` and respawn use the resolved spawn until per-destination safety
  checks exist.
- Later homes, warps, and travel commands should share the same floor and
  headroom checks instead of inventing separate rules.
- Unsafe destination rejection belongs in command handling, not protocol packet
  encoding.

## Verification

- Unit tests cover deterministic seed stability, headroom, and slope bounds.
- `terrain-generation` proves the spawn packet and first chunk delivery still
  work through the live protocol.
- Movement and persistence probes own post-teleport position persistence.
