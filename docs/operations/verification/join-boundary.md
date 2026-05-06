# Join Boundary

## Current State

As of current HEAD, all captured packet-shape failures in `tmp/` have
implemented regression fixes and compose smoke coverage.

A successful player join was reported in the task prompt on `2026-05-05` after
the `0x26 game_state_change` fix. No raw client log was attached, so this is
manual success evidence but not a packet-level regression artifact.

The latest stock-client report by filename/report time is
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
- missing required variant registries and timeline tag binding,
- chunk section paletted-container raw long shape,
- heightmap fixed-storage long count,
- periodic play keepalives,
- advertised chunk radius versus sent chunk count,
- level-chunk readiness game-state event `13`.
- oversized dropped item `add_entity` packet tail.

## Active Manual Boundary

No active disconnect boundary is known after the dropped item `add_entity` tail
fix and focused live item-entity verification. The next active boundary is the
first newer stock Minecraft Java Edition `1.21.11` disconnect or gameplay
blocker recorded against implementation commit `e936794` or later.

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
Record the exact disconnect text or successful gameplay observation in this
directory before changing the manual boundary again.
