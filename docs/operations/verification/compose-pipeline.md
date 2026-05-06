# Compose Pipeline

## Canonical Commands

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm profile-reconnect
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm chunk-stream
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-place
docker compose -f docker-compose.yml -f docker-compose.verify.yml restart server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-check
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build survival-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm survival-item
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm inventory-sync
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm item-pickup
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build smp-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smp-commands
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Behavior

1. `verify` runs Rust formatting, Clippy, tests, docs topology, and line limits.
2. `server` runs the product binary.
3. `smoke` connects to the live server over the compose network.
4. `profile-reconnect` verifies player profile persistence.
5. `chunk-stream` verifies bounded movement-driven chunk streaming.
6. `persist-place` writes a mutation through the public wire path.
7. `persist-check` verifies that mutation after restart.
8. `survival-item` verifies survival profile defaults and item persistence.
9. `inventory-sync` verifies client-visible hotbar and player inventory sync.
10. `item-pickup` verifies dropped item entity spawn, pickup, and inventory
    delta sync.
11. `smp-commands` verifies offline chat, permissions, travel commands, and
    kick.
12. Non-zero from any step blocks acceptance.
13. Initial `down -v` removes stale named volumes before stateful probes.
14. Final `down -v` removes disposable compose state.

## Stop Rule

No failing compose gate may be ignored for merge acceptance.
