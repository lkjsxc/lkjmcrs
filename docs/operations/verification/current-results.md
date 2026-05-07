# Current Results

## 2026-05-07 Auth Hardening

Implementation tested: `32006f4`, with process-local online login key reuse,
partial-write-safe encrypted streams, localhost Compose publishing, and an
`online-auth` probe that reaches encrypted play bootstrap.

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
- `online-server` startup with session fixture: pass.
- `online-auth`: pass, `online-auth probe ok`.
- final `down -v`: pass.

## Manual Boundary

No active stock-client disconnect boundary is known after the dropped item
`add_entity` tail fix. The latest user-reported successful join has no raw
client log attached, so fresh manual evidence is still needed.

## History

Older result summaries live in [results/README.md](results/README.md).
