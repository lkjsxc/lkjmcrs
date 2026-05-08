# Current Results

## 2026-05-08 Terra-Inspired Terrain Slice

Implementation tested: working tree on `1170851`, with natural terrain
generation, flat spawn plateau preservation, generator-backed sparse override
loads, terrain probe coverage, and flat scale verification configs.

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
- `terrain-server` startup: pass.
- `terrain-generation`: pass, `terrain-generation probe ok`.
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

## Manual Boundary

No active stock-client disconnect boundary is known after the dropped item
`add_entity` tail fix. The latest user-reported successful join has no raw
client log attached, so fresh manual evidence is still needed.

## History

Older result summaries live in [results/README.md](results/README.md).
