# The Bumblezone — validation record

Pin **`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`**, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, game 26.3, `api = "0.2"`.
No registry/ledger/PR edit was made by this job.

## Current version: 0.1.1 — nine biomes + the layer stack

| | |
|---|---|
| package | `ports/the_bumblezone/the_bumblezone.hmod` |
| size | **37,299 B** |
| SHA-256 | `d9b8f63c7d5438fe6b719af08efa5eb33aab87d34e1801c910e360385a046907` |
| version | **0.1.1** |
| entries | **18** (9 from 0.1.0 + 9 `data/the_bumblezone/worldgen/biome/*.json`) |
| packed `Cargo.lock` | none |
| tree match | **every packed file is the tree's bytes** (`verify-sdk-package.py`) |
| runtime | `src/runtime.rs` sha `1d5ae47cd8d9…`, byte-identical in tree and package |
| jar-derived asset bytes | **0** (5,509 jar hashes, 18 tree candidates / 86,544 B, 18 package entries) |

### Gates (fresh, against the delivered bytes)

| gate | command | result | log |
|---|---|---|---|
| codec gate | SDK surface / game decoder inspection | **accepted** — decoder `BiomeInfo::from_json` + `Biomes::with_packs`; no `register_biome` in SDK | `evidence/the_bumblezone/biome-codec-gate.log` |
| data contract (red) | added test before the carry | **FAIL** — `the_source_answers_the_jars_biomes`, left = 4 vanilla ids, right = 9 mod ids; `9 passed; 1 failed` | `evidence/the_bumblezone/unit-test-red-011.log` |
| data contracts | `carry.py --check`, `biome_contract.py --check`, `gen_biomes.py --check` | OK: 23 dimension keys; 9 biomes / 8 tags / 7 families; 9 generated files | — |
| unit tests | `cargo test --offline` (after final build) | **12 passed, 0 failed** | `evidence/the_bumblezone/sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, 0 warnings | `evidence/the_bumblezone/sdk-clippy.log` |
| format | `rustfmt --edition 2024 --check src/{lib,runtime}.rs` | clean | `evidence/the_bumblezone/sdk-fmt.log` |
| SDK check (source) | `cargo horizon-mod check` | *142 API items; Made for 0a875da* | `evidence/the_bumblezone/sdk-check.log` |
| package check | `cargo horizon-mod check the_bumblezone.hmod` | *0.1.1, native-1/0a875da…, 37,299 B, sha d9b8f63c…* | `evidence/the_bumblezone/sdk-package-check.log` |
| fits (pin) | `cargo horizon-mod fits … --build 0a875da` | **`0a875da made for`**, exit 0 | `evidence/the_bumblezone/sdk-fits.log` |
| fits (other build) | `cargo horizon-mod fits … --build 0b14f8a` | **`0b14f8a fits`**, exit 0 | `evidence/the_bumblezone/sdk-fits-other.log` |
| provenance | `python3 tools/check_provenance.py` | **0 jar-derived bytes**, wrote `PROVENANCE.md` | `evidence/the_bumblezone/provenance.log` |
| package gate | `verify-sdk-package.py the_bumblezone` | **1/1** | — |
| e2e log gate | `check-e2e-logs.py the_bumblezone-e2e --strict` | `logs=4 ok=4 missing-exit=0 bad-exit=0` | — |
| skills gate | `check-skills.py --notes ports/the_bumblezone --strict` | PASS | — |
| skipped gates | `check-recipe-types.py`, `check-manifest-deps.py` | no recipe types, no other registry mod named in code | — |

Every Rust step ran through `ports/tools/build-slot.sh` with
`CARGO_INCREMENTAL=0`, forwarded to the remote runner at **AdLf-PC** (logs begin
with `build-slot: forwarding …`). Only the launcher game build and the four e2e
legs were local. The launcher reported `Compiling the_bumblezone (1 of 1)`, and
the exported `native-mods.json` named **0.1.1** before the legs ran.

### In-game legs (`ports/_shared/evidence/the_bumblezone-e2e/`)

| leg | result | log |
|---|---|---|
| modded game | **PASS + `EXIT:0`** — probed cell names `the_bumblezone:sugar_water_floor (4)` | `e2e.log` |
| control (unmodded) | **FAIL + `EXIT:1`** — `Unknown dimension`, and the probe answers `Unknown or incomplete command` | `e2e-control.log` |
| `SELFTEST=1` (inverted biome: expects `minecraft:plains`) | **FAIL + `EXIT:1`** | `e2e-selftest.log` |
| `BLOCK_SELFTEST=1` | **FAIL + `EXIT:1`** | `e2e-block-selftest.log` |

The modded readout, verbatim:

```
ctl chat "/the_bumblezone dimension 0 8 0 20261105"
  -> the_bumblezone:the_bumblezone | generator the_bumblezone:chunk_generator |
     biome source the_bumblezone:biome_source | min_y=0 height=256 ambient_light=0.15
     coordinate_scale=4 monster_spawn_light_level=11..=11 | layers 7 |
     biome the_bumblezone:sugar_water_floor (4)
PASS / EXIT:0
```

### A defect found and fixed in the carry (artefact disagreement)

The first package attempt packed biome JSON with `carvers: {}` and the game
refused to open the world: `invalid world generation field: type` (the
generator's carver decoder treats a single object as one carver definition and
demands its `type`). The 26.3 vanilla biome files use a **list** of carvers, so
`tools/gen_biomes.py` now emits `carvers: []`; the rebuilt package launches and
all four legs are green. The failed run is kept in
`ports/_shared/evidence/e2e-guard/the_bumblezone-20261124-132706-1083939.log`.

## History: 0.1.0 (first slice)

Package **29,681 B**, sha
`7ce18cc9380e6a535899cedc32b3ab0a3115a7c629a8a7fdf6178a945bc9ec9c`, 9 entries;
registered the three ids and the `dimension_type` values; 9 tests, both fits
lines green, 4 e2e legs green. Superseded by 0.1.1.

## Re-run

```sh
# contracts and provenance
ports/the_bumblezone/tools/carry.py --check
ports/the_bumblezone/tools/biome_contract.py --check
ports/the_bumblezone/tools/gen_biomes.py --check
ports/the_bumblezone/tools/check_provenance.py --check

# ladder (each a separate lease; compile steps forward automatically)
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step test  -- env CARGO_INCREMENTAL=0 cargo test --offline
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step lint  -- env CARGO_INCREMENTAL=0 cargo clippy --all-targets --offline -- -D warnings
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step check -- cargo horizon-mod check
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step build -- bash -c 'rm -f Cargo.lock && cargo horizon-mod build && cp target/horizon-mod/the_bumblezone.hmod the_bumblezone.hmod'
ports/tools/launcher-game.sh build the_bumblezone
ports/tools/build-slot.sh run --job BZ --port ports/the_bumblezone --step e2e -- \
  bash -lc 'cd ports/the_bumblezone && eval "$(/home/adlf/Desktop/all_the_mods_10_rust_rewrite/ports/tools/launcher-game.sh env the_bumblezone)" && tools/e2e.sh'
```
