# Current Results

## 2026-05-05

Implementation commit tested: `ebff7b1`.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke`

Results:

- `verify`: pass.
- `smoke`: pass, `login-play probe ok`.
- Rust tests: `55` passed.
- docs maximum line count: `103`.
- source maximum line count: `180`.
- Manual join: user-reported success in the task prompt, with no raw client log
  attached.

## Active Manual Boundary

No active disconnect boundary is known after the reported successful join.
Record the next exact stock-client disconnect or gameplay blocker before
changing [join-boundary.md](join-boundary.md).
