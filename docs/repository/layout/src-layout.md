# Source Layout

## Module Rules

- `src/main.rs` stays a thin binary entrypoint.
- `src/lib.rs` exports internal modules.
- Each module owns one clear behavior group.
- Tests may live beside modules when small.

## Expected Modules

- `app`
- `config`
- `net`
- `probe`
- `protocol`
- `quality`
- `scheduler`
- `session`
- `world`

## Rules

1. Rust source files stay at `<= 200` lines.
2. Split packet helpers by responsibility.
3. Keep protocol constants named and documented.
