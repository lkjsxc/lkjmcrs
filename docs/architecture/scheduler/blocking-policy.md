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
- Bounded deterministic chunk generation in the first milestone.

## Async Work

Chunk persistence runs on separate blocking tasks and returns results through
explicit region-actor messages. Region actors may enqueue load or save work and
continue processing unrelated mailbox commands while storage runs. Mutation
replies wait for persistence; save failure rolls back the tentative block state
before the reply.

Future compression and online-mode auth follow the same handoff rule.

## Verification

Unit tests cover scheduler handoff rules. Later load tests must measure
per-region tick duration and mailbox depth.
