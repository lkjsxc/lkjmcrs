# Vitals

## Goal

Make health, hunger, and saturation visible gameplay state before implementing
the full damage-source ecosystem.

## Stored Fields

- Health is a floating-point value from `0.0` through `20.0`.
- Hunger is an integer value from `0` through `20`.
- Saturation is a floating-point value from `0.0` through `20.0`.
- New and respawned players use health `20.0`, hunger `20`, and saturation
  `5.0`.

## Runtime Rules

1. Play bootstrap sends the stored vitals with `update_health`.
2. Operator damage reduces health by a positive finite amount.
3. Health cannot fall below `0.0` or above `20.0`.
4. Lethal damage sets health to `0.0` and marks the session dead.
5. Dead sessions do not accept further block interaction mutations.
6. Respawn restores baseline vitals and moves the player to spawn.
7. Disconnect persists the latest vitals with the player profile.
8. Creative sessions do not run hunger, regeneration, or starvation ticks.
9. Dead sessions do not run hunger, regeneration, or starvation ticks.

## Hunger Loop

- Survival sessions evaluate vitals every `4s`.
- If health is below `20.0` and hunger is at least `18`, regenerate `1.0`
  health.
- Regeneration spends `1.0` saturation first; if saturation is empty, it spends
  `1` hunger instead.
- If regeneration does not run, the idle tick spends `0.5` saturation first.
- If saturation is empty, the idle tick spends `1` hunger.
- Hunger cannot fall below `0` or above `20`.
- Saturation cannot fall below `0.0` or above `20.0`.
- Hunger `0` causes `1.0` starvation damage on each hunger tick.
- Vitals changes send `update_health` immediately.

## Damage Command

- `/damage <player> <amount>` is operator-only.
- The target must be a connected player.
- The amount must be positive, finite, and at most `1000.0`.
- Damage uses the current generic damage source boundary and sends a plain death
  message when it is lethal.

## Vitals Command

- `/vitals <player> <health> <hunger> <saturation>` is operator-only.
- The target must be a connected player.
- Health must be finite from `0.0` through `20.0`.
- Hunger must be an integer from `0` through `20`.
- Saturation must be finite from `0.0` through `20.0`.
- Health `0.0` marks the target dead and sends the current death event.
- Health above `0.0` clears the target death flag and sends `update_health`.

## Out of Scope

- Armor, enchantments, and potion effects.
- Mob, fall, fire, lava, drowning, and block-specific damage.
- Death screen score, experience loss, item drops, and respawn anchors.
