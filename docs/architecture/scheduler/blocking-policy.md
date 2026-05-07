# Blocking Policy

## Tick Workers

Tick workers must not perform:

- filesystem writes,
- network calls,
- database calls,
- long compression jobs,
- blocking waits on other regions.

## Allowed Work

- Region-local block and entity mutation.
- Bounded packet preparation for nearby players.
- Bounded deterministic chunk generation in the current slice.

## Async Work

Chunk persistence runs on separate blocking tasks and returns results through
explicit region-actor messages. Region actors may enqueue load or save work and
continue processing unrelated mailbox commands while storage runs.

Block mutation replies are based on authoritative in-memory state, not on the
storage write finishing. Save failure is logged and retried by the storage job;
the in-memory state remains authoritative for connected sessions.

Compression and online-mode session verification follow the same handoff rule:
expensive work stays outside region actors and tick-owned mutation paths.

## Verification

Unit tests cover scheduler handoff rules. Later load tests must measure
per-region tick duration and mailbox depth.
