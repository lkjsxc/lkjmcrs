# Region Ownership

## Goal

Make each world mutation happen through the region that owns the affected chunk
or region section.

## Invariants

1. A loaded chunk has exactly one owning region.
2. A region may tick only while it owns all chunks it mutates.
3. Non-owner tasks submit messages instead of mutating directly.
4. Region-local state does not need shared locks for normal ticking.
5. Cross-region operations are split into explicit phases.

## Current Slice

- Use static region ownership for the flat spawn area.
- Prove actor/mailbox scheduling with tests.
- Defer dynamic split and merge until persistent chunk loading exists.

## Mutation Publication

- Sessions submit block mutations as actor messages.
- The region actor returns the authoritative final state to the caller.
- Accepted loaded-chunk mutations are published to session observers through the
  session registry.
- The region actor does not write client sockets directly.
