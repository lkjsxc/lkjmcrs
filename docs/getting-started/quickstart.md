# Quickstart

## Run the Server

```bash
docker compose up --build server
```

Default bind:

- Host: `0.0.0.0`
- Container port: `25565`
- Host port: `${HOST_PORT:-25565}`

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
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```
