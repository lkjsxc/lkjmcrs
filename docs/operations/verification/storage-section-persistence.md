# Storage Section Persistence

## Goal

Verify binary sparse override persistence beyond a single happy-path placed
block.

## Required Coverage

- Save more than one override in one chunk.
- Save overrides in at least two vertical sections of the same chunk.
- Restart the server without deleting the data volume.
- Load the same chunk through the public play protocol.
- Verify every persisted block appears in `level_chunk_with_light`.
- Verify replacing a block with its generated base removes that override.

## Schema Assertions

- `world.redb` contains the `chunk_overrides` table.
- `meta` contains `world_override_format`.
- The persisted marker is `lkjmcrs.chunk_overrides.v1`.
- Chunk values use the binary record described in
  [../../architecture/world/section-storage.md](../../architecture/world/section-storage.md).

## Acceptance Rules

1. The probe must use an isolated data volume.
2. The probe must not read private process memory.
3. Invalid binary fixture tests may be unit tests instead of compose probes.
4. Old JSON chunk values remain unsupported.

## Deferred Command

The current compose pipeline covers single-block persistence through
[persistence-smoke.md](persistence-smoke.md). A broader section-persistence
command is not active yet.
