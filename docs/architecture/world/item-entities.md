# Item Entities

## Goal

Make simple survival drops visible in the world before they enter player
inventory.

## Entity Model

- Region actors own dropped item entity state.
- Entity IDs are server-local integers and must not collide with player entity
  ID `1`.
- A dropped item stores entity ID, UUID, chunk, position, item ID, count, and
  alive state.
- Supported item IDs are `minecraft:stone` and `minecraft:dirt`.
- Count is `1` for this slice.

## Spawn Rules

1. Accepted survival breaking of stone spawns `minecraft:stone`.
2. Accepted survival breaking of dirt or grass block spawns `minecraft:dirt`.
3. Creative breaking does not spawn dropped items.
4. Immutable, unloaded, rejected, or out-of-reach interactions do not spawn
   dropped items.
5. Spawn position is the broken block center: `x + 0.5`, `y + 0.5`, `z + 0.5`.
6. Dropped items are in-memory only and do not persist across restart.

## Visibility

- Sessions receive item entities for chunks in their current visible window.
- Bootstrap and chunk-stream entry send existing dropped items in newly visible
  chunks.
- Pickup sends collect and entity destroy packets.
- Explicit destroy on chunk-stream exit is deferred until entity tracking needs
  it.
- Observer fanout uses chunk subscriptions; sessions never mutate item entity
  state directly.

## Pickup

- Pickup radius is `1.5` blocks from player position to item position.
- A pickup succeeds only when the player inventory has stack or empty-slot
  capacity in synced slots `0..35`.
- A successful pickup removes the entity, sends collect and destroy packets,
  and then sends matching player-inventory deltas.
- A failed pickup leaves the entity alive.

## Out of Scope

- Gravity, velocity, merging, despawn timers, and item physics.
- Item entity persistence.
- Item NBT semantics beyond the first slot encoder.
- Mob drops and thrown items.
