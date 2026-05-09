# Chunk Border Property

## Goal

Define the property test needed before richer terrain or section storage can
make chunk borders visibly inconsistent.

## Property

For any generated neighboring chunks with the same generator and seed:

- shared border columns must agree on absolute coordinates,
- height sampling must not depend on which chunk requested the column,
- block state at a border world position must be stable across regeneration,
- persisted overrides must apply only to their owning chunk-local coordinate.

## Coverage Targets

- East-west and north-south neighbor pairs.
- Positive and negative chunk coordinates.
- Chunks adjacent to the spawn safety core.
- At least one natural-terrain outer pair.
- At least one persisted override near a border.

## Acceptance Rules

1. The test may use in-process generation APIs.
2. The test must not require a live Minecraft client.
3. Failing examples should print seed, chunk pair, and world coordinate.
4. Compose terrain probes remain responsible for protocol-level chunk delivery.

## Deferred Command

No dedicated compose command is active yet. This property belongs in static
verification once implemented.
