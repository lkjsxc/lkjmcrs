# Online Vanilla Join

## Goal

Capture manual stock-client evidence for public-safe online-mode login.

## Manual Check

Use a stock Minecraft Java Edition `1.21.11` client with a real authenticated
account against a server configured with `online_mode=true`.

Capture:

- server commit,
- server config summary,
- server log lines for the connection,
- client result or disconnect text,
- whether terrain rendered after play entry.

## Expected Boundary

- Server-list ping advertises `1.21.11` and protocol `774`.
- Login performs encryption and session verification.
- The joined profile UUID is the authenticated UUID.
- The client reaches play state and renders the flat spawn terrain.
- A normal client close must not produce a server warning.

## Rules

1. Do not use the disposable compose session fixture for this manual check.
2. Do not record access tokens or account secrets.
3. If the client disconnects, record the exact client text and server phase.
4. Use [evidence-policy.md](evidence-policy.md) before changing the active
   manual boundary.
