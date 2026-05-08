# Render Distance Smoke

## Goal

Verify natural-terrain progressive streaming at configured radius `32`.

## Commands

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T render-distance
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T render-moving-pending
```

## Coverage

- Connects to `render32-server` using
  `config/verify/render32-server.json`.
- Verifies login and `chunk_cache_radius` advertise radius `32`.
- Verifies the first batch contains `25` near chunks.
- Verifies eventual convergence to `4225` unique chunks.
- Verifies follow-up batches contain at most `16` chunks.
- Verifies follow-up payload bytes stay within `1048576` unless one oversized
  chunk sends alone.
- Verifies streamed chunks use embedded light.
- Verifies at least one outer chunk is non-flat natural terrain.
- Verifies movement before far streaming completes replaces the pending queue
  with the new radius `32` window.

## Rules

1. This probe does not enable packet compression.
2. Radius `128` remains out of scope until a later acceptance batch.
3. Flat radius `4` and `8` probes remain regression gates.
