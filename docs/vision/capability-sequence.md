# Capability Sequence

## Playable Server

- Minimal first-party `1.21.11` wire protocol.
- Offline-mode login and play-state entry.
- Deterministic flat world.
- Region scheduler primitives and tests.
- Docker Compose verify and smoke gates.

## Survival Sandbox

- Region-owned block mutation.
- Creative placement, breaking, and multiplayer observer smoke coverage.
- `redb` JSON storage for sparse chunk overrides and player data.
- Basic block interaction and player movement validation.
- Persistent player profiles, gamemode, inventory shell, and vitals shell.
- Inventory and item stack model.
- Offline-mode SMP chat and command control surface.
- Client-visible dropped item entities and pickup.
- More complete registry and data-pack handling.

## Storage And Identity Foundation

- Storage abstraction centered on `redb` tables and JSON values.
- No compatibility with earlier SQLite files.
- Online-mode login, encryption, and session verification outside tick paths.
- UUID-owned permission model backed by online identity when enabled.
- Progressive chunk streaming budgets for configured radii through `32`.

## Normal Survival

- Health, hunger, damage, death, and respawn effects.
- Minimal crafting, recipes, tools, durability, and mining speed.
- Passive and hostile mob smoke behavior.
- Day-night gameplay effects.

## Scale Architecture

- Radius `32` load evidence for initial `25` and eventual `4225` chunks.
- Stale-pending movement evidence before raising distance caps.
- Region splitting and merging.
- Region-local task queues.
- Broader async chunk generation and storage pipeline.
- Contraption-focused tick benchmarks.
- Operational metrics for per-region tick pressure.

## Extension Surface

- Internal module hooks before public plugin API.
- Capability-scoped event contracts.
- Async-safe command and task APIs.
- No Bukkit compatibility unless explicitly re-approved.

## Original Gameplay

- Add original systems inspired by `lkjmcsmp`.
- Keep custom gameplay modular and removable.
- Preserve vanilla compatibility as the baseline behavior.
