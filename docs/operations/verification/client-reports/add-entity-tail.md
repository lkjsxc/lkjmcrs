# Add Entity Tail Report

## Source

- Raw source: user-pasted Minecraft Network Protocol Error Report.
- Report time: `2026-05-07 03:47:22`.
- Client: Minecraft Java Edition `1.21.11`.
- Client type: Fabric/modded.
- State: play.
- Flow: clientbound.
- Tested server commit: current HEAD after registry contract verification.
- Evidence class: active modded evidence that exposes a vanilla protocol shape
  issue.

## Client Failure

The client disconnected while decoding
`play/clientbound/minecraft:add_entity`. The decoder reported the packet was
larger than expected with `5` extra bytes.

## Interpretation

The dropped item entity spawn packet used the wrong protocol `774` tail shape.
For zero velocity, `add_entity` uses one zero `LP_VECTOR3` value, then zero
pitch, yaw, and headYaw bytes, then object data VarInt `0`.

## Fixed State

The item entity packet contract requires the zero-velocity spawn tail
`0x00 0x00 0x00 0x00 0x00` after position. Regression coverage must inspect
the live packet payload and assert no trailing bytes remain.
