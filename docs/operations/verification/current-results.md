# Current Results

## 2026-05-06

Implementation tested: committed tree after implicit runtime config,
held-item-only placement, player SQLite contention hardening, and deterministic
compose probes.

Compose commands:

- `docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
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
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm inventory-sync`
- `docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm item-pickup`
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
- `inventory-sync`: pass, `inventory-sync probe ok`.
- `item-pickup`: pass, `item-pickup probe ok`.
- `smp-commands`: pass, `smp-commands probe ok`.
- Survival material loop smoke: pass, a survival profile joined empty,
  reconciled empty-hand placement, broke grass, picked up dirt, rejected an
  out-of-reach placement without consuming the selected item, placed dirt,
  broke dirt for a persisted pickup, reconnected, placed the persisted dirt,
  and reconciled an empty selected slot without mutation.
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
- Survival item smoke: pass, a new survival profile joined empty, reconciled
  empty-hand placement, acquired dirt through grass break and pickup, rejected
  out-of-reach placement, placed and broke dirt, saved on disconnect, and spent
  the persisted drop after reconnect.
- Inventory sync smoke: pass, play bootstrap sent authoritative selected
  hotbar slot `0` and player inventory slots `0..35`; invalid held-slot input
  resent slot `0`; accepted placement sent a matching empty slot `0` delta;
  accepted breaking spawned a visible item entity and pickup sent the matching
  slot `0` delta.
- Item pickup smoke: pass, a survival profile broke an untouched grass block,
  received item entity spawn and metadata for dirt, moved into pickup range,
  received collect and entity destroy packets, and received a dirt inventory
  delta.
- Player profile persistence smoke: pass, a player moved to non-default
  position and look values, disconnected, reconnected with the same offline
  UUID, and received the saved state in the initial position packet.
- Multiplayer mutation smoke: pass, two play clients completed bootstrap; the
  actor observed held-item placement and break through prediction
  acknowledgements and block updates; the observer received both authoritative
  block updates without prediction acknowledgements.
- Persistence smoke: pass, held-item dirt placement at `3,80,0` was written
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
- Rust tests: `118` passed.
- docs maximum line count: `137`.
- source maximum line count: `200`.
- Manual join: user-reported success in the task prompt, with no raw client log
  attached.

## Active Manual Boundary

No active disconnect boundary is known after the movement flags-byte fix.
Record the next exact stock-client disconnect or gameplay blocker before
changing [join-boundary.md](join-boundary.md).
