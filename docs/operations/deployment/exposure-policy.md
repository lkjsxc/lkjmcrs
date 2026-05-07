# Exposure Policy

## Goal

Keep public servers tied to authenticated UUID identity.

## Offline Mode

- Offline UUIDs are deterministic from the supplied player name.
- A public offline-mode server lets reachable users claim names.
- Offline mode is private-only.
- Default and shared configs must keep `operator_uuids: []`.

## Online Mode

- `online_mode=true` performs encrypted login and session verification.
- The Mojang session server is the default verifier.
- HTTP verifier URLs require `allow_insecure_session_server=true` and are only
  for disposable verification fixtures.
- Operator permission is UUID-based through `operator_uuids`.

## Network Exposure

- Treat offline-mode runtime as private-only.
- Prefer localhost, LAN, VPN, or firewall-restricted access.
- Do not expose TCP `25565` publicly unless `online_mode=true` is configured.
- Default Compose publishing binds `127.0.0.1:${HOST_PORT:-25565}:25565`.
- On shared hosts, choose a non-conflicting `HOST_PORT`.
- `HOST_PORT=25575` is the recommended private test override when `25565` is
  already occupied.

## Public Blocker

Internet-facing deployment is blocked when `online_mode=false`.

## Rules

1. Runtime docs must not present offline mode as public-safe.
2. Example configs must not grant operator UUIDs.
3. Public exposure requires a docs update before implementation.
4. Verification probes may use fixed operator UUIDs only inside disposable
   compose services.
