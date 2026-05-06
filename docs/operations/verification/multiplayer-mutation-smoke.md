# Multiplayer Mutation Smoke

## Goal

Prove that an accepted region-owned block mutation is visible to another play
session subscribed to the changed chunk.

## Scenario

1. Start the server through Docker Compose.
2. Open two offline login/play probe clients.
3. Complete configuration and play bootstrap for both clients.
4. Client A acquires dirt and places it at `0,80,0`.
5. Client A must receive the prediction acknowledgement.
6. Both clients must receive the authoritative block update for dirt.
7. Client A breaks the same block back to air.
8. Client A must receive the prediction acknowledgement.
9. Both clients must receive the authoritative block update for air.

## Assertions

- Client B must not receive prediction acknowledgements for Client A actions.
- Both clients must remain in play while time or keepalive packets may arrive
  between mutation packets.
- The changed block is inside the advertised spawn radius.
- Failure in this probe blocks acceptance for observer fanout or persistence
  changes that affect mutation visibility.
