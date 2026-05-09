# Survival Core

## Goal

Start the Survival Sandbox with visible block mutation while preserving the
region-ownership architecture.

## Block Mutation

- New profiles default to survival unless config says otherwise.
- The server accepts basic block placement and block breaking in loaded chunks.
- Breaking writes `minecraft:air`.
- Bedrock cannot be replaced or broken.
- Mutations are visible to the initiating client through block update packets.
- Accepted mutations fan out to subscribed players in the changed chunk.
- Sparse block overrides persist across server restarts.

## Placement

- Placement uses the selected server-side hotbar item.
- The selected item must be in the main hand.
- Selected `minecraft:stone` places `minecraft:stone`.
- Selected `minecraft:dirt` places `minecraft:dirt`.
- Empty hands do not mutate the world.
- Unsupported selected items and empty selected slots do not mutate the world.
- Rejected placement still acknowledges prediction and reconciles the target
  block.
- Accepted survival placement consumes exactly one selected item.
- Accepted creative placement consumes no item but still requires a supported
  selected item.
- Placement and breaking require the target block center to be within `6.0`
  blocks of the player's eye position.
- The eye position is `(player.x, player.y + 1.62, player.z)`.
- Out-of-reach interactions do not mutate chunks or inventory.

## Drops

- Accepted survival breaking spawns a visible dropped item entity.
- Supported dropped items are `minecraft:stone` and `minecraft:dirt`.
- Simple drops are deterministic: stone drops stone, dirt drops dirt, and
  grass block drops dirt.
- Pickup adds the item to the player inventory only when a synced slot can
  accept it.
- Pickup sends inventory deltas through the existing player-inventory
  projection.
- Dropped items are in-memory only.

## Player Boundary

- Inventory contents and selected hotbar slot affect survival placement.
- The selected hotbar slot and canonical player inventory slots `0..35` are
  projected to the client during play bootstrap and after accepted survival
  inventory mutations.
- Tool-specific mining speed, recipes, and item durability are not validated in
  this slice.
- Block interactions outside loaded chunks are acknowledged and reconciled
  without mutating chunks or inventory.

## Rules

1. All block mutation requests go through the region scheduler.
2. The deterministic generated world remains the base for chunks without
   overrides.
3. Client prediction must be answered with an acknowledgement and a block update.
4. Persistence stores only sparse overrides above the deterministic generated
   base.
5. Survival placement inventory changes are committed only after an accepted
   region mutation result.
6. Rejected placement and invalid held-slot input preserve server inventory
   state and resend authoritative inventory selection when needed.
7. Survival breaking creates a region-owned dropped item; pickup owns the later
   inventory mutation.
8. No mode fabricates a fixed fallback block for placement.
