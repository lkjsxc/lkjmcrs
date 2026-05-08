# Current Results

## 2026-05-08 Scale Evidence Hardening

Implementation tested: `5f9a9b2`, with shared streaming budgets, exact
region chunk-load collection, process-shared flat payload caching, scale
counters, storage timing logs, and moving-pending stale queue evidence.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- initial `down -v`: pass.
- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- `server` startup: pass.
- `smoke`: pass, `login-play probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `chunk-stream`: pass, `chunk-stream probe ok`.
- `scale-server` startup: pass.
- `scale-chunk-stream`: pass, `scale-chunk-stream probe ok`.
- `scale-load-server` startup: pass.
- `scale-load-metrics`: pass,
  `scale-load-metrics counters radius=8 initial=25 total=289
  followup_batches=33 max_followup_batch=8
  max_followup_payload_bytes=481920`, then
  `scale-load-metrics probe ok`.
- `scale-moving-pending`: pass, `scale-moving-pending probe ok`.
- server-side scale counter log check: pass, latest line included
  `followup_batches=33`, `followup_chunks=264`,
  `pending_queue_len=0`, `active_sessions=1`,
  `region_mailbox_depth=0`, and `flat_cache_hits=289`.
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

## 2026-05-08 Scale Load Metrics

Implementation tested: `21683b3`, with batched pending chunk loads, scale
counters, radius `8` load verification, and refreshed docs canon.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- initial `down -v`: pass.
- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- `server` startup: pass.
- `smoke`: pass, `login-play probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `chunk-stream`: pass, `chunk-stream probe ok`.
- `scale-server` startup: pass.
- `scale-chunk-stream`: pass, `scale-chunk-stream probe ok`.
- `scale-load-server` startup: pass.
- `scale-load-metrics`: pass,
  `scale-load-metrics counters radius=8 initial=25 total=289
  followup_batches=33 max_followup_batch=8
  max_followup_payload_bytes=481920`, then
  `scale-load-metrics probe ok`.
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
