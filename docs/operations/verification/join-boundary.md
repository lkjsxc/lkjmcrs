# Join Boundary

## Current State

As of commit `d694a59`, all captured packet-shape failures in `tmp/` have
implemented regression fixes and compose smoke coverage.

The latest stock-client report is
`tmp/disconnect-2026-05-05_15.15.50-client.txt`. It is now historical packet-ID
evidence:

- vanilla client version: `1.21.11`,
- protocol state: play,
- clientbound packet decoded by the client:
  `clientbound/minecraft:disguised_chat`,
- root cause: the server sent the level-chunk readiness event payload on packet
  ID `0x21`,
- implemented fix: `0x26 game_state_change` with event ID `13`.

Other captured packet-shape failures also have regression fixes and compose
smoke coverage:

- login success trailing byte,
- missing required dynamic registry entries,
- chunk section paletted-container raw long shape,
- heightmap fixed-storage long count,
- periodic play keepalives,
- advertised chunk radius versus sent chunk count,
- level-chunk readiness game-state event `13`.

## Active Manual Boundary

No newer stock-client report has been captured after the `0x26` fix and live
smoke verification. The next active boundary is unknown until a fresh stock
Minecraft Java Edition `1.21.11` join attempt is recorded against commit
`d694a59` or later.

## Evidence Rules

1. Treat `tmp/disconnect-*.txt` as evidence only after linking it from
   [client-reports/README.md](client-reports/README.md).
2. If a report names a clientbound packet, add or strengthen a first-party wire
   assertion for that packet.
3. If a report is only a timeout, prove keepalive progress and bootstrap packet
   order before guessing at unrelated registries.
4. Follow [evidence-policy.md](evidence-policy.md) when deciding whether a
   report is active or historical.
5. Move this boundary only after implementation and compose smoke verification
   agree with the new docs.

## Next Manual Check

Use a stock offline-mode `1.21.11` client against the compose `server` service.
Record the exact disconnect text or successful terrain-rendering result in this
directory before changing the manual boundary again.
