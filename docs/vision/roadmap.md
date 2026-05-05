# Roadmap

## Milestone 1: Playable Skeleton

- Minimal first-party `1.21.11` wire protocol.
- Offline-mode login and play-state entry.
- Deterministic flat world.
- Region scheduler primitives and tests.
- Docker Compose verify and smoke gates.

## Milestone 2: Survival Core

- Persistent chunk storage.
- Basic block interaction and player movement validation.
- Entity registry and simple entity ticking.
- Inventory and item stack model.
- More complete registry and data-pack handling.

## Milestone 3: Scale Architecture

- Region splitting and merging.
- Region-local task queues.
- Async chunk generation and storage pipeline.
- Contraption-focused tick benchmarks.
- Operational metrics for per-region tick pressure.

## Milestone 4: Extension Surface

- Internal module hooks before public plugin API.
- Capability-scoped event contracts.
- Async-safe command and task APIs.
- No Bukkit compatibility unless explicitly re-approved.

## Milestone 5: Original Gameplay

- Add original systems inspired by `lkjmcsmp`.
- Keep custom gameplay modular and removable.
- Preserve vanilla compatibility as the baseline behavior.
