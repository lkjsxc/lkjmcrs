# Game State Packet ID

## Evidence

- Raw report: `tmp/disconnect-2026-05-05_15.15.50-client.txt`.
- Client: stock Minecraft Java Edition `1.21.11`.
- Time: `2026-05-05 15:15:50`.
- State: play.

## Client Failure

The client rejects a clientbound packet decoded as
`clientbound/minecraft:disguised_chat`. The nested error is:

- `Loading NBT data`
- `Invalid tag id: 13`

## Interpretation

The payload byte `13` is the intended `start_waiting_for_level_chunks` game
state event. It was sent on packet ID `0x21`. In protocol `774`, `0x21` is a
chat packet, so the client tries to decode the payload as chat NBT.

The correct packet ID is `0x26 game_state_change`.

## Required Fix

1. Change the clientbound play game-state packet ID to `0x26`.
2. Keep event ID `13` and value `0.0`.
3. Make the live smoke probe assert the `0x26` packet before chunk radius and
   chunk data.
4. Run compose `verify` and `smoke`.

## Fixed State

Commit `d694a59` implements the packet ID fix and updates the live smoke probe.
Compose `verify` and `smoke` passed on `2026-05-05`.
