# lkjmcrs

`lkjmcrs` is a Rust rewrite of the Minecraft Java Edition server, targeting
Minecraft `1.21.11`.

The project is docs-first. Start at [docs/README.md](docs/README.md), then
change implementation only after the relevant contract is clear.

## Current Milestone

- Rust-only server core.
- First-party minimal `1.21.11` protocol layer.
- Offline-mode playable skeleton first.
- Folia-inspired region ownership and asynchronous task handoff.
- Docker Compose verification as the required acceptance path.

## Verification

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

docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build survival-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm survival-item
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v

docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build smp-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smp-commands
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```
