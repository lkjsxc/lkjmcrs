# Compatibility

## Client Target

- Minecraft Java Edition `1.21.11`.
- Protocol version `774`.
- Only exact-version compatibility is expected in the first milestone.

## Authentication

- `online_mode=false` is implemented first.
- `online_mode=true` is a documented future mode.
- If `online_mode=true` is configured before implementation, startup must fail
  with a clear unsupported-mode error.

## Server List

The status response advertises:

- name: `1.21.11`,
- protocol: `774`,
- MOTD from configuration,
- current player count,
- max player count.

## Compatibility Non-Goals

- No ViaVersion-style multi-version support.
- No Bedrock support.
- No Bukkit/Paper/Folia plugin compatibility.
- No migration promises for pre-1.0 internal files.
