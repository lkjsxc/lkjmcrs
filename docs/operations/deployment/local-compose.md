# Local Compose

## Service

`server` is the product runtime service.

## Defaults

- Image is built from the local Dockerfile.
- Container port is `25565`.
- Host port is `127.0.0.1:${HOST_PORT:-25565}`.
- Working mode is offline by default.
- Runtime data is mounted from the `server-data` named volume to `/app/data`.
- Runtime config is bind-mounted from `./config/server.json` to
  `/app/config/server.json`.
- The service runs `lkjmcrs serve`.
- The Docker image does not copy runtime config files.
- The service has no Docker healthcheck.
- Long-lived runtime commands live in [runtime-compose.md](runtime-compose.md).
- Offline-mode exposure rules live in
  [exposure-policy.md](exposure-policy.md).

## Rules

1. Compose runtime must use the same binary built by release Dockerfile.
2. Verification may use separate cache volumes.
3. Runtime state persists until `docker compose down -v` removes volumes.
4. Config edits require container restart or recreate, not image rebuild.
5. Normal runtime docs must use `docker compose down`, not `down -v`.
