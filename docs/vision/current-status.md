# Current Status

## Goal

Give agents one short place to learn the current project state before choosing
the next work batch.

## Current Capability

- The server target constants are owned by
  [../architecture/protocol/minecraft-1-21-11.md](../architecture/protocol/minecraft-1-21-11.md).
- Offline-mode login, online-mode encrypted login, configuration, play entry,
  chunks, light, position, time, keepalive, and command declaration are
  implemented.
- The playable world is a deterministic flat overworld with sparse persisted
  block overrides stored in `redb`.
- Player profiles, online identity, and world overrides use the current
  `redb` storage foundation; earlier SQLite storage is not supported.
- Survival placement, breaking, simple drops, pickup, inventory projection, and
  reconnect persistence are compose-verified.
- Health, operator damage, death state, and respawn restoration are
  compose-verified.
- Offline chat, homes, warps, operator commands, gamemode changes, and kick are
  compose-verified.
- Chunk batches send embedded light through `level_chunk_with_light` without
  per-chunk `update_light` packets.
- Progressive chunk streaming is implemented: radius `2` bootstraps eagerly,
  configured radii through `8` stream farther chunks under chunk and byte
  budgets.
- Radius `8` automated load evidence is landed for initial `25` chunks and
  eventual `289` unique chunks.
- Play keepalive timeout is implemented and compose-verified.
- Public runtime exposure requires `online_mode=true` with session
  verification.

## Active Blockers

- Manual stock-client evidence still needs a fresh raw client log or explicit
  success artifact after the latest packet-shape fixes.
- Normal survival is incomplete: tools and crafting are absent, terrain is
  flat, and mobs/weather are not gameplay systems yet.
- Large distance targets above the current cap require stale-pending movement
  evidence and stronger scale counter evidence before the configured cap can
  increase.

## Next Implementation Target

Scale evidence is the active foundation for larger-distance work:

- keep the near bootstrap small,
- batch farther chunk loads under explicit budgets,
- prove stale queued chunks are replaced when movement happens before far
  streaming finishes,
- record active sessions, region mailbox depth, cache counters, pending queue
  length, and storage timings before raising caps.

## Rules

1. Update owner docs before behavior-changing code.
2. Keep custom `lkjmcsmp`-style gameplay out of the normal survival path.
3. Keep current-state summaries short; detailed command evidence belongs under
   operations verification.
