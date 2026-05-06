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
2. `verify` uses `scripts/verify-static.sh` and owns compact static-gate
   output.
3. Successful `verify` output is limited to `verify <stage> ... ok` lines and
   `verify pass`, aside from Compose lifecycle output.
4. Failed `verify` output prints `verify <stage> ... failed`, then dumps only
   the captured stdout and stderr for that failed stage.
5. `server` runs the product binary.
6. `smoke` connects to the live server over the compose network.
7. `profile-reconnect` verifies player profile persistence.
8. `chunk-stream` verifies bounded movement-driven chunk streaming.
9. `persist-place` writes a mutation through the public wire path.
10. `persist-check` verifies that mutation after restart.
11. `survival-item` verifies survival profile defaults and item persistence.
12. `inventory-sync` verifies client-visible hotbar and player inventory sync.
13. `item-pickup` verifies dropped item entity spawn, pickup, and inventory
    delta sync.
14. `smp-commands` verifies offline chat, permissions, travel commands, and
    kick.
15. Non-zero from any step blocks acceptance.
16. Initial `down -v` removes stale named volumes before stateful probes.
17. Final `down -v` removes disposable compose state.

## Readiness

- Compose services must not use Docker `healthcheck`.
- Probe commands own readiness.
- Live probes retry their own connection and login boundary for up to `60s`.
- Retry delay starts at `250ms` and caps at `1s`.
- A readiness timeout is a probe failure.

## Stop Rule

No failing compose gate may be ignored for merge acceptance.
