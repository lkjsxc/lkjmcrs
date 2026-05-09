# Observability

## Goal

Make runtime behavior inspectable without adding unstable metrics contracts or
high-cardinality labels.

## Current Signals

- Structured logs are emitted through `tracing`.
- Connection close logs include peer, phase, and error level.
- Storage jobs log operation name, chunk count, elapsed milliseconds, and
  success state.
- Chunk streaming emits counters for initial chunks, total chunks, follow-up
  batches, batch size, payload bytes, and pending queue behavior.
- Compose probes print compact success lines owned by verification docs.

## Log Rules

1. Avoid player names and raw UUIDs in routine metric labels.
2. Prefer phase, operation, chunk count, elapsed time, and success fields.
3. Warnings must indicate an operator-relevant failure or retry.
4. Debug logs may describe ignored packets and accepted movement.
5. Probe output should stay compact enough for CI logs.

## Required Evidence

- Scale claims need recorded probe output under
  [../../operations/verification/results/README.md](../../operations/verification/results/README.md).
- New counters need a verification doc naming the command that observes them.
- Runtime exposure evidence must not rely on private process state when a wire
  probe can observe the same behavior.

## Deferred

- Prometheus endpoint.
- OpenTelemetry export.
- Per-region tick histograms.
- Long-running dashboard contracts.
