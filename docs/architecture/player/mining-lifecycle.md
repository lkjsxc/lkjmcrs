# Mining Lifecycle

## Goal

Make survival block breaking a start, wait, stop lifecycle instead of an
immediate mutation.

## Player Actions

- `StartDestroyBlock` begins mining when the player is alive, in reach, and the
  target is loaded and breakable.
- `AbortDestroyBlock` clears active mining and reconciles the current block.
- `StopDestroyBlock` breaks only the same position after the required time has
  elapsed.
- Other player actions clear active mining and reconcile the current block.

## Break Timing

- Creative breaking has no delay.
- Survival dirt and grass block breaking requires `750ms`.
- Survival stone breaking requires `1500ms`.
- Air and bedrock cannot start mining.
- Tool-specific speed, enchantments, and durability are later survival-tools
  work.

## Mutation Rules

1. Start validates against the current region-owned block state.
2. Stop requests the region mutation only after the active mining timer is
   ready.
3. Dead, out-of-reach, unloaded, changed-position, or early stop attempts do
   not mutate chunks or inventory.
4. Accepted survival breaking follows the drop and pickup contracts in
   [../world/item-entities.md](../world/item-entities.md).
5. Every handled action sends prediction acknowledgement and an authoritative
   block update.
