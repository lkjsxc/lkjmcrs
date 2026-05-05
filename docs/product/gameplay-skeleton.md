# Gameplay Skeleton

## Goal

Give the first-party `1.21.11` wire probe enough state to enter a simple world
contract and remain connected while the server ticks.

## Vanilla Client Boundary

- Server-list status targets real vanilla client behavior.
- Full vanilla play rendering is not claimed until registry and chunk packets
  are complete enough for a stock client.
- The first milestone proves the server-side play lifecycle with the compose
  smoke probe.

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

- Full vanilla registry synchronization.
- Vanilla-complete chunk packet encoding.
- Block breaking and placement.
- Entity AI.
- Inventory and recipes.
- Chat signing.
- Persistence.
