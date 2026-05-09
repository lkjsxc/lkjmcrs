# Storage Direction

## Goal

Keep persistence explicit while the world remains generated-first and
region-owned.

## Direction

- Runtime opens player storage and world storage before binding the listener.
- World storage loads a deterministic generated base, then applies sparse
  persisted overrides.
- `WorldStore` is the canonical world persistence boundary. Runtime and
  scheduler code must not depend on the concrete `redb` table layout.
- Current world override values are binary records in `redb`; JSON chunk
  values are not part of the supported storage contract.
- Region actors own loaded world mutation and request storage load or save jobs.
- Blocking storage work runs in short blocking tasks, then returns results to
  the owning actor or session path.
- Player storage remains separate from region-owned world state.
- Player profile writes replace the whole profile value on disconnect.
- Home and warp writes remain command-owned player storage operations.

## Rules

1. Do not store generated terrain columns when sparse overrides are enough.
2. Do not make player profile state part of region chunk persistence.
3. Memory is authoritative for accepted world mutations until a save succeeds.
4. Storage failure handling belongs to the owner path that requested the work.
5. Anvil import, entity persistence, and compatibility migration are later
   work.
