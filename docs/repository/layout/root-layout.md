# Root Layout

## Required Root Entries

- `.gitignore`
- `LICENSE`
- `README.md`
- `Cargo.toml`
- `Cargo.lock`
- `Dockerfile`
- `docker-compose.yml`
- `docker-compose.verify.yml`
- `docs/`
- `src/`

## Rules

1. Root stays limited to entrypoint manifests, containers, docs, and source.
2. Product code lives under `src/`.
3. Product and architecture contracts live under `docs/`.
4. Disposable compose state is not committed.
