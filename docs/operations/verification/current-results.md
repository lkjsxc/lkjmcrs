# Current Results

## 2026-05-05

Implementation commit tested: `0434b0f`.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`

Results:

- `verify`: pass.
- `smoke`: pass, `multiplayer-mutation probe ok`.
- Multiplayer mutation smoke: pass, two play clients completed bootstrap; the
  actor observed fixed-stone placement and break through prediction
  acknowledgements and block updates; the observer received both authoritative
  block updates without prediction acknowledgements.
- Block mutation smoke: still covered by the actor side of the multiplayer
  probe.
- Movement flags regression: pass, movement probe now sends one protocol `774`
  flags byte.
- Rust tests: `68` passed.
- docs maximum line count: `103`.
- source maximum line count: `198`.
- Manual join: user-reported success in the task prompt, with no raw client log
  attached.

## Active Manual Boundary

No active disconnect boundary is known after the movement flags-byte fix.
Record the next exact stock-client disconnect or gameplay blocker before
changing [join-boundary.md](join-boundary.md).
