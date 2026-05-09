# Current Results

## 2026-05-09 Survival Timing And Radius 32 Stream Acceptance

Latest recorded compose result: implementation commit `383cd26`, with the
documentation refresh, `WorldStore` redb facade, survival mining timing,
pickup-delay AABB checks, spawn-to-natural terrain blending, isolated
streaming verification services, and probe readers that tolerate live packet
interleaving.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- data cleanup: pass. Test data volumes were removed where a clean world was
  required; cargo and target caches were preserved between probe runs.
- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- `smoke`: pass, `login-play probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `chunk-stream`: pass against isolated `chunk-stream-server`,
  `chunk-stream probe ok`.
- `scale-chunk-stream`: pass, `scale-chunk-stream probe ok`.
- `terrain-generation`: pass, `terrain-generation probe ok`.
- `scale-load-metrics`: pass,
  `scale-load-metrics counters radius=8 initial=25 total=289
  followup_batches=17 max_followup_batch=16
  max_followup_payload_bytes=963840`, then
  `scale-load-metrics probe ok`.
- `scale-moving-pending`: pass against isolated `scale-moving-server`,
  `scale-moving-pending probe ok`.
- `render-distance`: pass,
  `render-distance counters radius=32 initial=25 total=4225
  followup_batches=263 max_followup_batch=16
  max_followup_payload_bytes=982351`, then `render-distance probe ok`.
- `render-moving-pending`: pass, `render-moving-pending probe ok`.
- `persist-place`: pass, `persist-place probe ok`.
- server restart: pass.
- `persist-check`: pass, `persist-check probe ok`.
- `survival-item`: pass, `survival-item probe ok`.
- `inventory-sync`: pass, `inventory-sync probe ok`.
- `item-pickup`: pass, `item-pickup probe ok`.
- `survival-vitals`: pass, `survival-vitals probe ok`.
- `smp-commands`: pass, `smp-commands probe ok`.
- `online-auth`: pass, `online-auth probe ok`.

## Manual Boundary

No active stock-client disconnect boundary is known. Fresh manual stock-client
evidence is still useful because this compose run validates protocol behavior
through probes rather than a graphical client.

## History

Older result summaries live in [results/README.md](results/README.md).
