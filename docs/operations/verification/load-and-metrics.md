# Load And Metrics

## Goal

Define scale evidence before adding broad terrain, entity, and distance claims.

## Metric Targets

- Chunk bootstrap chunks per session.
- Chunk bootstrap bytes per session.
- Follow-up chunk batch size.
- Follow-up chunk payload bytes.
- Chunk payload cache hits and misses.
- Region mailbox depth.
- Storage read and write duration.
- Storage commit duration.
- Active sessions.

## Load Scenarios

- Moving player with progressive chunk streaming.
- Multiple players sharing generated flat chunks.
- Mutation-heavy players in neighboring chunks.
- Reconnect loop with persisted player and world data.

## Rules

1. Metrics must avoid high-cardinality player names and raw UUID labels.
2. Load probes must use Docker Compose.
3. Scale claims require recorded command evidence under verification results.
