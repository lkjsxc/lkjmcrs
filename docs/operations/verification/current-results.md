# Current Results

## 2026-05-06

Implementation commit tested: `8cf4213`.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm profile-reconnect`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-place`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml restart server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-check`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`

Results:

- `verify`: pass.
- `smoke`: pass, `multiplayer-mutation probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `persist-place`: pass, `persist-place probe ok`.
- `persist-check`: pass after restart, `persist-check probe ok`.
- Player profile persistence smoke: pass, a player moved to non-default
  position and look values, disconnected, reconnected with the same offline
  UUID, and received the saved state in the initial position packet.
- Multiplayer mutation smoke: pass, two play clients completed bootstrap; the
  actor observed fixed-stone placement and break through prediction
  acknowledgements and block updates; the observer received both authoritative
  block updates without prediction acknowledgements.
- Persistence smoke: pass, fixed-stone placement at `0,80,0` was written
  through the public play wire path, survived server restart with the compose
  `server-data` volume, and was observed in the bootstrap chunk payload.
- Block mutation smoke: still covered by the actor side of the multiplayer
  probe.
- Movement flags regression: pass, movement probe now sends one protocol `774`
  flags byte.
- Rust tests: `79` passed.
- docs maximum line count: `103`.
- source maximum line count: `194`.
- Manual join: user-reported success in the task prompt, with no raw client log
  attached.

## Active Manual Boundary

No active disconnect boundary is known after the movement flags-byte fix.
Record the next exact stock-client disconnect or gameplay blocker before
changing [join-boundary.md](join-boundary.md).
