# The Bumblezone — validation record (first slice, v0.1.0)

Pin **`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`**, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, game 26.3 (Nightly
`nightly-2026-11-20-0a875da`), `api = "0.2"`.

**Version 0.1.0** — first publication. No registry/ledger/PR edit was made by
this job. Cargo and manifest versions agree at 0.1.0.

## The deliverable

| | |
|---|---|
| package | `ports/the_bumblezone/the_bumblezone.hmod` |
| size | **29,681 B** |
| SHA-256 | `7ce18cc9380e6a535899cedc32b3ab0a3115a7c629a8a7fdf6178a945bc9ec9c` |
| version | **0.1.0** |
| entries | **9** (`horizon-mod.toml`, `Cargo.toml`, `LICENSE`, `LICENSE-NOTICE.md`, `README.md`, `icon.png`, `src/lib.rs`, `src/runtime.rs`, `uses.json`) |
| packed `Cargo.lock` | none |
| runtime | `src/runtime.rs` byte-identical in tree and package (`2964e97ba080…`) |
| jar-derived asset bytes | **0** (`tools/check_provenance.py`, 5,509 distinct jar hashes, 9 candidate files / 71,541 bytes + 9 package entries) |

## Gates (fresh, against the delivered bytes)

| gate | command | result | log |
|---|---|---|---|
| data contract (red) | `python3 tools/carry.py --check` before authors | **FAIL**, `tools/dimension_contract.json: missing` | `evidence/the_bumblezone/carry-contract-red.log` |
| data contract (green) | `python3 tools/carry.py --check` | OK, 23 snapshot/jar keys agree, ids in Rust, blocked families absent | `evidence/the_bumblezone/carry-contract.log` |
| unit tests (red) | `cargo test --offline`, zero-valued stub | **1 failed / 4 passed**, `assertion left == right failed: height left: 0 right: 256` | `evidence/the_bumblezone/unit-test-red.log` |
| unit tests | `cargo test --offline` | **9 passed, 0 failed** | `evidence/the_bumblezone/sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `evidence/the_bumblezone/sdk-clippy.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `evidence/the_bumblezone/sdk-fmt.log` |
| SDK check (source) | `cargo horizon-mod check` | *138 API items; Made for 0a875da* | `evidence/the_bumblezone/sdk-check.log` |
| package check | `cargo horizon-mod check the_bumblezone.hmod` | *0.1.0, native-1/0a875da…, 29,681 bytes, sha 7ce18cc9…* | `evidence/the_bumblezone/sdk-package-check.log` |
| fits (pin) | `cargo horizon-mod fits … --build 0a875da` | **`0a875da made for`**, exit 0 | `evidence/the_bumblezone/sdk-fits.log` |
| fits (other build) | `cargo horizon-mod fits … --build 0b14f8a` | **`0b14f8a fits`**, exit 0 | `evidence/the_bumblezone/sdk-fits-other.log` |
| provenance | `python3 tools/check_provenance.py` | **0 jar-derived bytes**, wrote `PROVENANCE.md` | `evidence/the_bumblezone/provenance.log` |
| monogram | `python3 tools/gen_icon.py --check` | 2,096 B, sha `c78c73ab…`, deterministic | — |
| package gate | `verify-sdk-package.py the_bumblezone` | **1/1** | — |
| e2e log gate | `check-e2e-logs.py the_bumblezone-e2e --strict` | `logs=4 ok=4 missing-exit=0 bad-exit=0` | — |
| skills gate | `check-skills.py --notes ports/the_bumblezone --strict` | PASS | — |
| skipped gates | `check-recipe-types.py`, `check-manifest-deps.py` | no recipe types, no other registry mod named in code | — |

Every Rust step ran through `ports/tools/build-slot.sh` with `CARGO_INCREMENTAL=0`
and no `--target-dir`; compile/test/check/build/lint/fmt steps **forwarded to the
remote runner at AdLf-PC** (each log above begins with `build-slot: forwarding …`).
Only the launcher game build and the four e2e legs were local, because
`launcher-game.sh` and this script name the campaign's absolute roots.

The package was built before these notes and this file were written; neither is
packed, so `verify-sdk-package.py` still reports **package matches the tree**.

## In-game legs (`ports/_shared/evidence/the_bumblezone-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS + `EXIT:0`** | `e2e.log` |
| control (same Nightly, no package) | **FAIL + `EXIT:1`** — `Unknown dimension 'the_bumblezone:the_bumblezone'` | `e2e-control.log` |
| `SELFTEST=1` (inverted readout) | **FAIL + `EXIT:1`** — the readout named the real ids, not the fake one | `e2e-selftest.log` |
| `BLOCK_SELFTEST=1` (inverted block read) | **FAIL + `EXIT:1`** — the glass is present | `e2e-block-selftest.log` |

### The modded `ctl` transcript (headless, creative, seed 20261105)

```
ctl state -> {"dimension":"minecraft:overworld","screen":"playing"}  (baseline)
ctl command "execute in the_bumblezone:the_bumblezone run forceload add 8 8"
   -> {"chat":["Marked chunk [0, 0] in the_bumblezone:the_bumblezone to be force loaded"],"ok":true}
ctl command "execute in the_bumblezone:the_bumblezone run setblock 8 8 8 minecraft:glass"
   -> {"chat":["Changed the block at 8, 8, 8"],"ok":true}
ctl command "execute in the_bumblezone:the_bumblezone if block 8 8 8 minecraft:glass run say BZ_GLASS_OK"
   -> {"chat":["[Player16656] BZ_GLASS_OK"],"ok":true}
ctl chat "/the_bumblezone dimension"
   -> the_bumblezone:the_bumblezone | generator the_bumblezone:chunk_generator |
      biome source the_bumblezone:biome_source | min_y=0 height=256 ambient_light=0.15
      coordinate_scale=4 monster_spawn_light_level=11..=11
ctl command "execute in the_bumblezone:no_such_dimension run setblock 8 8 8 minecraft:glass"
   -> {"chat":["Unknown dimension 'the_bumblezone:no_such_dimension'"],"ok":true}
PASS: the modded game registers the_bumblezone:the_bumblezone,
      the_bumblezone:chunk_generator and the_bumblezone:biome_source
EXIT:0
```

The ids in the readout are produced by this package alone: the control game
answers `Unknown dimension` for the same id and has no `/the_bumblezone` command.
`ctl chat` is used for the mod command (the console path bypasses
`run_mod_command`).

## Re-run

```sh
# ladder (each a separate lease; compile steps forward automatically)
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step test  -- env CARGO_INCREMENTAL=0 cargo test --offline
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step lint  -- env CARGO_INCREMENTAL=0 cargo clippy --all-targets --offline -- -D warnings
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step check -- cargo horizon-mod check
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step build -- bash -c 'rm -f Cargo.lock && cargo horizon-mod build && cp target/horizon-mod/the_bumblezone.hmod the_bumblezone.hmod'
BUILD_LOCAL=1 ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step build -- ports/tools/launcher-game.sh build the_bumblezone
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step e2e -- \
  bash -lc 'cd ports/the_bumblezone && eval "$(/home/adlf/Desktop/all_the_mods_10_rust_rewrite/ports/tools/launcher-game.sh env the_bumblezone)" && tools/e2e.sh'
```
