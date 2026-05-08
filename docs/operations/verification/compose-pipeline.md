# Compose Pipeline

## Canonical Commands

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml down -v
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --build --quiet-build --quiet-pull -T verify
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T smoke
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T profile-reconnect
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T chunk-stream
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull scale-server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T scale-chunk-stream
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull terrain-server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T terrain-generation
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull scale-load-server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T scale-load-metrics
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T scale-moving-pending
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
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build --quiet-build --quiet-pull online-server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T online-auth
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
6. `smoke` connects to the live server over the compose network and runs the
   full status, ping, play bootstrap, mutation, observer, and keepalive path.
7. `profile-reconnect` verifies player profile persistence.
8. `chunk-stream` verifies bounded movement-driven chunk streaming.
9. `scale-chunk-stream` verifies progressive radius `4` chunk streaming.
10. `terrain-generation` verifies natural terrain outside the spawn plateau and
    embedded chunk light without `update_light`.
11. `scale-load-metrics` verifies radius `8` total chunks, follow-up batch
    sizes, payload bytes, and scale counter emission.
12. `scale-moving-pending` verifies stale pending chunks are replaced when a
    player moves before radius `8` far streaming completes.
13. `persist-place` writes a mutation through the public wire path.
14. `persist-check` verifies that mutation after restart.
15. `survival-item` verifies survival profile defaults and item persistence.
16. `inventory-sync` verifies client-visible hotbar and player inventory sync.
17. `item-pickup` verifies dropped item entity spawn, pickup, and inventory
    delta sync.
18. `survival-vitals-server` mounts `config/verify/smp-server.json` so the
    vitals probe can use disposable operator damage.
19. `survival-vitals` verifies visible health, lethal damage, death, respawn,
    regeneration, and starvation.
20. `smp-commands` verifies offline chat, permissions, travel commands, and
    kick.
21. `online-auth` verifies encrypted login and fixture-authenticated UUIDs.
22. Non-zero from any step blocks acceptance.
23. Initial `down -v` removes stale named volumes before stateful probes.
24. Final `down -v` removes disposable compose state.
25. Quiet flags are part of the contract for routine acceptance runs.
26. `smp-server` mounts `config/verify/smp-server.json` so disposable operator
    checks do not require operator UUIDs in normal runtime config.
27. `online-server` mounts `config/verify/online-server.json` and may use the
    HTTP session fixture only with explicit insecure-fixture allowance.

## Readiness

- Compose services must not use Docker `healthcheck`.
- Probe commands own readiness.
- Live probes retry their own connection and login boundary for up to `60s`.
- Retry delay starts at `250ms` and caps at `1s`.
- A readiness timeout is a probe failure.

## Stop Rule

No failing compose gate may be ignored for merge acceptance.
