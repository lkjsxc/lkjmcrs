# Survival Item Smoke

## Goal

Verify the first survival item loop through the public play wire path.

## Compose Command

Run the server with survival defaults and one starter stone:

```bash
LKJMCRS_DEFAULT_GAME_MODE=survival \
LKJMCRS_SURVIVAL_STARTER_STONE=1 \
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm survival-item
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Behavior

- A new survival profile joins with one selected stone item.
- The first placement succeeds and consumes that stone.
- The second placement is acknowledged but reconciled without mutation.
- Breaking a mutable block adds one deterministic simple drop.
- A reconnect sees the persisted position and inventory state.

## Boundary

This probe does not prove full client inventory UI synchronization. It proves
server-authoritative inventory mutation, block mutation, and profile
persistence for the first survival item loop.
