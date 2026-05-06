# Authentication

## Offline Mode

- Implemented in the current slice.
- Player name is validated for basic length and allowed characters.
- UUID is deterministic from `OfflinePlayer:<name>`.
- No Mojang session request is made.
- Offline-mode runtime is private-only when reachable players are not trusted.
- Exposure rules live in
  [../../operations/deployment/exposure-policy.md](../../operations/deployment/exposure-policy.md).

## Online Mode

- Documented but not implemented in the current slice.
- Setting `online_mode=true` must fail startup.
- Future online mode must run session verification off tick workers.

## Rules

1. Auth mode is configured at startup.
2. Login rejects unsupported protocol before creating a session.
3. Offline UUID generation is covered by tests.
4. Online-mode implementation must include compose-verifiable failure tests.
5. Name-based operator permission is unsafe for public offline-mode servers.
