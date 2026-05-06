# Block Interaction Packets

## Source Baseline

Protocol `774` packet IDs are pinned from the official `1.21.11` server
`packets.json` report generated from Mojang's server jar.

## Clientbound Packets

- `0x04 block_changed_ack`: VarInt sequence.
- `0x08 block_update`: packed block position, VarInt block-state ID.

For each handled client-predicted mutation packet, the initiator receives
`block_changed_ack`, then a direct `block_update` for the changed or reconciled
position.

## Serverbound Packets

- `0x28 player_action`: VarInt action, packed block position, unsigned-byte
  face, VarInt sequence.
- `0x3c swing`: VarInt hand.
- `0x3f use_item_on`: VarInt hand, block hit result, VarInt sequence.

`player_action` uses these action IDs for this slice:

- `0`: start destroy block.
- `1`: abort destroy block.
- `2`: stop destroy block.

`use_item_on` reads block hit result as:

1. packed block position,
2. VarInt face,
3. hit `x`, `y`, and `z` as `f32` offsets,
4. inside-block boolean,
5. world-border-hit boolean.

## Block State IDs

The first mutation slice reuses the flat-world IDs documented in
[packet-contract.md](packet-contract.md):

- air: `0`,
- stone: `1`,
- bedrock: `85`.

## Rules

1. Decode tests reject trailing bytes.
2. Packet ID tests cover every new ID.
3. The smoke probe must prove placement and breaking over the live compose wire.
4. Empty-hand placement is acknowledged and reconciled without mutation.
5. Unsupported held items are acknowledged and reconciled without mutation.
6. Accepted placement sends the initiator update before observer fanout.
7. Observer fanout excludes the initiating session for that block update.
