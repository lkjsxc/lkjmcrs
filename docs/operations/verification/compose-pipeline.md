# Compose Pipeline

## Canonical Commands

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Behavior

1. `verify` runs Rust formatting, Clippy, tests, docs topology, and line limits.
2. `server` runs the product binary.
3. `smoke` connects to the live server over the compose network.
4. Non-zero from any step blocks acceptance.
5. Final `down -v` removes disposable compose state.

## Stop Rule

No failing compose gate may be ignored for merge acceptance.
