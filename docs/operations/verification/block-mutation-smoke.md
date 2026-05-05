# Block Mutation Smoke

## Goal

Prove that the live compose server can accept a simple block placement and
break sequence after play bootstrap.

## Probe Sequence

After the existing login-play bootstrap assertions:

1. Send `use_item_on` targeting the grass block at `0,79,0` on face `up`.
2. Expect `block_changed_ack` with the sent sequence.
3. Expect `block_update` for `0,80,0` with block-state ID `1`.
4. Send `player_action` `start_destroy_block` for `0,80,0`.
5. Expect `block_changed_ack` with the sent sequence.
6. Expect `block_update` for `0,80,0` with block-state ID `0`.
7. Continue observing periodic time and keepalive packets.

## Acceptance

- The smoke probe uses the same packet IDs and payload shape as protocol `774`.
- The mutation path must go through the region actor.
- Failure in this probe blocks acceptance for block-interaction changes.
