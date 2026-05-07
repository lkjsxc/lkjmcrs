# Current Results

## 2026-05-07 Streamlined Chunk Batches

Implementation tested: working tree after `177e55f`, with chunk batches sending
embedded light through `level_chunk_with_light` and no per-chunk `update_light`.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- initial `down -v`: pass.
- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- `server` startup: pass.
- `smoke`: pass, `multiplayer-mutation probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `chunk-stream`: pass, `chunk-stream probe ok`.
- `persist-place`: pass, `persist-place probe ok`.
- server restart: pass.
- `persist-check`: pass, `persist-check probe ok`.
- `survival-server` startup: pass.
- `survival-item`: pass, `survival-item probe ok`.
- `inventory-sync`: pass, `inventory-sync probe ok`.
- `item-pickup`: pass, `item-pickup probe ok`.
- `survival-vitals-server` startup: pass.
- `survival-vitals`: pass, `survival-vitals probe ok`.
- `smp-server` startup: pass.
- `smp-commands`: pass, `smp-commands probe ok`.
- final `down -v`: pass.

## 2026-05-07 redb JSON Persistence

Implementation tested: working tree after `e2d439c`, with world and player
persistence moved from SQLite to `redb` JSON values.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- initial `down -v`: pass.
- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- `server` startup: pass.
- `smoke`: pass, `multiplayer-mutation probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `chunk-stream`: pass, `chunk-stream probe ok`.
- `persist-place`: pass, `persist-place probe ok`.
- server restart: pass.
- `persist-check`: pass, `persist-check probe ok`.
- `survival-server` startup: pass.
- `survival-item`: pass, `survival-item probe ok`.
- `inventory-sync`: pass, `inventory-sync probe ok`.
- `item-pickup`: pass, `item-pickup probe ok`.
- `survival-vitals-server` startup: pass.
- `survival-vitals`: pass, `survival-vitals probe ok`.
- `smp-server` startup: pass.
- `smp-commands`: pass, `smp-commands probe ok`.
- final `down -v`: pass.

## 2026-05-07 Survival Vitals

Implementation tested: working tree after `caa93a7`, survival vitals,
operator damage, death, respawn, and compose vitals probe.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- initial `down -v`: pass.
- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- `server` startup: pass.
- `smoke`: pass, `multiplayer-mutation probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `chunk-stream`: pass, `chunk-stream probe ok`.
- `persist-place`: pass, `persist-place probe ok`.
- server restart: pass.
- `persist-check`: pass, `persist-check probe ok`.
- `survival-server` startup: pass.
- `survival-item`: pass, `survival-item probe ok`.
- `inventory-sync`: pass, `inventory-sync probe ok`.
- `item-pickup`: pass, `item-pickup probe ok`.
- `survival-vitals-server` startup: pass.
- `survival-vitals`: pass, `survival-vitals probe ok`.
- `smp-server` startup: pass.
- `smp-commands`: pass, `smp-commands probe ok`.
- final `down -v`: pass.

## Manual Boundary

No active stock-client disconnect boundary is known after the dropped item
`add_entity` tail fix. The latest user-reported successful join has no raw
client log attached, so fresh manual evidence is still needed.

## History

Older result summaries live in [results/README.md](results/README.md).
