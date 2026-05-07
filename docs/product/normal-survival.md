# Normal Survival

## Goal

Define the next vanilla-survival target after the current survival sandbox.

## Required Behavior

- Health affects whether the player remains alive.
- Hunger and saturation affect regeneration and starvation.
- Damage can reduce health.
- Death moves the player into a respawn flow.
- Respawn returns the player to spawn with restored baseline vitals.
- Tools affect mining speed for the first supported block set.
- Durability decreases when supported tools are used.
- Minimal recipes transform inventory inputs into outputs.

## First Vitals Slice

- The server sends health, hunger, and saturation during play bootstrap.
- Operator `/damage <player> <amount>` is the deterministic first damage source.
- Lethal damage sends death state and waits for client respawn request.
- Respawn restores baseline vitals at spawn.
- Hunger, regeneration, and starvation remain stored values only in this slice.

## Deferred Behavior

- Full vanilla recipe book.
- Full enchantments.
- Full natural terrain generation.
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
3. Prefer small vanilla-compatible slices over broad incomplete systems.
4. Every new survival behavior needs a compose-verifiable probe or unit test.
