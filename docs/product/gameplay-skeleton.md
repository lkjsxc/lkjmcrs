# Gameplay Skeleton

## Goal

Give a stock offline-mode `1.21.11` client enough protocol state to pass
`login_finished`, complete configuration, enter play, and receive a minimal
flat spawn area.

## Vanilla Client Boundary

- Server-list status targets real vanilla client behavior.
- The first-party probe proves login, configuration, registry, chunk, light,
  position, and keepalive packet order.
- Stock-client rendering is accepted only after manual evidence is captured in
  the verification docs.

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
- Full variant registry contents beyond one valid entry per required registry.
- Terrain outside the deterministic spawn batch.
- Block breaking and placement.
- Entity AI.
- Inventory and recipes.
- Chat signing.
- Persistence.
