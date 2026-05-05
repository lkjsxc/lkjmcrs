# SMP Travel

## Goal

Give early SMP players durable, command-driven travel without introducing
parties, requests, cooldowns, or temporary dimensions yet.

## Player Commands

All players may use:

- `/sethome [name]`: save the caller's current position as a personal home.
- `/home [name]`: teleport the caller to a saved personal home.
- `/homes`: list the caller's saved homes.
- `/warp <name>`: teleport the caller to a global warp.
- `/warps`: list global warps.

Operators may use:

- `/setwarp <name>`: save or replace a global warp at the caller's current
  position.

## Names

- The omitted home name is `home`.
- Names are trimmed, converted to ASCII lowercase, and stored normalized.
- Names may contain `a-z`, `0-9`, `_`, and `-`.
- Names must be 1 to 32 characters after normalization.
- Invalid names return a caller-only system chat error.

## Limits

- Each player may have up to 16 homes.
- Replacing an existing home does not count against the limit.
- Global warps have no product limit in this slice.

## Teleport Rules

1. Teleports use the same absolute position packet shape as `/spawn`.
2. Teleports update the in-memory profile position immediately.
3. Saved locations include world, `x`, `y`, `z`, `yaw`, and `pitch`.
4. The only valid world value today is `minecraft:overworld`.
5. Missing homes or warps return a caller-only system chat error.

## Deferred Work

- `/delhome` and `/delwarp`.
- Teleport requests between players.
- Random teleport.
- Cooldowns, warmups, movement stability checks, and safety scans.
- Per-world homes or dimensions beyond the overworld.
