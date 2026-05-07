# Player Storage

## Goal

Persist player profiles in a storage format that can later support economy,
quests, parties, and cross-player queries.

## Storage Root

- Runtime storage root comes from JSON config field `data_dir`.
- Player database path is `players.redb` inside the storage root.
- Docker Compose stores it in the configured named volume.

## Database Contract

- Database path: `players.redb`.
- Table `profiles` maps UUID strings to JSON profile values.
- Table `homes` maps `{uuid}/{name}` to JSON location values.
- Table `warps` maps warp names to JSON warp values.
- Profile values contain name, game mode, position, selected hotbar slot,
  vitals, and inventory slots.
- Saving a profile replaces the whole profile value, including inventory.
- Existing `players.sqlite3` files are intentionally ignored.

## Location Values

- Home keys are scoped by UUID and name.
- Warp keys are global names.
- Coordinates use JSON numbers.
- `world` is stored as text and must be `minecraft:overworld` today.
- Warp values include `created_by_uuid` for audit behavior.

## I/O Rules

1. Player storage is not region-owned world state.
2. Profile reads happen after login name validation.
3. Profile saves happen when a play session disconnects.
4. Home and warp writes happen synchronously during command dispatch.
5. Storage failures disconnect the affected login or play session.
6. Blocking database work must not run from region actor tick paths.
7. PlayerStore serializes profile, home, and warp writes inside the process.
8. Reads and writes use short blocking tasks.
9. Invalid stored game modes, locations, hotbar slots, or inventory slots fail
   the affected operation.
10. Home names and warp names are returned in sorted order.

## Out of Scope

- Online-mode identity verification.
- Migration from earlier SQLite files.
- Economy, quests, parties, achievements, and stored permissions.
