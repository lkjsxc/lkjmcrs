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
- The product config uses deterministic natural terrain with a protected spawn
  safety core and sparse persisted block overrides stored in `redb`.
- Player profiles and online identity use the current `redb` storage
  foundation; earlier SQLite storage is not supported.
- World overrides are owned by `WorldStore`; chunk override values use the
  current binary `redb` format.
- Survival placement, breaking, simple drops, pickup, inventory projection, and
  reconnect persistence are compose-verified.
- Health, operator damage, death state, and respawn restoration are
  compose-verified.
- Offline chat, homes, warps, operator commands, gamemode changes, and kick are
  compose-verified.
- Chunk batches send embedded light through `level_chunk_with_light` without
  per-chunk `update_light` packets.
- Progressive chunk streaming is implemented: radius `2` bootstraps eagerly,
  configured radii through `32` stream farther chunks under chunk and byte
  budgets.
- Radius `8` automated flat load, moving-pending, scale-counter, and storage
  timing evidence is landed.
- Radius `32` natural-terrain implementation and probes are compose-verified
  in the latest recorded full result.
- Stateful persistence, survival-item, inventory-sync, and item-pickup probes
  use isolated compose data volumes.
- Play keepalive timeout is implemented and compose-verified.
- Public runtime exposure requires `online_mode=true` with session
  verification.

## Active Blockers

- Manual stock-client evidence still needs a fresh raw client log or explicit
  success artifact after the latest packet-shape fixes.
- Normal survival is incomplete: tools, crafting, mobs, weather, caves,
  structures, ores, and decorations are not gameplay systems yet.

## Next Implementation Target

World foundation and verification hardening are the active target:

- replace the plateau-first natural generator with staged deterministic terrain,
- add a spawn resolver instead of hardcoded player spawn coordinates,
- move world persistence toward section-oriented `redb` records,
- add deterministic worldgen golden coverage,
- add generated chunk-border property coverage,
- broaden storage persistence coverage across multiple overrides and vertical
  sections,
- add dedicated movement-authority smoke coverage before deeper SMP hardening.

## Rules

1. Update owner docs before behavior-changing code.
2. Keep custom `lkjmcsmp`-style gameplay out of the normal survival path.
3. Keep current-state summaries short; detailed command evidence belongs under
   operations verification.
