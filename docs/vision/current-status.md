# Current Status

## Goal

Give agents one short place to learn the current project state before choosing
the next work batch.

## Current Capability

- The server target constants are owned by
  [../architecture/protocol/minecraft-1-21-11.md](../architecture/protocol/minecraft-1-21-11.md).
- Offline-mode login, configuration, play entry, chunks, light, position, time,
  keepalive, and command declaration are implemented.
- The playable world is a deterministic flat overworld with sparse persisted
  block overrides stored in `redb`.
- Survival placement, breaking, simple drops, pickup, inventory projection, and
  reconnect persistence are compose-verified.
- Health, operator damage, death state, and respawn restoration are
  compose-verified.
- Offline chat, homes, warps, operator commands, gamemode changes, and kick are
  compose-verified.
- Chunk batches send embedded light through `level_chunk_with_light` without
  per-chunk `update_light` packets.
- Runtime deployment is private-only while identity is name-based offline mode.

## Active Blockers

- Public internet exposure is blocked until online identity proof exists or an
  external private-access boundary is documented.
- Manual stock-client evidence still needs a fresh raw client log or explicit
  success artifact after the latest packet-shape fixes.
- Normal survival is incomplete: hunger drain and regeneration are absent,
  tools and crafting are absent, terrain is flat, and mobs/weather are not
  gameplay systems yet.

## Next Implementation Target

Close stale play sessions deterministically before deeper survival or larger
distance work:

- document a keepalive timeout owner contract,
- disconnect clients that do not answer the latest keepalive in time,
- keep mismatched keepalive responses observable without clearing timeout state.

## Rules

1. Update owner docs before behavior-changing code.
2. Keep custom `lkjmcsmp`-style gameplay out of the normal survival path.
3. Keep current-state summaries short; detailed command evidence belongs under
   operations verification.
