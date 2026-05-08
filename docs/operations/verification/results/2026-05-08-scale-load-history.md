# 2026-05-08 Scale Load History

Implementation tested: `21683b3`, with batched pending chunk loads, scale
counters, radius `8` load verification, and refreshed docs canon.

Result summary:

- `verify`: pass.
- `smoke`, `profile-reconnect`, and `chunk-stream`: pass.
- `scale-chunk-stream`: pass.
- `scale-load-metrics`: pass with radius `8`, initial `25`, total `289`,
  follow-up batches `33`, and max follow-up batch `8`.
- Persistence, survival item, inventory sync, item pickup, survival vitals,
  SMP commands, and online auth probes: pass.
- final `down -v`: pass.
