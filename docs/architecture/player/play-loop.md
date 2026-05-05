# Play Loop

## Goal

Keep a joined player session alive while making the first observable play-state
values deterministic and testable.

## Session-Local State

Each play session tracks:

- server-local session ID,
- position `x`, `y`, `z`,
- yaw and pitch,
- on-ground flag,
- horizontal-collision flag,
- last clientbound keepalive ID,
- current world age and day time.

The initial state is spawn position `0.5, 80.0, 0.5`, yaw `0.0`, pitch `0.0`,
and both flags set to `false`.

## Movement

The first milestone accepts these serverbound movement packets:

- `0x1d position`,
- `0x1e position_look`,
- `0x1f look`,
- `0x20 status_only`.

Movement packets update session-local state only. They do not mutate world
chunks, broadcast movement, load chunks, or validate survival physics.

The final movement field is one unsigned flags byte:

- bit `0x01`: on ground,
- bit `0x02`: horizontal collision.

Payload decoding must reject trailing bytes in tests. Runtime handling may log
malformed movement and close through the normal connection error path.

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
