# Normal Survival

## Goal

Define the next vanilla-survival target after the current survival sandbox.

## Current Verified Behavior

- New player profiles default to survival unless config overrides gamemode.
- Placement requires a selected held item in survival.
- Breaking supported blocks can create simple drops.
- Nearby dropped items can be collected into inventory.
- Health affects whether the player remains alive.
- Hunger and saturation affect regeneration and starvation.
- Damage can reduce health.
- Death moves the player into a respawn flow.
- Respawn returns the player to spawn with restored baseline vitals.
- Sparse block overrides and profile state persist through `redb`.

## Future Required Behavior

- New players spawn into a dry, readable natural area with nearby wood, water
  access, and enough open ground to orient safely.
- Natural terrain provides ordinary survival starts: surface ground, forests,
  river or coast access, and visible travel routes.
- Tools affect mining speed for the first supported block set.
- Durability decreases when supported tools are used.
- Minimal recipes transform inventory inputs into outputs.
- Spawn and teleport safety checks before broader terrain hazards matter.

## First Vitals Slice

- The server sends health, hunger, and saturation during play bootstrap.
- Operator `/damage <player> <amount>` is the deterministic first damage source.
- Lethal damage sends death state and waits for client respawn request.
- Respawn restores baseline vitals at spawn.
- Survival hunger ticks drain saturation before hunger.
- Natural regeneration runs while health is below `20.0` and hunger is at least
  `18`.
- Starvation damages players at hunger `0`.
- Operator `/vitals <player> <health> <hunger> <saturation>` exists only for
  deterministic administration and probes.

## Deferred Behavior

- Full vanilla recipe book.
- Full enchantments.
- Underground cave gameplay beyond retained generated cave terrain.
- Complete passive and hostile mob ecology.
- Weather effects beyond documented smoke behavior.

## Acceptance Probes

- `probe survival-vitals` must cover damage, death, and respawn when added.
- `probe survival-tools` must cover mining speed, durability, and drops when
  added.
- `probe survival-crafting` must cover at least one shaped or shapeless recipe
  when added.

## Rules

1. Add docs for each gameplay rule before implementation.
2. Keep custom `lkjmcsmp`-style systems outside this path.
3. Prefer small target-aligned slices over broad incomplete systems.
4. Every new survival behavior needs a compose-verifiable probe or unit test.
