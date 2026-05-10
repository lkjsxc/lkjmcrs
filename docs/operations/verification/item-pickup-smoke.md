# Item Pickup Smoke

## Goal

Verify that survival drops become client-visible item entities before entering
inventory.

## Compose Command

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build item-pickup-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm item-pickup
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Behavior

- Breaking grass in survival sends block ack and block update.
- The server spawns a dirt item entity in the broken block chunk.
- Entity metadata carries a dirt `Slot` with item ID `28` and count `1`.
- Moving within `1.5` blocks collects the item.
- Pickup sends collect, entity destroy, and a matching player-inventory delta.

## Boundary

This probe does not prove item physics, despawn timers, merging, persistence,
crafting, or full container windows.
