# SMP Commands

## Goal

Keep the first multiplayer control surface explicit, small, and testable.

## Permission Model

- Player UUIDs are the permission identity.
- `operator_uuids` is a JSON array of operator UUIDs.
- Empty `operator_uuids` means no operator-only commands are available.

## Runtime Model

- Each registered play session stores session ID, UUID, latest name, and op
  flag.
- Chat and command results are delivered through the play outbound channel.
- The play loop remains the only task that writes to its TCP stream.
- Commands that target another player look up the target in the session
  registry.

## Command Effects

- `/help` sends one system chat response to the caller.
- `/spawn` sends an absolute position packet to the caller and updates the
  caller profile position.
- `/sethome` saves the caller's current location under a normalized home name.
- `/home` teleports the caller to a saved home.
- `/homes` lists the caller's saved home names.
- `/setwarp` saves the caller's current location under a normalized warp name.
- `/warp` teleports the caller to a saved global warp.
- `/warps` lists normalized global warp names.
- `/say` broadcasts one system chat message.
- `/gamemode` changes profile mode, sends abilities and game-event mode update,
  and confirms with system chat.
- `/damage` reduces a connected target player's health and may trigger death.
- `/vitals` sets a connected target player's health, hunger, and saturation and
  may trigger or clear death state.
- `/kick` sends a play disconnect packet to the target session.

## Out of Scope

- Persistent permissions beyond JSON config.
- Command suggestions beyond the declared command tree.
- Full Brigadier parser coverage.
- Chat signing and signed message verification.
- Teleport cooldowns, warmups, request flows, or safety scans.
