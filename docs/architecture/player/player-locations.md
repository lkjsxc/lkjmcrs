# Player Locations

## Goal

Persist named teleport locations in a form that is simple to query and easy for
future travel systems to reuse.

## Location Shape

Every stored location has:

- Normalized `name`.
- `world`, currently always `minecraft:overworld`.
- `x`, `y`, and `z` as double-precision coordinates.
- `yaw` and `pitch` as single-precision view angles.

## Home Rules

- Homes are keyed by player UUID and normalized name.
- `/sethome` inserts or replaces one row.
- `/home` reads one row and teleports the caller if it exists.
- `/homes` lists the caller's home names sorted by name.
- A player may own at most 16 home names.

## Warp Rules

- Warps are keyed by normalized name.
- `/setwarp` inserts or replaces one row.
- `/warp` reads one row and teleports the caller if it exists.
- `/warps` lists global warp names sorted by name.
- The creating operator UUID is recorded for audit-oriented future work.

## Runtime Rules

1. Location storage belongs to player storage, not region storage.
2. Command dispatch performs location reads and writes from the play loop.
3. Storage failures disconnect the affected play session.
4. Teleport writes update the caller's in-memory profile before the next save.
5. No safety scan runs in this slice because teleport targets are trusted
   command outputs.

## Out of Scope

- Deleting homes or warps.
- Cross-world validation.
- Player-to-player teleport requests.
- Cooldowns, warmups, or combat restrictions.
