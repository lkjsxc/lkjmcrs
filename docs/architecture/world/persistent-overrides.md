# Persistent Overrides

## Goal

Keep early block mutations across restarts without replacing the deterministic
flat-world base or introducing Anvil compatibility.

## Storage Root

- Runtime storage root comes from `LKJMCRS_DATA_DIR`.
- The default local root is `data`.
- Docker Compose mounts a named volume at `/data` and sets
  `LKJMCRS_DATA_DIR=/data`.
- Chunk files live under `chunks/` inside the storage root.

## File Contract

- File path: `chunks/c.<chunk_x>.<chunk_z>.json`.
- Schema version: `1`.
- Each file stores its chunk coordinate and sparse block overrides.
- Each override stores block `x`, `y`, `z`, and block state name.
- Only states supported by the current flat-world palette may be stored.
- Missing files are valid and mean no overrides for that chunk.

## Write Rules

1. Region actors remain the only writers for loaded chunk overrides.
2. A mutation that changes an accepted loaded chunk is saved before fanout.
3. Setting a block back to its generated base removes the override.
4. A chunk with no overrides deletes its chunk file.
5. Failed persistence prevents acknowledgement for that mutation.

## Out of Scope

- Entity, time, weather, and inventory-backed gameplay persistence.
- Anvil region files.
- Storage migrations beyond rejecting unsupported schema versions.
