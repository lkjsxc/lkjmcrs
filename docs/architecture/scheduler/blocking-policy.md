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

Future persistence, compression, and online-mode auth run on separate async
tasks and return results through explicit messages.

## Verification

Unit tests cover scheduler handoff rules. Later load tests must measure
per-region tick duration and mailbox depth.
