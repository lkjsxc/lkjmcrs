# SMP Commands

## Goal

Keep the first multiplayer control surface explicit, small, and testable.

## Permission Model

- Offline player names are the permission identity in this slice.
- `LKJMCRS_OPS` is a comma-separated list of operator names.
- Name comparison is ASCII case-insensitive.
- Empty `LKJMCRS_OPS` means no operator-only commands are available.

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
- `/say` broadcasts one system chat message.
- `/gamemode` changes profile mode, sends abilities and game-event mode update,
  and confirms with system chat.
- `/kick` sends a play disconnect packet to the target session.

## Out of Scope

- Persistent permissions beyond environment config.
- Command suggestions beyond the declared command tree.
- Full Brigadier parser coverage.
- Chat signing and signed message verification.
