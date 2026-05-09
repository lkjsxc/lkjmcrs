# Survival Target

## Goal

Describe the near product target for normal survival without mixing in custom
`lkjmcsmp` systems.

## Current Baseline

- New player profiles default to survival unless config overrides gamemode.
- Placement requires a selected held item in survival.
- Breaking supported blocks can create simple drops.
- Nearby dropped items can be collected into inventory.
- Health, hunger, saturation, damage, death, and respawn are visible and
  compose-verified.
- Sparse block overrides and profile state persist through `redb`.

## Next Normal-Survival Capabilities

- Tool-aware mining speed for the first supported block set.
- Tool durability loss for supported actions.
- Minimal crafting for one or more documented recipes.
- Spawn and teleport safety checks before broader terrain hazards matter.

## Acceptance

- Each capability needs a product rule doc before implementation.
- Each user-visible behavior needs a compose probe or focused unit test.
- Custom gameplay remains outside the normal survival path until the basic
  loop is credible.

## Out of Scope

- Full recipe book.
- Enchantments.
- Mob ecology.
- Weather gameplay.
- Structures, caves, ores, and decorations.
- Public plugin API.
