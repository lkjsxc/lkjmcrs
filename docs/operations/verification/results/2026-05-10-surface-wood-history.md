# 2026-05-10 Surface Wood Terrain History

## Covered Work

Implementation commit `dcd1e26` covered canonical research disposition, owned
spruce log and leaf block states, deterministic spruce-style surface
decorators, generated-wood spawn scoring, terrain-quality probing, and natural
render-movement probe hardening.

## Result Summary

The canonical compose sequence passed.

Passing gates and probes:

- static `verify`,
- `smoke`,
- `profile-reconnect`,
- `movement-authority`,
- `chunk-stream`,
- `scale-chunk-stream`,
- `terrain-generation`,
- `terrain-quality`,
- `river-terrain`,
- `terrain-caves`,
- `scale-load-metrics`,
- `scale-moving-pending`,
- `render-distance`,
- `render-moving-pending`,
- `persist-place`,
- `persist-check`,
- `storage-section-persistence`,
- `survival-item`,
- `inventory-sync`,
- `item-pickup`,
- `survival-vitals`,
- `smp-commands`,
- `online-auth`.

Recorded scale counters:

- radius `8`: total `289`, follow-up batches `17`, max payload `963840`.
- radius `32`: total `4225`, follow-up batches `263`, max payload `1031544`.

## Notes

This is historical evidence. The active result is
[../current-results.md](../current-results.md).
