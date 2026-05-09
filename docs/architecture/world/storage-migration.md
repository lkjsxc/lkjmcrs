# Storage Migration

## Goal

Avoid accidental compatibility promises for early internal storage files.

## Current Policy

- `WorldStore` is the public behavior boundary.
- The current world override backend is `redb` with binary
  `chunk_overrides` values.
- Earlier `world.redb` JSON chunk values are unsupported and may be ignored.
- Earlier `world.sqlite3` and `chunks/*.json` files are unsupported and ignored.
- Acceptance runs use clean Compose volumes unless a probe explicitly verifies
  restart behavior within the same format.

## Migration Requirements

Any future compatibility migration must define:

1. the source format and exact detection rule,
2. the destination format,
3. failure behavior,
4. operator rollback or cleanup instructions,
5. verification evidence using a fixture file.

No migration exists until an owner doc and tests define those points.
