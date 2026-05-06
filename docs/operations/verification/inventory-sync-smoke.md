# Inventory Sync Smoke

## Goal

Verify client-visible hotbar and player inventory projection through the public
play wire path.

## Compose Command

Run the dedicated survival server with one starter stone:

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build survival-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm inventory-sync
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Behavior

- Play bootstrap sends `held_item_slot` with slot `0`.
- Play bootstrap sends `set_player_inventory` for slots `0..35`.
- Starter stone appears in slot `0` as item ID `1` and count `1`.
- Invalid held-slot input resends authoritative selected slot `0`.
- Accepted placement sends a slot `0` delta to empty.
- Accepted breaking sends a slot `0` delta with stone item ID `1`.

## Boundary

This probe does not prove full container windows, cursor items, armor, offhand,
crafting, recipes, or item entities.
