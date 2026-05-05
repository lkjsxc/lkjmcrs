# Line Limits

## Hard Limits

- Docs files stay at `<= 300` lines.
- Authored source files stay at `<= 200` lines.
- Rust files under `src/` are checked.
- Markdown files under `docs/` are checked.

## Headroom

- Actively edited source files should target `<= 180` lines when a clean split
  is available.
- Do not reduce clarity only to satisfy line counts.
- Split cohesive modules before a file reaches the hard limit.
