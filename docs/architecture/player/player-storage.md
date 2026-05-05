# Player Storage

## Goal

Persist player profiles in a storage format that can later support economy,
quests, parties, and cross-player queries.

## Storage Root

- Runtime storage root comes from `LKJMCRS_DATA_DIR`.
- Player database path is `players.sqlite3` inside the storage root.
- Docker Compose stores it in the existing `/data` volume.

## Schema Contract

- SQLite `PRAGMA user_version` is `1`.
- `player_profiles` owns one row per UUID.
- `player_inventory_slots` owns zero or more rows per UUID.
- Inventory slot rows are replaced as part of profile save.
- Unsupported nonzero schema versions fail startup or profile access.

## I/O Rules

1. Player storage is not region-owned world state.
2. Profile reads happen after login name validation.
3. Profile saves happen when a play session disconnects.
4. Storage failures disconnect the affected login or play session.
5. SQLite access must not run from region actor tick paths.

## Out of Scope

- Online-mode identity verification.
- Player-data migrations beyond rejecting unsupported schema versions.
- Economy, quests, parties, achievements, and permissions.
