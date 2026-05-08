# Load And Metrics

## Goal

Define scale evidence before adding broad terrain, entity, and distance claims.

## Metric Targets

- Chunk bootstrap chunks per session: radius `8` starts with `25` chunks.
- Follow-up chunk batch size: at most `8` chunks.
- Follow-up chunk payload bytes: at most `524288` bytes, except that one
  oversized queued chunk may still send alone.
- Radius `8` total loaded chunks: `289` unique chunks.
- Chunk payload cache hits, misses, and override bypasses.
- Pending chunk queue length.
- Follow-up chunk batch, chunk, and payload byte counters.
- Region mailbox depth.
- Storage load and save job durations.
- Active sessions.

## Load Scenarios

- Moving player with progressive chunk streaming.
- Multiple players sharing generated flat chunks.
- Mutation-heavy players in neighboring chunks.
- Reconnect loop with persisted player and world data.

## Log Check

After running scale probes, verify server-side counter emission from the host:

```sh
docker compose -f docker-compose.yml -f docker-compose.verify.yml logs scale-load-server | grep "chunk stream counters"
```

## Rules

1. Metrics must avoid high-cardinality player names and raw UUID labels.
2. Load probes must use Docker Compose.
3. Scale claims require recorded command evidence under verification results.
4. `scale-load-metrics` is the radius `8` automated gate for chunk count,
   follow-up batch size, payload bytes, and emitted scale counters.
5. `scale-moving-pending` proves movement replaces old queued chunks before
   far streaming finishes.
