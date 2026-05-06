# Survival Item Smoke

## Goal

Verify the first survival item loop through the public play wire path.

## Compose Command

Run the dedicated survival server:

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build survival-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm survival-item
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm inventory-sync
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Behavior

- A new survival profile joins with an empty synced inventory.
- Empty-hand placement is acknowledged and reconciled without mutation.
- Breaking grass, pickup, dirt placement, and dirt breaking use the selected
  server-side inventory state.
- A reconnect sees the persisted position and inventory state.

## Boundary

This probe proves server-authoritative inventory mutation, block mutation, and
profile persistence for the first survival item loop. `inventory-sync` covers
the client-visible player inventory projection.
