# Task Handoff

## Same-Region Work

- Execute mutation on the owning region worker.
- Keep work small enough to preserve tick cadence.
- Return results through typed responses or session messages.

## Cross-Region Work

1. Capture source state on the source owner.
2. Submit a destination task to the destination owner.
3. Complete the operation only after destination ownership accepts it.
4. Report failure explicitly to the session.

## Examples

- Teleport source removal and destination placement.
- Future piston or contraption effects crossing a region boundary.
- Future portal destination search and placement.

## Rule

No code path may hold a region-local mutable borrow while waiting on another
region response.
