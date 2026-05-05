# Verification

## Canonical Compose Flow

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Result

- `verify` exits `0`.
- `server` becomes reachable on port `25565` inside the compose network.
- `smoke` exits `0`.
- `down -v` clears disposable compose state.

## Stop Rule

No failing compose gate may be ignored for acceptance.
