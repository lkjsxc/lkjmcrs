# 2026-05-07 Foundation History

## Covered Commits

- `caa93a7`: survival vitals command and respawn support.
- `e2d439c`: world and player persistence moved to `redb` JSON values.
- `177e55f`: chunk batches stopped sending duplicate per-chunk light packets.
- `589a2bb`: play keepalive timeout enforcement.

## Result Summary

Each covered batch passed the canonical Compose sequence owned by
[../compose-pipeline.md](../compose-pipeline.md).

Passing probes across the covered history:

- static `verify`,
- `smoke`,
- `profile-reconnect`,
- `chunk-stream`,
- `persist-place`,
- `persist-check`,
- `survival-item`,
- `inventory-sync`,
- `item-pickup`,
- `survival-vitals`,
- `smp-commands`.

## Notes

- `redb` persistence replaced earlier SQLite-backed storage without migration.
- Streamed chunks send embedded light through `level_chunk_with_light`.
- Keepalive timeout closes through the normal connection error path.
- These entries are historical acceptance evidence, not the active boundary.
