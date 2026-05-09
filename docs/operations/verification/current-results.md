# Current Results

## 2026-05-10 World Foundation Focused Verification

Latest focused result: implementation commit `757994d`, with staged natural
terrain modules, section-keyed `redb` world overrides, centralized spawn
settings, first movement-authority rejection, and `RUST_LOG`-driven tracing.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- `verify`: pass with compact output:
  `verify fmt ... ok`, `verify clippy ... ok`, `verify test ... ok`,
  `verify docs-topology ... ok`, `verify line-limits ... ok`, `verify pass`.
- initial data cleanup: pass with `down -v`.
- `terrain-generation`: pass, `terrain-generation probe ok`.
- `persist-place`: pass against `persistence-server`,
  `persist-place probe ok`.
- `persistence-server` restart: pass.
- `persist-check`: pass against `persistence-server`,
  `persist-check probe ok`.
- `smoke`: pass, `login-play probe ok`.
- final data cleanup: pass with `down -v`.

The full compose pipeline was not rerun for this focused result. The latest
full recorded result remains below.

## 2026-05-10 Binary Storage And Isolated Stateful Probes

Latest full recorded compose result: implementation commit `cb269a9`, with binary
`redb` world override values, `WorldStore` codec validation, and isolated data
volumes for persistence, survival-item, inventory-sync, and item-pickup probes.

Command owner:

- [compose-pipeline.md](compose-pipeline.md)

Result:

- initial data cleanup: pass with `down -v`.
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
- `persist-place`: pass against isolated `persistence-server`,
  `persist-place probe ok`.
- `persistence-server` restart: pass.
- `persist-check`: pass against isolated `persistence-server`,
  `persist-check probe ok`.
- `survival-item`: pass against isolated `survival-item-server`,
  `survival-item probe ok`.
- `inventory-sync`: pass against isolated `inventory-sync-server`,
  `inventory-sync probe ok`.
- `item-pickup`: pass against isolated `item-pickup-server`,
  `item-pickup probe ok`.
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
