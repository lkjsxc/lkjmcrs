# Current Results

## 2026-05-05

Implementation commit tested: `d997cb7`.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke`

Results:

- `verify`: pass.
- `smoke`: pass, `login-play probe ok`.
- Block mutation smoke: pass, fixed-stone placement and break observed through
  prediction acknowledgements and block updates.
- Movement flags regression: pass, movement probe now sends one protocol `774`
  flags byte.
- Rust tests: `66` passed.
- docs maximum line count: `103`.
- source maximum line count: `185`.
- Manual join: user-reported success in the task prompt, with no raw client log
  attached.

## Active Manual Boundary

No active disconnect boundary is known after the movement flags-byte fix.
Record the next exact stock-client disconnect or gameplay blocker before
changing [join-boundary.md](join-boundary.md).
