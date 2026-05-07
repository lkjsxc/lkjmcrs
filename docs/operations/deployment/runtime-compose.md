# Runtime Compose

## Goal

Run a long-lived local server without destroying state.

## Start

```bash
docker compose up -d --build server
```

Use a non-default host port when needed:

```bash
HOST_PORT=25575 docker compose up -d --build server
```

## Stop

```bash
docker compose down
```

Do not use `down -v` for long-lived runtime state.

## Restart After Config Changes

```bash
docker compose up -d --force-recreate server
```

Config is read once at process startup from `config/server.json`.

## Storage

- Runtime state lives in the `server-data` named volume.
- The container mounts that volume at `/app/data`.
- `world.redb` and `players.redb` live under the configured `data_dir`.
- `docker compose down` preserves named volumes.
- `docker compose down -v` removes named volumes.

## Backup

```bash
mkdir -p backups
docker run --rm \
  -v lkjmcrs_server-data:/from:ro \
  -v "$PWD/backups:/to" \
  alpine:3.22 \
  sh -c 'cd /from && tar czf /to/lkjmcrs-server-data-$(date +%F-%H%M%S).tgz .'
```

## Rules

1. Runtime runbooks must preserve `server-data` unless they explicitly say the
   operation is destructive.
2. Disposable acceptance owns `down -v`; runtime compose does not.
3. Config edits require process restart or container recreate.
4. Image rebuilds must not be required for config edits.
