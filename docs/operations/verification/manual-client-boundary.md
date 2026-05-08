# Manual Client Boundary

## Goal

Own active manual stock-client evidence in one place.

## Current Boundary

No active stock-client disconnect boundary is known after the implemented
packet-shape fixes and item-entity tail fix.

Fresh manual evidence is still required because the latest successful stock
client join report did not include a raw client log.

## Next Manual Evidence

Run a stock Minecraft Java Edition `1.21.11` client against the current server
commit and record:

- commit hash,
- online or offline mode,
- exact client result or disconnect text,
- whether terrain renders,
- relevant server connection log lines.

## Historical Sources

- [join-boundary.md](join-boundary.md) is historical/procedural.
- [online-vanilla-join.md](online-vanilla-join.md) is the online-mode manual
  checklist.
- [client-reports/README.md](client-reports/README.md) indexes captured
  packet-shape regressions.

## Rules

1. This file owns the active manual boundary.
2. Client reports become active only after evidence-policy review.
3. Do not record account secrets, access tokens, or raw authenticated session
   material.
