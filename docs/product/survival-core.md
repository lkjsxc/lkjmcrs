# Survival Core

## Goal

Start the Survival Sandbox with visible block mutation while preserving the
region-ownership architecture.

## First Slice

- Joined players remain in creative mode.
- The server accepts basic block placement and block breaking in loaded spawn
  chunks.
- Placement writes a fixed `minecraft:stone` block.
- Breaking writes `minecraft:air`.
- Bedrock cannot be replaced or broken.
- Mutations are visible to the initiating client through block update packets.
- Accepted mutations fan out to subscribed players in the changed chunk.
- Sparse block overrides persist across server restarts once the persistence
  slice is implemented.

## Survival Item Slice

- New profiles still default to creative unless configuration says otherwise.
- `default_game_mode: "survival"` creates new profiles in survival mode.
- `survival_starter_stone` grants new survival profiles that many
  `minecraft:stone` items in selected hotbar slot `0`.
- Creative placement keeps the fixed-stone behavior from the first slice.
- Survival placement requires the selected slot to contain `minecraft:stone`.
- Accepted survival placement consumes one selected stone item.
- Survival placement without an item acknowledges prediction and reconciles the
  target block without mutating the world.
- Survival breaking writes `minecraft:air` for mutable blocks and adds a simple
  drop to the player inventory.
- Simple drops are deterministic: stone drops stone, dirt drops dirt, and
  grass block drops dirt.
- Inventory mutations are saved with the player profile on disconnect.

## Material Loop Slice

- Survival placement uses the selected server-side hotbar item.
- Selected `minecraft:stone` places `minecraft:stone`.
- Selected `minecraft:dirt` places `minecraft:dirt`.
- Unsupported selected items and empty selected slots do not mutate the world.
- Rejected placement still acknowledges prediction and reconciles the target
  block.
- Accepted survival placement consumes exactly one selected item.
- Accepted survival breaking keeps the simple deterministic drops from the
  Survival Item Slice.
- Placement and breaking require the target block center to be within `6.0`
  blocks of the player's eye position.
- The eye position is `(player.x, player.y + 1.62, player.z)`.
- Out-of-reach interactions do not mutate chunks or inventory.

## Player Boundary

- Inventory contents and selected hotbar slot affect survival placement.
- The selected hotbar slot and canonical player inventory slots `0..35` are
  projected to the client during play bootstrap and after accepted survival
  inventory mutations.
- Survival mining speed, recipes, and item durability are not validated in this
  slice.
- Block interactions outside the loaded spawn-radius chunks are acknowledged and
  reconciled without loading new chunks.

## Rules

1. All block mutation requests go through the region scheduler.
2. The deterministic flat world remains the base for chunks without overrides.
3. Client prediction must be answered with an acknowledgement and a block update.
4. Persistence stores only sparse overrides above the deterministic flat base.
5. Survival inventory changes are committed only after an accepted region
   mutation result.
6. Rejected placement and invalid held-slot input preserve server inventory
   state and resend authoritative inventory selection when needed.
