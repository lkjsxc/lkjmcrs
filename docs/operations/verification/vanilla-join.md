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

The latest fixed boundary was a terrain-loading timeout after the full-radius
chunk batch fix. The server now sends game event `13`, start waiting for level
chunks, before the chunk batch. See
[client-reports/post-radius-terrain-timeout.md](client-reports/post-radius-terrain-timeout.md).

The next manual stock-client check must recapture the join result after the
readiness event fix. If a stock client still disconnects before rendering
terrain, record the exact new disconnect text here before moving the boundary
again.
