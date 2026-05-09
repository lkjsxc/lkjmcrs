# Persistent Overrides

## Goal

Keep early block mutations across restarts without replacing the deterministic
generated base or introducing Anvil compatibility.

## Storage Root

- Runtime storage root comes from JSON config field `data_dir`.
- The default local root is `data`.
- Docker Compose mounts a named volume at the configured storage root.
- World overrides live in `world.redb` inside the storage root.

## Database Contract

- Database path: `world.redb`.
- Table `meta` stores string keys with byte values.
- `meta` key `world_override_format` stores the active format marker.
- Table `chunk_overrides` stores key `overworld/{chunk_x}/{chunk_z}`.
- Each chunk value uses the binary override format owned by
  [section-storage.md](section-storage.md).
- Each override stores local `x`, absolute `y`, local `z`, and a block state
  code.
- Only states supported by the current terrain palette may be stored.
- Missing chunk keys are valid and load the configured generator base.
- Existing `world.sqlite3` and `chunks/*.json` files are ignored.

## Write Rules

1. Region actors remain the only writers for loaded chunk overrides.
2. Accepted mutations update region-owned memory before client acknowledgement.
3. Setting a block back to its generated base removes the override.
4. A chunk with no overrides has no `chunk_overrides` table value.
5. `redb` chunk writes are serialized inside `WorldStorage`.
6. A save failure logs a warning and retries the latest in-memory chunk state.
7. Memory is authoritative until the next successful save.

## Out of Scope

- Entity, time, and weather persistence.
- Anvil region files.
- Migration from earlier SQLite files or earlier JSON chunk override values.
