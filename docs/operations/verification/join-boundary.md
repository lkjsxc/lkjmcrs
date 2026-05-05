# Join Boundary

## Current State

As of commit `26f8d20`, all captured packet-shape failures in `tmp/` have
implemented regression fixes and compose smoke coverage:

- login success trailing byte,
- missing required dynamic registry entries,
- chunk section paletted-container raw long shape,
- heightmap fixed-storage long count,
- periodic play keepalives,
- advertised chunk radius versus sent chunk count,
- level-chunk readiness game event `13`.

## Active Manual Boundary

No newer stock-client report has been captured after the readiness event and
live chunk-payload probe fixes. The next active boundary is therefore unknown
until a fresh stock Minecraft Java Edition `1.21.11` join attempt is recorded
against commit `26f8d20` or later.

Existing `tmp/disconnect-*.txt` files are historical evidence unless a newer
manual run reproduces the same failure against the current implementation.

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
