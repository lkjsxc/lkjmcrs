# Inventory

## Goal

Define the first server-authoritative item loop without claiming full vanilla
inventory synchronization.

## Slot Model

- Inventory slots are server-internal numeric slots.
- The selected hotbar slot is an integer from `0` through `8`.
- New profiles start with selected hotbar slot `0`.
- This slice does not send full clientbound inventory contents.
- Slot rows persist item ID, count, and optional opaque data.

## Starter Items

- Creative profiles start with no persisted item requirements.
- Survival profiles may start with stone through
  `LKJMCRS_SURVIVAL_STARTER_STONE`.
- Starter stone is written as item ID `minecraft:stone`.
- Starter count must be between `0` and `64`.
- Starter stone is placed in selected hotbar slot `0`.

## Mutation Rules

1. Creative placement ignores inventory and writes fixed stone.
2. Survival placement consumes one selected supported block item.
3. Supported survival placement items are `minecraft:stone` and
   `minecraft:dirt`.
4. `minecraft:stone` places stone, and `minecraft:dirt` places dirt.
5. Survival placement with no selected supported item does not mutate the world.
6. Survival breaking adds simple drops after an accepted block mutation.
7. Empty slots are removed before profile save.
8. Item stacks cannot exceed `64` in this slice.

## Out of Scope

- Full vanilla inventory windows.
- Clientbound slot synchronization.
- Item NBT semantics beyond stored opaque text.
- Recipes, durability, tools, and mining speed.
