# Persistent Overrides

## Goal

Keep early block mutations across restarts without replacing the deterministic
flat-world base or introducing Anvil compatibility.

## Storage Root

- Runtime storage root comes from JSON config field `data_dir`.
- The default local root is `data`.
- Docker Compose mounts a named volume at the configured storage root.
- World overrides live in `world.sqlite3` inside the storage root.

## Database Contract

- Database path: `world.sqlite3`.
- Schema version: `PRAGMA user_version = 1`.
- SQLite runs with WAL mode and a `5s` busy timeout.
- Table: `chunk_overrides(chunk_x, chunk_z, local_x, y, local_z, state,
  PRIMARY KEY (...))`.
- Each row stores one sparse block override.
- Only states supported by the current flat-world palette may be stored.
- Missing rows are valid and mean no override for that block.
- Existing `chunks/*.json` files are ignored.

## Write Rules

1. Region actors remain the only writers for loaded chunk overrides.
2. Accepted mutations update region-owned memory before client acknowledgement.
3. Setting a block back to its generated base removes the override.
4. A chunk with no overrides has no rows in `chunk_overrides`.
5. SQLite chunk writes are serialized inside `WorldStorage`.
6. A save failure logs a warning and retries the latest in-memory chunk state.
7. Memory is authoritative until the next successful save.

## Out of Scope

- Entity, time, and weather persistence.
- Anvil region files.
- Storage migrations beyond rejecting unsupported schema versions.
