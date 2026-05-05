# Compose Pipeline

## Canonical Commands

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm profile-reconnect
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm chunk-stream
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-place
docker compose -f docker-compose.yml -f docker-compose.verify.yml restart server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-check
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Survival Item Slice

Use a separate clean volume because this slice changes new-profile defaults:

```bash
LKJMCRS_DEFAULT_GAME_MODE=survival LKJMCRS_SURVIVAL_STARTER_STONE=1 \
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm survival-item
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Behavior

1. `verify` runs Rust formatting, Clippy, tests, docs topology, and line limits.
2. `server` runs the product binary.
3. `smoke` connects to the live server over the compose network.
4. `profile-reconnect` verifies player profile persistence.
5. `chunk-stream` verifies load-only movement-driven chunk streaming.
6. `persist-place` writes a mutation through the public wire path.
7. `persist-check` verifies that mutation after restart.
8. Non-zero from any step blocks acceptance.
9. Final `down -v` removes disposable compose state.

## Stop Rule

No failing compose gate may be ignored for merge acceptance.
