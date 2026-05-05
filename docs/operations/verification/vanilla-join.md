# Vanilla Join Troubleshooting

## Manual Check

Use a stock Minecraft Java Edition `1.21.11` client in offline mode against the
local server.

Capture both:

- server log lines for the connection,
- the client disconnect text.

## Expected Boundary

Server-list ping should show version `1.21.11` and protocol `774`.
The reported client error that `minecraft:login_finished` had `1 bytes extra`
must be gone.

During join, a normal client close must not produce a
`WARN ... unexpected end of file` server log.

## Current Limit

The server now sends minimal registry data for the registries the vanilla
client reported as required, including the `minecraft:timeline` tag binding
used by the overworld dimension and the grouped `minecraft:damage_type`
bootstrap entries used by client-level construction.

The latest fixed boundary was a terrain-loading timeout caused by advertising
chunk-cache radius `2` while sending only a `3x3` chunk batch. The automated
smoke probe now requires the full `5x5` batch for that radius. See
[client-reports/terrain-radius-timeout.md](client-reports/terrain-radius-timeout.md).

The next manual stock-client check must recapture the join result after the
full-radius batch fix. If the client still disconnects before rendering
terrain, record the exact new disconnect text here before moving the boundary
again.
