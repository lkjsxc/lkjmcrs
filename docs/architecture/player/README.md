# Player

Use this subtree for authentication, persistent player state, session
lifecycle, and play-session visibility contracts.

## Read This Section When

- You need login behavior.
- You need player profile persistence rules.
- You need online/offline mode policy.
- You need session state rules.
- You need chunk subscription or observer fanout behavior.

## Child Index

- [authentication.md](authentication.md): offline and future online auth.
- [inventory.md](inventory.md): selected hotbar and first item-loop rules.
- [player-locations.md](player-locations.md): persisted homes and global warps.
- [player-state.md](player-state.md): persistent profile, gamemode, inventory,
  and vitals.
- [vitals.md](vitals.md): health, hunger, saturation, damage, death, and
  respawn rules.
- [player-storage.md](player-storage.md): SQLite profile storage contract.
- [session-lifecycle.md](session-lifecycle.md): player session states.
- [play-loop.md](play-loop.md): movement, keepalive, and time behavior in play.
- [chunk-observers.md](chunk-observers.md): chunk subscriptions and block
  update fanout.
- [smp-commands.md](smp-commands.md): chat, commands, and operator rules.
