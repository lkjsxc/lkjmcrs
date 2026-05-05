# Core Principles

## Design Principles

1. Region ownership is the default answer for world mutation.
2. Cross-region behavior uses explicit task handoff.
3. Blocking I/O never runs on tick workers.
4. Protocol constants stay documented beside implementation.
5. Client compatibility work is accepted only when it is verified by wire tests.
6. Data structures favor locality, compact keys, and predictable ownership.
7. Original gameplay systems wait until the basic server is stable.

## Build Principles

- Rust is the only implementation language for product code.
- Docker Compose is the required verification transport.
- First-party protocol code owns the first `1.21.11` milestone.
- Host machines do not need Rust installed.
- Public API design is deferred until internal ownership rules are proven.
- Docs, source, and verification scripts are optimized for LLM retrieval.
