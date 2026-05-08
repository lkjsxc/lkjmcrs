# Terrain Generation Smoke

## Goal

Verify the first natural-terrain lane through the public play protocol.

## Command

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --quiet-pull -T terrain-generation
```

## Coverage

- Connects to `terrain-server` using `config/verify/terrain-server.json`.
- Verifies the initial chunk batch shape.
- Verifies spawn plateau chunks keep the current flat safe surface.
- Verifies at least one outer bootstrap chunk has non-flat deterministic
  terrain.
- Verifies chunk batches use embedded light in `level_chunk_with_light`.

## Rules

1. This probe does not require Terra config compatibility.
2. Terrain evidence is about generated blocks only; caves, ores, biomes, trees,
   mobs, weather, structures, and Anvil files remain out of scope.
