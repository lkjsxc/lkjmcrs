# Current Results

## 2026-05-05

Commit tested: `d694a59`.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke`

Results:

- `verify`: pass.
- `smoke`: pass, `login-play probe ok`.
- docs maximum line count: `103`.
- source maximum line count: `191`.

## Remaining Manual Boundary

Run a stock Minecraft Java Edition `1.21.11` client against commit `d694a59` or
later. Record either successful terrain rendering or the next exact disconnect
report before changing [join-boundary.md](join-boundary.md).
