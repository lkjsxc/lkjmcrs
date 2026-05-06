# Gate Line Checks

## Goal

Document how the quality gate enforces repository line limits.

## Canonical Policy

The limits are owned by
[../../repository/rules/line-limits.md](../../repository/rules/line-limits.md).

## Gate Behavior

- Markdown files under `docs/` are checked.
- Rust files under `src/` are checked.
- The gate fails when any checked file exceeds its canonical limit.
- Actively edited source files should target `<= 180` lines when a clean split
  is available.
- `docs validate-topology` also fails when a docs directory `README.md` omits
  an immediate child link.
