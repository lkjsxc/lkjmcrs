# Player State

## Goal

Make a player a persistent game object instead of a connection-local packet
holder.

## Profile Fields

Each player profile stores:

- deterministic offline UUID,
- latest accepted player name,
- game mode,
- position `x`, `y`, `z`,
- yaw and pitch,
- inventory shell,
- selected hotbar slot,
- vitals shell.

## Defaults

New offline profiles use:

- game mode: `creative`,
- position: `0.5, 80.0, 0.5`,
- yaw: `0.0`,
- pitch: `0.0`,
- selected hotbar slot: `0`,
- inventory: empty slots unless survival starter config grants stone,
- health: `20.0`,
- hunger: `20`,
- saturation: `5.0`.

## Game Modes

- `survival` maps to vanilla game mode `0`.
- `creative` maps to vanilla game mode `1`.
- New players default to `creative`.
- `LKJMCRS_DEFAULT_GAME_MODE=survival` creates missing profiles in survival.
- Stored game mode controls play login and player abilities.

## Runtime Contract

1. Login validates protocol and name before loading a profile.
2. Missing profiles are created with the documented defaults.
3. Play bootstrap uses stored position, yaw, pitch, and game mode.
4. Movement updates the connection-local play state.
5. Disconnect saves the latest play state back to the profile.
6. The selected hotbar slot and inventory are persisted with the profile.
7. Creative mode ignores inventory for block placement and breaking.
8. Survival placement and simple drops use the selected slot and inventory.
9. Vitals are persisted but do not affect gameplay yet.
