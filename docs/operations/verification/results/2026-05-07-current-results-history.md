# 2026-05-07 Current Results History

## Covered Commits

- `4b7ebbd`: docs canon refresh, hunger-loop vitals, operator `/vitals`, full
  status/play/mutation/observer/keepalive smoke, stricter verifier URL
  validation, protocol import-boundary checks, and probe-local position helpers.
- `32006f4`: process-local online login key reuse, partial-write-safe encrypted
  streams, localhost Compose publishing, and encrypted online-auth probe.

## Result Summary

Both covered batches passed the canonical Compose sequence owned by
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
- `smp-commands`,
- `online-auth`.

## Notes

- `online-auth` used the session fixture and encrypted login path.
- These entries are historical acceptance evidence, not the active boundary.
