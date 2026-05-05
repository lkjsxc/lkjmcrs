# Client Reports

Use this subtree for manual Minecraft client disconnect evidence.

## Read This Section When

- You need the latest stock-client or modded-client join blocker.
- You need to decide whether a report exposes a vanilla protocol gap.
- You need evidence before moving the join boundary.

## Child Index

- Reports listed here are historical unless
  [../join-boundary.md](../join-boundary.md) names one as active.
- [game-state-packet-id.md](game-state-packet-id.md): fixed vanilla disconnect
  caused by sending game-state event `13` on chat packet ID `0x21`.
- [movement-flags-byte.md](movement-flags-byte.md): fixed play disconnect
  caused by decoding movement flags as two booleans.
- [post-radius-terrain-timeout.md](post-radius-terrain-timeout.md): fixed
  terrain-loading timeout after the full-radius chunk batch fix.
- [terrain-radius-timeout.md](terrain-radius-timeout.md): fixed terrain-loading
  timeout caused by advertised radius and sent chunk-count mismatch.
- [play-keepalive-timeout.md](play-keepalive-timeout.md): fixed post-chunk
  timeout after heightmap sizing was fixed.
- [heightmap-long-count.md](heightmap-long-count.md): fixed terrain-load
  warning after chunk-section decoding was fixed.
- [level-chunk-with-light.md](level-chunk-with-light.md): fixed play packet
  chunk-section decoding blocker.
- [damage-type-in-fire.md](damage-type-in-fire.md): first play-login damage
  type blocker.
- [damage-type-campfire.md](damage-type-campfire.md): grouped damage registry
  bootstrap evidence.

See [../join-boundary.md](../join-boundary.md) for the active manual boundary.
