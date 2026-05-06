# Verification

## Canonical Compose Flow

The full command owner is
[../operations/verification/compose-pipeline.md](../operations/verification/compose-pipeline.md).
Run that flow for acceptance.

## Required Result

- `verify` exits `0`.
- Successful `verify` service output is concise:

```text
verify fmt ... ok
verify clippy ... ok
verify test ... ok
verify docs-topology ... ok
verify line-limits ... ok
verify pass
```

- `server` becomes reachable on port `25565` inside the compose network.
- `smoke` exits `0`.
- `persist-check` exits `0` after a server restart.
- Survival, inventory, item pickup, chunk stream, reconnect, and SMP command
  probes exit `0`.
- `down -v` clears disposable compose state.

## Stop Rule

No failing compose gate may be ignored for acceptance.
