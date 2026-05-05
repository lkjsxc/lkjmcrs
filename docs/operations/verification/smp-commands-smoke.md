# SMP Commands Smoke

## Goal

Verify the first chat and command surface through public play packets.

## Compose Command

Run the dedicated SMP command probe service:

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d --build smp-server
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm smp-commands
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Expected Behavior

- `Admin` and `Guest` both receive the declared command tree.
- Plain chat from `Guest` is observed by `Admin` as system chat.
- `Guest` is denied an operator-only command.
- `Admin` changes `Guest` to survival.
- `Guest` reconnects and enters play in survival mode.
- `Admin` kicks `Guest` and the target receives a play disconnect reason.

## Boundary

This probe proves unsigned offline-mode SMP control packets. It does not prove
chat signing, online-mode identity, command suggestions, or full permission
storage.
