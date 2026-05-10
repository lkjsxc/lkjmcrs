# Quickstart

## Run the Server

```bash
docker compose up --build server
```

Default network boundary:

- Container bind: `0.0.0.0:25565`
- Container port: `25565`
- Host publish: `127.0.0.1:${HOST_PORT:-25565}`

## Try the Wire Probes

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke
```

The smoke probe checks:

- status request,
- ping round trip,
- offline login path,
- known-pack, registry, tag, and feature-flag configuration,
- initial chunk, light, position, and keepalive packets.

## Stop and Clean State

```bash
docker compose down
```

This preserves the `server-data` named volume.

Use the destructive acceptance cleanup only when following
[../operations/verification/compose-pipeline.md](../operations/verification/compose-pipeline.md).
