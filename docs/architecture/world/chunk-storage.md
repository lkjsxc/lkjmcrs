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
- Store overrides in server-owned JSON files under the configured data
  directory.
- One chunk file owns one chunk coordinate and schema version.
- Missing chunk files mean generated flat terrain with no overrides.
- Empty override sets delete the chunk file.
- Accepted mutations save before prediction acknowledgement and fanout.
- Corrupt storage fails startup or mutation instead of being ignored.

## Future Persistence

- Vanilla Anvil import is a later compatibility feature.
- Server-owned storage may be designed before Anvil import if it improves
  scheduler locality.

## Rules

1. Chunk data APIs expose chunk coordinates explicitly.
2. Region-owned mutation is required for writable chunks.
3. Generated chunks must be cheap to discard and regenerate.
