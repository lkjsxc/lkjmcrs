# Damage Type In Fire Report

## Source

Captured report:
`tmp/disconnect-2026-05-05_13.05.50-client.txt`

Client context:

- Minecraft Java Edition `1.21.11`
- Fabric client with client-side mods
- Evidence is accepted only for the vanilla protocol gap it exposes

## Packet Boundary

- Packet: `clientbound/minecraft:login`
- Protocol phase: play
- Flow: clientbound
- Error:
  `Missing element ResourceKey[minecraft:damage_type / minecraft:in_fire]`

## Interpretation

The server completed login and configuration far enough for the client to enter
the play protocol. The client then failed while constructing its client level
from the play login packet.

The missing key proved that the `minecraft:damage_type` dynamic registry must
exist before play login.

## Outcome

The implementation now sends `minecraft:damage_type / minecraft:in_fire`.
The next report moved the same boundary to `minecraft:campfire`.
