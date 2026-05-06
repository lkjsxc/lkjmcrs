# Chunk Storage

## Current Model

- Use generated in-memory chunks.
- Represent block states with compact palette identifiers.
- Avoid per-block heap allocation.
- Keep serialization code separate from world ownership.
- Serialize section data using the protocol contract in
  [../protocol/chunk-packets.md](../protocol/chunk-packets.md).

## Persistent Override Slice

- Persist only sparse block overrides that differ from generated flat terrain.
- Store overrides in `world.sqlite3` under the configured data directory.
- The database schema version is `PRAGMA user_version = 1`.
- `chunk_overrides` rows are keyed by chunk coordinate and local block
  coordinate.
- Missing rows mean generated flat terrain with no override.
- Setting a block back to generated base deletes that override row.
- Accepted mutations save before prediction acknowledgement and fanout.
- Corrupt storage fails startup or mutation instead of being ignored.
- Legacy `chunks/*.json` files are ignored; backward compatibility is out of
  scope for this milestone.

## Future Persistence

- Vanilla Anvil import is a later compatibility feature.
- Server-owned storage may be designed before Anvil import if it improves
  scheduler locality.

## Rules

1. Chunk data APIs expose chunk coordinates explicitly.
2. Region-owned mutation is required for writable chunks.
3. Generated chunks must be cheap to discard and regenerate.
