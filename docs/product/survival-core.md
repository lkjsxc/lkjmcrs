# Survival Core

## Goal

Start the second milestone with visible block mutation while preserving the
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

## Player Boundary

- Inventory contents are persisted as a shell but do not affect gameplay yet.
- Held item selection may be accepted but has no gameplay effect.
- Reach distance, survival mining speed, drops, recipes, and item durability are
  not validated in this slice.
- Block interactions outside the loaded spawn-radius chunks are acknowledged and
  reconciled without loading new chunks.

## Rules

1. All block mutation requests go through the region scheduler.
2. The deterministic flat world remains the base for chunks without overrides.
3. Client prediction must be answered with an acknowledgement and a block update.
4. Persistence stores only sparse overrides above the deterministic flat base.
