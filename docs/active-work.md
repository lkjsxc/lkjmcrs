# Active Work Index

## Purpose

Give automation agents one short map before reading detailed docs.

## Current State

- Current status: [vision/current-status.md](vision/current-status.md).
- Latest automated result: [operations/verification/current-results.md](operations/verification/current-results.md).
- Manual client boundary:
  [operations/verification/manual-client-boundary.md](operations/verification/manual-client-boundary.md).
- Active implementation target:
  [architecture/world/section-storage.md](architecture/world/section-storage.md).
- Canonical compose pipeline:
  [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md).

## Rules

1. Treat `docs/` as canon.
2. Treat `tmp/deep-research-report*.md` as stale input unless copied into
   canon docs with verification.
3. Refresh canon docs before changing behavior.
