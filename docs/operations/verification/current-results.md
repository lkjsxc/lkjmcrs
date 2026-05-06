# Current Results

## 2026-05-06

Implementation tested: working tree after `32daa87`.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smoke`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm profile-reconnect`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm chunk-stream`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-place`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml restart server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm persist-check`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build survival-server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm survival-item`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build smp-server`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smp-commands`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`

Results:

- `verify`: pass.
- `smoke`: pass, `multiplayer-mutation probe ok`.
- `profile-reconnect`: pass, `profile-reconnect probe ok`.
- `chunk-stream`: pass, `chunk-stream probe ok`.
- `persist-place`: pass, `persist-place probe ok`.
- `persist-check`: pass after restart, `persist-check probe ok`.
- `survival-item`: pass, `survival-item probe ok`.
- `smp-commands`: pass, `smp-commands probe ok`.
- Survival material loop smoke: pass, a survival profile placed starter stone,
  broke it for a simple stone drop, rejected an out-of-reach placement without
  consuming the selected item, placed the retained stone nearby, broke grass
  for a dirt drop, placed dirt from the selected server-side item, broke that
  dirt for a persisted drop, reconnected, placed the persisted dirt, and
  reconciled an empty selected slot without mutation.
- Reach regression: pass, block interactions outside `6.0` blocks from eye
  position acknowledge prediction and reconcile without chunk or inventory
  mutation.
- SMP commands smoke: pass, `Admin` and `Guest` received the declared command
  tree, `Guest` chat reached `Admin` as system chat, `Guest` was denied
  operator-only `/say` and `/setwarp`, `Guest` saved and used personal home
  `base`, `Admin` saved global warp `spawnish`, `Guest` listed and used it,
  `Admin` changed `Guest` to survival, survival mode and the home row
  persisted across reconnect, and `Admin` kicked `Guest` with a play disconnect
  reason.
- Survival item smoke: pass, a new survival profile received one starter
  stone, consumed it on successful placement, reconciled a second empty-slot
  placement without mutation, broke the placed block for a simple drop, saved
  on disconnect, and spent the persisted drop after reconnect.
- Player profile persistence smoke: pass, a player moved to non-default
  position and look values, disconnected, reconnected with the same offline
  UUID, and received the saved state in the initial position packet.
- Multiplayer mutation smoke: pass, two play clients completed bootstrap; the
  actor observed fixed-stone placement and break through prediction
  acknowledgements and block updates; the observer received both authoritative
  block updates without prediction acknowledgements.
- Persistence smoke: pass, fixed-stone placement at `0,80,0` was written
  through the public play wire path, survived server restart with the compose
  `server-data` volume backed by `world.sqlite3`, and was observed in the
  bootstrap chunk payload.
- Chunk-stream smoke: pass, movement from center `0,0` to `1,0` streamed
  column `x=3` and unloaded column `x=-2`; movement to center `2,0` streamed
  column `x=4` and unloaded column `x=-1`; a successful reachable placement in
  streamed chunk column `x=3` used the normal ack and block-update path.
- Block mutation smoke: still covered by the actor side of the multiplayer
  probe.
- Movement flags regression: pass, movement probe now sends one protocol `774`
  flags byte.
- Rust tests: `109` passed.
- docs maximum line count: `103`.
- source maximum line count: `200`.
- Manual join: user-reported success in the task prompt, with no raw client log
  attached.

## Active Manual Boundary

No active disconnect boundary is known after the movement flags-byte fix.
Record the next exact stock-client disconnect or gameplay blocker before
changing [join-boundary.md](join-boundary.md).
