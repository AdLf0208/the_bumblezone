# The Bumblezone — a Horizon 26.3 port

A clean-room Rust rewrite of *The Bumblezone* by **TelepathicGrunt**
(<https://github.com/TelepathicGrunt/Bumblezone>) for Horizon at pin
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a` (mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`).

**Licence:** code LGPL-3.0-only (the jar's `LICENSE.txt` beats its `mods.toml` MIT
claim); assets All Rights Reserved. No upstream art is carried — every visual is
regenerated, and `tools/check_provenance.py` proves zero jar-derived bytes. The code
is published with its complete corresponding source under LGPL-3.0. See
[`LICENSE-NOTICE.md`](LICENSE-NOTICE.md).

## What this first slice does

Registers the three ids the jar declares:

- `the_bumblezone:the_bumblezone` — the dimension,
- `the_bumblezone:chunk_generator` — its generator,
- `the_bumblezone:biome_source` — its biome source,

using the exact values of the jar's `dimension_type` (`min_y` 0, `height` 256,
`logical_height` 256, `coordinate_scale` 4.0, `ambient_light` 0.15, no skylight, a
ceiling, monster light 11..=11 / block limit 12, no raids, no respawn anchors,
piglins safe, beds work, fixed time 6000).

`/the_bumblezone dimension` is a fail-capable readout: it prints the three ids and
the numbers, and is asserted in game on the modded build, the unmodded control and
an inverted `SELFTEST=1` leg.

The Java terrain shape, the mod's ten biomes, its configured features and its client
dimension effects are recorded as gaps in [`notes.md`](notes.md); nothing is faked to
fill them.

## Build and verify

```sh
python3 tools/carry.py --check            # snapshot vs jar vs Rust contract
python3 tools/check_provenance.py --check # zero jar-derived asset bytes
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step test -- \
    env CARGO_INCREMENTAL=0 cargo test --offline
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step check -- \
    cargo horizon-mod check
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step build -- \
    bash -c 'rm -f Cargo.lock && cargo horizon-mod build && cp target/horizon-mod/the_bumblezone.hmod the_bumblezone.hmod'
BUILD_LOCAL=1 ports/tools/launcher-game.sh build the_bumblezone
E2E_TMP=/home/adlf/.cache/horizon-e2e/scratch ports/tools/launcher-game.sh run the_bumblezone
```
