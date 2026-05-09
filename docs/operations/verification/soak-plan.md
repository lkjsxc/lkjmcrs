# Soak Plan

## Goal

Define long-running verification before raising public runtime confidence.

## Initial Lanes

- Repeated login, movement, disconnect, and reconnect cycles.
- Repeated chunk streaming around radius `8` and radius `32`.
- Persistence restart loops that place, restart, and re-check stored overrides.
- Survival item loops that break, pick up, disconnect, and reconnect.
- Online-mode authentication loops against the local session fixture.

## Required Evidence

- Command sequence and compose files.
- Runtime config files.
- Commit identifier.
- Duration and iteration count.
- Any disconnect, timeout, panic, warning spike, or failed probe.

## Rules

1. Soaks use isolated disposable data volumes.
2. Soaks preserve cargo and target caches when useful.
3. A soak failure blocks only when its lane is promoted to an acceptance gate.
4. Current merge acceptance remains the compose pipeline in
   [compose-pipeline.md](compose-pipeline.md).
