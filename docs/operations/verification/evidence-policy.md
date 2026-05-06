# Evidence Policy

## Goal

Keep manual client reports useful without letting old disconnect files override
newer verified implementation.

## Evidence Classes

- Active evidence: the newest manual stock-client result captured after the
  latest relevant implementation fix.
- Historical evidence: older reports that already have documented fixes.
- Modded evidence: Fabric or client-side mod reports accepted only when they
  expose a vanilla protocol shape issue.
- Probe evidence: first-party compose smoke checks that prove the live wire
  path implemented by this repository.

## Rules

1. `tmp/disconnect-*.txt` files are raw input until a client report links them.
2. A linked report must state whether it is active or historical.
3. Report order follows the filename/report timestamp, not filesystem mtime.
4. Filesystem mtime is discovery-only when scanning `tmp/`.
5. Active evidence moves only after docs, implementation, and compose smoke
   agree.
6. Historical reports stay in the tree as regression stories, not as the current
   join boundary.
7. New protocol behavior requires manual client evidence or an existing docs
   contract.

## Current Baseline

The latest automated baseline is recorded in
[current-results.md](current-results.md). Existing `tmp/` reports are
historical unless a newer stock-client run reproduces the same failure against
a later implementation commit.
