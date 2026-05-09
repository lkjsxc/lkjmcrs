# Active Work Index

## Purpose

Give automation agents one short map before reading detailed docs.

## Current State

- Current status: [vision/current-status.md](vision/current-status.md).
- Latest automated result: [operations/verification/current-results.md](operations/verification/current-results.md).
- Manual client boundary:
  [operations/verification/manual-client-boundary.md](operations/verification/manual-client-boundary.md).
- Active implementation target:
  [product/normal-survival.md](product/normal-survival.md).
- Active world target:
  [architecture/world/caves.md](architecture/world/caves.md).
- Active verification targets:
  [operations/verification/worldgen-golden.md](operations/verification/worldgen-golden.md),
  [operations/verification/chunk-border-property.md](operations/verification/chunk-border-property.md),
  [operations/verification/storage-section-persistence.md](operations/verification/storage-section-persistence.md),
  [operations/verification/movement-authority-smoke.md](operations/verification/movement-authority-smoke.md),
  [operations/verification/terrain-rivers-smoke.md](operations/verification/terrain-rivers-smoke.md),
  and [operations/verification/terrain-caves-smoke.md](operations/verification/terrain-caves-smoke.md).
- Canonical compose pipeline:
  [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md).

## Rules

1. Treat `docs/` as canon.
2. Treat `tmp/deep-research-report*.md` as stale input unless copied into
   canon docs with verification.
3. Refresh canon docs before changing behavior.
