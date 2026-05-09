# Storage Schema

## Goal

Document the target server-owned world storage schema for section-oriented
`redb` persistence.

## Database Files

- Runtime storage root comes from JSON config field `data_dir`.
- World data lives in `world.redb` under that root.
- Player data lives in `players.redb` and is owned by
  [../player/player-storage.md](../player/player-storage.md).

## World Tables

- `meta`: string keys with byte values.
- `chunk_meta`: one record per generated or mutated chunk.
- `chunk_sections`: one record per dirty vertical section.
- `chunk_entities`: persisted non-player entity records after entity storage is
  documented.

## Meta Keys

- `world_storage_schema`: stores a documented schema marker.
- Existing literal marker `lkjmcrs.chunk_overrides.v1` is only an old persisted
  string and must not name new Rust APIs or docs.

## Section Keys

- Shape: `overworld/{chunk_x}/{chunk_z}/{section_y}`.
- `chunk_x` and `chunk_z` are signed decimal chunk coordinates.
- `section_y` is the vertical section index for the world height.
- Only `minecraft:overworld` is supported in this slice.

## Values

- `chunk_meta` stores terrain hash, dirty-section bitmap, last save tick,
  feature mask, and lighting state.
- `chunk_sections` stores compact block states plus optional biome and light
  payloads.
- A missing section means the generated base has no persisted changes there.
- A chunk with no dirty sections must not keep empty section records.
- Old JSON chunk override values are unsupported.

## Rules

1. `WorldStore` remains the public storage boundary.
2. Region actors request world loads and saves through storage jobs.
3. Generated terrain is not persisted as whole chunks.
4. Storage schema changes need an owner doc and verification fixture before
   implementation.
5. Player profiles, homes, and warps stay outside world section tables.
