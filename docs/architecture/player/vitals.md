# Vitals

## Goal

Make health, hunger, and saturation visible gameplay state before implementing
full vanilla hunger drain, regeneration, and damage sources.

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

## Damage Command

- `/damage <player> <amount>` is operator-only.
- The target must be a connected player.
- The amount must be positive, finite, and at most `1000.0`.
- Damage uses the current generic damage source boundary and sends a plain death
  message when it is lethal.

## Out of Scope

- Hunger drain, regeneration, starvation, armor, enchantments, and potion
  effects.
- Mob, fall, fire, lava, drowning, and block-specific damage.
- Death screen score, experience loss, item drops, and respawn anchors.
