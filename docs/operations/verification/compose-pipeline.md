# Compose Pipeline

## Canonical Commands

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml down -v
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --build --quiet-build --quiet-pull -T verify
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T smoke
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T profile-reconnect
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T chunk-stream
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T persist-place
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml restart server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T persist-check
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull survival-server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T survival-item
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T inventory-sync
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T item-pickup
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull survival-vitals-server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T survival-vitals
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull smp-server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T smp-commands
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Behavior

1. `verify` runs Rust formatting, Clippy, tests, docs topology, and line limits.
2. `verify` uses `scripts/verify-static.sh` and owns compact static-gate
   output.
3. Successful `verify` output is limited to `verify <stage> ... ok` lines and
   `verify pass`, aside from unavoidable Compose lifecycle output.
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
14. `survival-vitals-server` mounts `config/verify/smp-server.json` so the
    vitals probe can use disposable operator damage.
15. `survival-vitals` verifies visible health, lethal damage, death, and
    respawn.
16. `smp-commands` verifies offline chat, permissions, travel commands, and
    kick.
17. Non-zero from any step blocks acceptance.
18. Initial `down -v` removes stale named volumes before stateful probes.
19. Final `down -v` removes disposable compose state.
20. Quiet flags are part of the contract for routine acceptance runs.
21. `smp-server` mounts `config/verify/smp-server.json` so disposable operator
    checks do not require operator names in normal runtime config.

## Readiness

- Compose services must not use Docker `healthcheck`.
- Probe commands own readiness.
- Live probes retry their own connection and login boundary for up to `60s`.
- Retry delay starts at `250ms` and caps at `1s`.
- A readiness timeout is a probe failure.

## Stop Rule

No failing compose gate may be ignored for merge acceptance.
