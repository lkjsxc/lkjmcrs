# Vanilla Join Troubleshooting

## Manual Check

Use a stock Minecraft Java Edition `1.21.11` client in offline mode against the
local server.

Capture both:

- server log lines for the connection,
- the client disconnect text.
- the server commit being tested.

## Expected Boundary

Server-list ping should show version `1.21.11` and protocol `774`.
The reported client error that `minecraft:login_finished` had `1 bytes extra`
must be gone.

During join, a normal client close must not produce a
`WARN ... unexpected end of file` server log.

## Current Limit

The server now sends the documented minimal registry data, full advertised
spawn chunk radius, chunk readiness event, light payloads, initial position, and
periodic keepalives.

The latest documented boundary is tracked in
[join-boundary.md](join-boundary.md). The active report is the vanilla
`clientbound/minecraft:disguised_chat` decode caused by the server sending game
state event `13` on packet ID `0x21` instead of `0x26`.

The next manual stock-client check must recapture the join result after the
`0x26 game_state_change` fix. If a stock client still disconnects before
rendering terrain, record the exact new disconnect text before moving the
boundary again.

Use [evidence-policy.md](evidence-policy.md) to decide whether a new report
becomes the active boundary or only confirms a historical regression.
