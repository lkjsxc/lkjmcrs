# Current Results

## 2026-05-10 Surface Wood Terrain

Latest full recorded compose result: implementation commit `dcd1e26`, with
canonical research disposition, owned spruce log and leaves block states,
deterministic spruce-style surface decorators, generated-wood spawn scoring,
terrain-quality probing, and natural render-movement probe hardening.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- initial data cleanup: pass with `down -v`.
- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- `smoke`: pass, `login-play probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `movement-authority`: pass, `movement-authority probe ok`.
- `chunk-stream`: pass, `chunk-stream probe ok`.
- `scale-chunk-stream`: pass, `scale-chunk-stream probe ok`.
- `terrain-generation`: pass, `terrain-generation probe ok`.
- `terrain-quality`: pass, `terrain-quality probe ok`.
- `river-terrain`: pass, `terrain-rivers probe ok`.
- `terrain-caves`: pass, `terrain-caves probe ok`.
- `scale-load-metrics`: pass,
  `scale-load-metrics counters radius=8 initial=25 total=289
  followup_batches=17 max_followup_batch=16
  max_followup_payload_bytes=963840`, then
  `scale-load-metrics probe ok`.
- `scale-moving-pending`: pass, `scale-moving-pending probe ok`.
- `render-distance`: pass,
  `render-distance counters radius=32 initial=25 total=4225
  followup_batches=263 max_followup_batch=16
  max_followup_payload_bytes=1031544`, then `render-distance probe ok`.
- `render-moving-pending`: pass, `render-moving-pending probe ok`.
- `persist-place`: pass, `persist-place probe ok`.
- `persistence-server` restart: pass.
- `persist-check`: pass, `persist-check probe ok`.
- `storage-section-persistence`: pass,
  `storage-section-persistence probe ok`.
- `survival-item`: pass, `survival-item probe ok`.
- `inventory-sync`: pass, `inventory-sync probe ok`.
- `item-pickup`: pass, `item-pickup probe ok`.
- `survival-vitals`: pass, `survival-vitals probe ok`.
- `smp-commands`: pass, `smp-commands probe ok`.
- `online-auth`: pass, `online-auth probe ok`.
- final data cleanup: pass with `down -v`.

## Manual Boundary

No active stock-client disconnect boundary is known. Fresh manual stock-client
evidence is still useful because this compose run validates protocol behavior
through probes rather than a graphical client.

## History

Older result summaries live in [results/README.md](results/README.md).
