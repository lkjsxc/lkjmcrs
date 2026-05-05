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

The latest captured boundary is play packet decoding inside
`clientbound/minecraft:level_chunk_with_light`. The client reaches level chunk
section and paletted container decoding, then overruns the packet while reading
a fixed raw long. See
[client-reports/level-chunk-with-light.md](client-reports/level-chunk-with-light.md).

If a stock client still disconnects before rendering terrain, record the exact
new disconnect text here before moving the boundary again.
