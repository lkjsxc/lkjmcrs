# Inventory

## Goal

Define the first server-authoritative item loop and the client-visible player
inventory projection.

## Slot Model

- Canonical synced player inventory slots are numeric slots `0` through `35`.
- Hotbar slots are `0` through `8`.
- The selected hotbar slot is an integer from `0` through `8`.
- New profiles start with selected hotbar slot `0`.
- Slot rows persist item ID, count, and optional opaque data.

## Starter Items

- Creative profiles start with no persisted item requirements.
- Survival profiles may start with stone through
  `survival_starter_stone`.
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
6. Survival breaking spawns simple drops after an accepted block mutation.
7. Empty slots are removed before profile save.
8. Item stacks cannot exceed `64` in this slice.
9. Play bootstrap sends `held_item_slot` and `set_player_inventory` for all
   slots `0..35`.
10. Accepted survival placement and breaking send `set_player_inventory`
    deltas for changed slots when inventory changes.
11. Invalid held-slot input preserves server state and resends the
    authoritative selected hotbar slot.
12. Item pickup adds to an existing compatible stack before using the first
    empty synced slot.

## Wire Item IDs

- Empty slot: `itemCount = 0`.
- `minecraft:stone`: item ID `1`.
- `minecraft:dirt`: item ID `28`.
- Non-empty slots write item count, item ID, zero added components, and zero
  removed components.

## Out of Scope

- Full vanilla inventory windows and container state.
- Cursor items.
- Armor and offhand slots.
- Dropped item entity ownership and pickup rules; see
  [../world/item-entities.md](../world/item-entities.md).
- Item NBT semantics beyond stored opaque text.
- Recipes, durability, tools, and mining speed.
