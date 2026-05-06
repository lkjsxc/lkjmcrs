# Module Boundary

## Goal

Keep wire encoding independent from domain ownership.

## Contract

- `protocol` owns packet IDs, codecs, wire payloads, and protocol-local value
  types.
- `protocol` must not import `world`, `player`, `scheduler`, or `session`.
- Domain modules convert their state into protocol-local DTOs before encoding.
- Protocol-local DTOs contain only values needed on the wire.

## Allowed Dependencies

- Standard library.
- Shared crate dependencies already approved by repository policy.
- Other `protocol` submodules.

## Rules

1. `world` and `player` may call `protocol`; `protocol` must not call them.
2. Packet tests may decode protocol-local payloads only.
3. New protocol helpers must accept protocol DTOs or primitives.
4. Domain-to-wire mapping lives in `session` or another coordinating module.
