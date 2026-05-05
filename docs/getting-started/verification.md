# Verification

## Canonical Compose Flow

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm profile-reconnect
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-place
docker compose -f docker-compose.yml -f docker-compose.verify.yml restart server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-check
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Result

- `verify` exits `0`.
- `server` becomes reachable on port `25565` inside the compose network.
- `smoke` exits `0`.
- `persist-check` exits `0` after a server restart.
- `down -v` clears disposable compose state.

## Stop Rule

No failing compose gate may be ignored for acceptance.
