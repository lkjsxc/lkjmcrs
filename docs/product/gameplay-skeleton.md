# Gameplay Skeleton

## Goal

Give a vanilla `1.21.11` client enough state to enter a simple world and remain
connected while the server ticks.

## World

- One dimension: `minecraft:overworld`.
- Flat deterministic terrain.
- Spawn defaults to `0, 80, 0`.
- Initial game mode is creative for easier smoke testing.
- Time starts at `0` and advances by server ticks.

## Player Behavior

- Offline-mode name is accepted after protocol version validation.
- Server creates a deterministic offline UUID from the player name.
- Movement packets are accepted and update session-local position.
- Keepalive is sent periodically and timed out if ignored.

## Out of Scope

- Block breaking and placement.
- Entity AI.
- Inventory and recipes.
- Chat signing.
- Persistence.
