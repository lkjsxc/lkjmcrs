# 2026-05-06 Survival Sandbox

## Covered Work

- Compact static verification output.
- Implicit runtime config loading.
- Held-item-only placement.
- Player SQLite contention hardening.
- Deterministic compose probes.

## Result Summary

- Static `verify` passed.
- `smoke`, `profile-reconnect`, `chunk-stream`, `persist-place`,
  `persist-check`, `survival-item`, `inventory-sync`, `item-pickup`, and
  `smp-commands` passed.
- Rust tests reported `118` passing tests at that time.
- Survival item smoke covered empty-hand rejection, grass break to dirt pickup,
  out-of-reach rejection, dirt placement, dirt break, reconnect persistence, and
  selected-slot reconciliation.
- SMP command smoke covered chat, permission denial, homes, warps, gamemode,
  persistence, and kick.

## Manual Join Note

A successful stock-client join was user-reported on `2026-05-05`, but no raw
client log was attached. Treat it as weak historical success evidence until a
fresh report is captured.
