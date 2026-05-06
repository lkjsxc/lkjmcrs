# Player Storage

## Goal

Persist player profiles in a storage format that can later support economy,
quests, parties, and cross-player queries.

## Storage Root

- Runtime storage root comes from JSON config field `data_dir`.
- Player database path is `players.sqlite3` inside the storage root.
- Docker Compose stores it in the configured named volume.

## Schema Contract

- SQLite `PRAGMA user_version` is `3`.
- `player_profiles` owns one row per UUID.
- `player_profiles.selected_hotbar_slot` stores the selected hotbar slot.
- `player_inventory_slots` owns zero or more rows per UUID.
- Inventory slot rows are replaced as part of profile save.
- `player_homes` owns zero or more normalized home locations per UUID.
- `warps` owns normalized global warp locations.
- Unsupported nonzero schema versions fail startup or profile access.
- Existing version `1` and `2` databases are intentionally unsupported.

## Location Tables

- `player_homes` primary key is `(uuid, name)`.
- `warps` primary key is `name`.
- Coordinates use SQLite `REAL` values.
- `world` is stored as text and must be `minecraft:overworld` today.
- Warp rows include `created_by_uuid` for future audit behavior.

## I/O Rules

1. Player storage is not region-owned world state.
2. Profile reads happen after login name validation.
3. Profile saves happen when a play session disconnects.
4. Home and warp writes happen synchronously during command dispatch.
5. Storage failures disconnect the affected login or play session.
6. SQLite access must not run from region actor tick paths.

## Out of Scope

- Online-mode identity verification.
- Player-data migrations beyond rejecting unsupported schema versions.
- Economy, quests, parties, achievements, and stored permissions.
