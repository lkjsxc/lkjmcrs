# Survival Vitals Smoke

## Goal

Prove that visible health, death, and respawn behavior works through the live
Minecraft protocol path.

## Probe Sequence

1. Connect an operator client and a survival target client.
2. Confirm the target receives bootstrap `update_health` with health `20.0`,
   food `20`, and saturation `5.0`.
3. Send `/damage <target> 7.5` from the operator.
4. Expect the target to receive `update_health` with health `12.5`.
5. Send lethal damage from the operator.
6. Expect the target to receive `update_health` with health `0.0` and a death
   combat event.
7. Send serverbound client command action `0` from the target.
8. Expect respawn, position, and `update_health` with restored baseline vitals.

## Acceptance

- The probe uses compose service `survival-vitals`.
- Damage must go through the command and live session registry path.
- Respawn must update the persisted profile state before disconnect.
- Failure blocks normal-survival vitals changes.
