# Chunk Storage

## First Milestone

- Use generated in-memory chunks.
- Represent block states with compact palette identifiers.
- Avoid per-block heap allocation.
- Keep serialization code separate from world ownership.

## Future Persistence

- Persistent storage is introduced after block mutation exists.
- Vanilla Anvil import is a later compatibility feature.
- Server-owned storage may be designed before Anvil import if it improves
  scheduler locality.

## Rules

1. Chunk data APIs expose chunk coordinates explicitly.
2. Region-owned mutation is required for writable chunks.
3. Generated chunks must be cheap to discard and regenerate.
