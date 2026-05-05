# Current Results

## 2026-05-05

Implementation commit tested: `6f3e065`.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke`

Results:

- `verify`: pass.
- `smoke`: pass, `login-play probe ok`.
- Block mutation smoke: pass, fixed-stone placement and break observed through
  prediction acknowledgements and block updates.
- Rust tests: `65` passed.
- docs maximum line count: `103`.
- source maximum line count: `185`.
- Manual join: user-reported success in the task prompt, with no raw client log
  attached.

## Active Manual Boundary

No active disconnect boundary is known after the reported successful join.
Record the next exact stock-client disconnect or gameplay blocker before
changing [join-boundary.md](join-boundary.md).
