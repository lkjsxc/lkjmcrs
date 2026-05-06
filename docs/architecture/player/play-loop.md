# Play Loop

## Goal

Keep a joined player session alive while making the first observable play-state
values deterministic and testable.

## Session-Local State

Each play session starts from the loaded player profile and tracks:

- server-local session ID,
- position `x`, `y`, `z`,
- yaw and pitch,
- on-ground flag,
- horizontal-collision flag,
- last clientbound keepalive ID,
- current world age and day time.

New profiles start at position `0.5, 80.0, 0.5`, yaw `0.0`, pitch `0.0`,
and both flags set to `false`. Returning profiles start from saved position
and look values.

## Movement

The first milestone accepts these serverbound movement packets:

- `0x1d position`,
- `0x1e position_look`,
- `0x1f look`,
- `0x20 status_only`.

Movement packets update session-local state and may trigger bounded chunk
streaming when the derived chunk center changes. The final state is persisted
on disconnect. Movement does not mutate world chunks, broadcast movement, or
validate survival physics.

The final movement field is one unsigned flags byte:

- bit `0x01`: on ground,
- bit `0x02`: horizontal collision.

Payload decoding must reject trailing bytes in tests. Runtime handling may log
malformed movement and close through the normal connection error path.

## Chunk Streaming

The play loop tracks the current chunk center for each session. Crossing from
center `0,0` to `1,0` with radius `2` sends one new visible column at chunk
`x=3`, unloads column `x=-2`, updates the client chunk-cache center, and
updates the registry subscriptions. The streaming contract is documented in
[../world/chunk-streaming.md](../world/chunk-streaming.md).

## Keepalive

- The server sends keepalive ID `1` during play bootstrap.
- Periodic keepalives continue every `10` seconds while the session is open.
- Serverbound keepalive responses are decoded as signed 64-bit IDs.
- A mismatched keepalive response is logged but does not disconnect the client
  until timeout policy is documented.

## Time

The play loop sends observable time updates after bootstrap:

- update interval: `1` second,
- world age increment: `20` ticks per update,
- day time increment: `20` ticks per update,
- `do_daylight_cycle` remains `true`.

Time is session-visible only in this milestone. It is not yet a persisted world
clock and does not drive block, entity, or weather behavior.

## Outbound Fanout

The play loop owns all writes to its TCP stream. Region-owned gameplay events
that target a session arrive through the internal outbound channel documented in
[chunk-observers.md](chunk-observers.md). This keeps TCP ownership local to the
session while allowing region actors and registries to publish authoritative
updates.

## Pickup Polling

The play loop checks nearby dropped items after accepted movement and after
survival block interactions. Successful pickup sends collect, destroy, and
inventory delta packets through the same TCP writer owned by the session.
