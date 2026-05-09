# Load And Metrics

## Goal

Define scale evidence before adding broad terrain, entity, and distance claims.

## Metric Targets

- Chunk bootstrap chunks per session: radius `32` starts with `25` chunks.
- Follow-up chunk batch size: at most `16` chunks.
- Follow-up chunk payload bytes: at most `1048576` bytes, except that one
  oversized queued chunk may still send alone.
- Radius `32` total loaded chunks: `4225` unique chunks.
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
4. `scale-load-metrics` remains the flat radius `8` regression gate.
5. `render-distance` is the natural radius `32` automated gate for chunk
   count, follow-up batch size, payload bytes, terrain shape, and emitted scale
   counters.
6. `scale-moving-pending` uses an isolated `scale-moving-server` and proves
   movement replaces old queued chunks before far streaming finishes.
7. `render-moving-pending` proves the same boundary for radius `32`.
