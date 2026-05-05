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
3. Active evidence moves only after docs, implementation, and compose smoke
   agree.
4. Historical reports stay in the tree as regression stories, not as the current
   join boundary.
5. New protocol behavior requires manual client evidence or an existing docs
   contract.

## Current Baseline

As of commit `26f8d20`, compose `verify` and live `smoke` are the automated
baseline. The existing `tmp/` reports are historical unless a newer stock-client
run reproduces the same failure against a later commit.
