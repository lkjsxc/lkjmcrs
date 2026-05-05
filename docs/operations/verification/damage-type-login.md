# Damage Type Login Evidence

## Source

Captured report:
`tmp/disconnect-2026-05-05_13.05.50-client.txt`

Client context:

- Minecraft Java Edition `1.21.11`
- Fabric client with client-side mods
- Treated as evidence only for vanilla protocol gaps

## Packet Boundary

- Packet: `clientbound/minecraft:login`
- Phase: play
- Flow: clientbound
- Error:
  `Missing element ResourceKey[minecraft:damage_type / minecraft:in_fire]`

## Interpretation

The server completed login and configuration far enough for the client to enter
the play protocol. Registry synchronization no longer fails at the earlier
configuration boundary.

The client constructs its client level while handling the play login packet.
That construction requires the vanilla `minecraft:damage_type` registry to
contain `minecraft:in_fire`.

## Required Follow-Up

Add a minimal first-party `minecraft:damage_type` registry packet containing
`minecraft:in_fire`, and emit an empty `minecraft:damage_type` tag group in the
configuration tags packet.
