# Inventory Sync Smoke

## Goal

Verify client-visible hotbar and player inventory projection through the public
play wire path.

## Compose Command

Run the dedicated survival server:

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build inventory-sync-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm inventory-sync
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Behavior

- Play bootstrap sends `held_item_slot` with slot `0`.
- Play bootstrap sends `set_player_inventory` for slots `0..35`.
- All bootstrap inventory slots are empty.
- Invalid held-slot input resends authoritative selected slot `0`.
- Breaking grass and pickup sends a slot `0` delta with dirt item ID `28`.
- Accepted dirt placement sends a slot `0` delta to empty.

## Boundary

This probe does not prove full container windows, cursor items, armor, offhand,
crafting, recipes, or item entities.
