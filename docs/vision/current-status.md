# Current Status

## Goal

Give agents one short place to learn the current project state before choosing
the next work batch.

## Current Capability

- The server targets Minecraft Java Edition `1.21.11` and protocol `774`.
- Offline-mode login, configuration, play entry, chunks, light, position, time,
  keepalive, and command declaration are implemented.
- The playable world is a deterministic flat overworld with sparse persisted
  block overrides.
- Survival placement, breaking, simple drops, pickup, inventory projection, and
  reconnect persistence are compose-verified.
- Health, operator damage, death state, and respawn restoration are
  compose-verified.
- Offline chat, homes, warps, operator commands, gamemode changes, and kick are
  compose-verified.
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

Reduce chunk streaming waste before increasing any distance target:

- stop sending explicit `update_light` after every chunk batch entry,
- keep light data inside `level_chunk_with_light`,
- load only newly visible movement chunks through the region actor,
- document progressive budgets before raising the distance ceiling.

## Rules

1. Update owner docs before behavior-changing code.
2. Keep custom `lkjmcsmp`-style gameplay out of the normal survival path.
3. Keep current-state summaries short; detailed command evidence belongs under
   operations verification.
