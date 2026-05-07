# Wording Policy

## Goal

Keep docs easy for agents to scan without carrying release-family language that
suggests compatibility promises.

## Rules

1. Avoid `v1`, `v2`, and similar shorthand in authored docs.
2. Avoid casual `version` wording when `target`, `protocol`, `schema`, or
   `Minecraft release` is more exact.
3. Keep required external names exact when they are protocol or tool facts.
4. Do not create backward-compatibility language unless a current owner doc
   explicitly requires it.

## Exceptions

- Cargo metadata and dependency manifests may use package-manager field names.
- Minecraft protocol facts may use Mojang's own terminology when exact.
- Historical evidence files may quote raw client errors exactly.
