# Damage Type Campfire Report

## Source

Captured report:
`tmp/disconnect-2026-05-05_13.17.17-client.txt`

Client context:

- Minecraft Java Edition `1.21.11`
- Fabric client with client-side mods
- Evidence is accepted only for the vanilla protocol gap it exposes

## Packet Boundary

- Packet: `clientbound/minecraft:login`
- Protocol phase: play
- Flow: clientbound
- Error:
  `Missing element ResourceKey[minecraft:damage_type / minecraft:campfire]`

## Interpretation

Adding only `minecraft:in_fire` moved the client-level construction boundary to
the next missing built-in damage source. The server should stop iterating one
missing key at a time and send the vanilla damage sources constructed by
`DamageSources` during client-level bootstrap.

## Required Follow-Up

Send a grouped `minecraft:damage_type` registry packet containing the vanilla
client-level bootstrap damage sources, including `minecraft:campfire`.
