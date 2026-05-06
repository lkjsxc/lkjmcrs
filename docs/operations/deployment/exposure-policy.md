# Exposure Policy

## Goal

Keep offline-mode servers private until identity proof exists.

## Offline Mode

- `online_mode=false` is the only implemented authentication mode.
- Offline UUIDs are deterministic from the supplied player name.
- Operator permission is currently name-based through `ops`.
- A public offline-mode server lets reachable users claim operator names.
- Default and shared configs must keep `ops: []`.

## Network Exposure

- Treat offline-mode runtime as private-only.
- Prefer localhost, LAN, VPN, or firewall-restricted access.
- Do not expose TCP `25565` publicly unless online mode is implemented.
- On shared hosts, choose a non-conflicting `HOST_PORT`.
- `HOST_PORT=25575` is the recommended private test override when `25565` is
  already occupied.

## Public Blocker

Internet-facing deployment is blocked until one of these is true:

- `online_mode=true` performs Mojang session verification, or
- a documented external access boundary makes the server private.

## Rules

1. Runtime docs must not present offline mode as public-safe.
2. Example configs must not grant operator names.
3. Public exposure requires a docs update before implementation.
4. Verification probes may use fixed operator names only inside disposable
   compose services.
