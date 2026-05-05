# Level Chunk With Light Report

## Source

Captured report:
`tmp/disconnect-2026-05-05_13.35.41-client.txt`

Client context:

- Minecraft Java Edition `1.21.11`
- Fabric client with client-side mods
- Evidence is accepted only for the vanilla protocol gap it exposes

## Packet Boundary

- Packet: `clientbound/minecraft:level_chunk_with_light`
- Protocol phase: play
- Flow: clientbound
- Error:
  `readerIndex(6345) + length(8) exceeds writerIndex(6345)`

## Interpretation

The client has moved past configuration registries, tag loading, and play login.
It is now decoding the first chunk packet far enough to enter level chunk section
and paletted container deserialization.

The read overrun happens while the client expects another fixed-size raw long
from a paletted container. This points at the chunk section wire shape, not at
registry loading.

## Required Follow-Up

Update the chunk section encoder to match the vanilla `1.21.11` paletted
container contract. Section containers write a bits-per-entry byte, a palette
payload, and then a fixed-size raw long array. They do not write a VarInt
long-array length inside the section data.

## Resolution

Implemented fixed-size raw long encoding for section paletted containers and
added parser-style tests that consume the full chunk-data payload exactly.
