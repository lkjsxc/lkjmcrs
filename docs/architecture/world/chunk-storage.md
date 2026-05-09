# Chunk Storage

## Current Model

- Use generated in-memory chunks.
- Represent block states with compact palette identifiers.
- Avoid per-block heap allocation.
- Keep serialization code separate from world ownership.
- Serialize section data using the protocol contract in
  [../protocol/chunk-packets.md](../protocol/chunk-packets.md).

## Persistent Override Slice

Persistent override ownership lives in
[persistent-overrides.md](persistent-overrides.md).
The current redb value format is owned by
[section-storage.md](section-storage.md).

## Future Persistence

- Vanilla Anvil import is a later compatibility feature.
- Server-owned storage may be designed before Anvil import if it improves
  scheduler locality.

## Rules

1. Chunk data APIs expose chunk coordinates explicitly.
2. Region-owned mutation is required for writable chunks.
3. Generated chunks must be cheap to discard and regenerate.
4. Storage write ordering follows [persistent-overrides.md](persistent-overrides.md).
