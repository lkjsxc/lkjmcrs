# Persistent Overrides

## Goal

Keep early block mutations across restarts without replacing the deterministic
flat-world base or introducing Anvil compatibility.

## Storage Root

- Runtime storage root comes from `LKJMCRS_DATA_DIR`.
- The default local root is `data`.
- Docker Compose mounts a named volume at `/data` and sets
  `LKJMCRS_DATA_DIR=/data`.
- World overrides live in `world.sqlite3` inside the storage root.

## Database Contract

- Database path: `world.sqlite3`.
- Schema version: `PRAGMA user_version = 1`.
- SQLite runs with WAL mode and a busy timeout.
- Table: `chunk_overrides(chunk_x, chunk_z, local_x, y, local_z, state,
  PRIMARY KEY (...))`.
- Each row stores one sparse block override.
- Only states supported by the current flat-world palette may be stored.
- Missing rows are valid and mean no override for that block.
- Existing `chunks/*.json` files are ignored.

## Write Rules

1. Region actors remain the only writers for loaded chunk overrides.
2. A mutation that changes an accepted loaded chunk is saved before fanout.
3. Setting a block back to its generated base removes the override.
4. A chunk with no overrides has no rows in `chunk_overrides`.
5. Failed persistence rolls back the tentative mutation and returns a
   reconciliation result.

## Out of Scope

- Entity, time, weather, and inventory-backed gameplay persistence.
- Anvil region files.
- Storage migrations beyond rejecting unsupported schema versions.
