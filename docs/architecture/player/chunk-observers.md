# Chunk Observers

## Goal

Make block changes visible to every connected play session whose current chunk
subscription includes the changed chunk.

## Subscription Model

- Each play session receives one server-local `SessionId` after login and
  configuration complete.
- Bootstrap sends the full advertised spawn radius before the session subscribes
  to those chunks.
- Movement across a chunk-center boundary removes chunks leaving the visible
  window, sends unload packets, sends newly visible chunks, and updates the
  session subscription set.
- Disconnect removes the session and all subscriptions.
- Subscriptions are bounded by the current visible window.

## Outbound Messages

- The play loop is the only task that writes to its TCP stream.
- Other tasks send session-bound messages through bounded internal channels.
- If a channel is closed, the registry treats that session as disconnected for
  future fanout.
- Backpressure policy is fail-closed for now: a full outbound channel may drop
  that session from the registry rather than blocking region progress.

## Fanout

1. A session decodes a vanilla-shaped block interaction packet.
2. The session submits the mutation to the owning region actor.
3. The region actor returns the authoritative result.
4. The initiating session receives prediction acknowledgement and direct
   reconciliation on its own TCP writer.
5. Accepted mutations in loaded chunks broadcast a single-block update to every
   subscribed observer except the initiator.
6. Dropped item entity spawn, collect, and destroy fan out to sessions
   subscribed to the entity chunk.

Invalid, immutable, or unloaded mutations must not broadcast to observers. The
initiator may still receive reconciliation so its predicted local state matches
the region-owned state.

## Rules

- Region actors remain the authority for final block state.
- Sessions never mutate chunk state directly.
- Observer fanout sends small deltas; it does not resend whole chunks.
- The multiplayer and chunk-stream smoke probes own the acceptance evidence for
  this contract.
