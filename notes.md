# The Bumblezone — port notes (first slice, v0.1.0)

Skills: superpowers/using-superpowers, brainstorming, writing-plans, test-driven-development, systematic-debugging, verification-before-completion; BeautifulRust/rust-best-practices

**Class + tier:** `content` / `deep` (worldgen/dimension). Ledger row
`the_bumblezone,The Bumblezone,content,deep,LGPL-3.0 (code); ARR (assets),pending,P0,,`.
Source map: `ports/_shared/source-map/atm10-sources.csv` (repo
`TelepathicGrunt/Bumblezone`). Approach from `horizon/docs/modpacks.md` §"Tiers"
and §"Carry-over": carry the artefact's ids and dimension values, write the
registrar glue as typed Rust, and leave the Java terrain/feature families as
measured gaps. No renderer or new vanilla hook in this slice.

**Pin:** Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` + horizon-mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, `api = "0.2"`, game 26.3. Do not
repin mid-wave; the additive nightly `f1bb66f353cf…` is not our pin.

**Licence: code LGPL-3.0-only (rank 1), assets ARR (`assets-arr`).** The jar ships
`LICENSE.txt` = "GNU LESSER GENERAL PUBLIC LICENSE, Version 3, 29 June 2007"
(7,817 B) and `LICENSE_ASSETS.txt` = "All Bumblezone Assets are ARR (All Rights
Reserved)" (523 B, with the author's reason: bought/borrowed music, a texture
lent from another mod, and the textures he made himself). `META-INF/neoforge.mods.toml`
says `license="MIT"`, but the specific files win — ARCHITECTURE.md §8's 2026-11-19
ruling names this mod. So:
- **every visual is regenerated**, jar-derived asset byte count **0**, measured by
  `tools/check_provenance.py` (log `ports/_shared/evidence/the_bumblezone/provenance.log`,
  record `PROVENANCE.md`); the icon is a **monogram** drawn by `tools/gen_icon.py`,
  never the project art;
- the **LGPL-3.0-only text travels with the port** (`LICENSE`, canonical SPDX text
  from the system collection, not the jar's file), with `LICENSE-NOTICE.md`
  crediting **TelepathicGrunt** and linking <https://github.com/TelepathicGrunt/Bumblezone>;
- the code is publishable under LGPL-3.0 with its complete corresponding source
  and maintainer approval; the ARR assets are never redistributed.

**Artefact:** `ports/_shared/licence-cache/the_bumblezone-7.17.3+1.21.1-neoforge.jar`,
83,641,779 B, SHA-256 `859d87b44d3d17c63f38317ac34d124e30b6de8c3043fa61ee987ff8b1f236d6`
(re-verified at dispatch). Upstream repo read only as analysis aid; no Java code
was copied or decompiled.

## Jar inventory (measured with `unzip -l` / `unzip -Z1`, before any family was chosen)

6,878 entries, 96,189,149 uncompressed bytes. Top-level census: `data/` 3,294,
`assets/` 1,959, `com/` 1,079 (Java classes), `resourcepacks/` 350, `datapacks/`
180, `META-INF/` 12.

| `data/the_bumblezone/` family | Files | Verdict |
|---|---:|---|
| `structure/` | 906 | not ported (structure registrar exists, templates/processor lists are a later slice) |
| `worldgen/` (biome 10, noise_settings 1, others) | 624 | biomes/noise settings not carried; 10 biome ids are the artefact's |
| `tags/` | 548 | not ported in slice 1 |
| `bz_bee_queen_trades/` | 343 | custom data, not ported |
| `recipe/`, `recipes/` | 197 + 5 | not ported |
| `loot_table/` | 173 | not ported |
| `advancement/` | 162 | not ported |
| `bz_pollen_puff_entity_flowers/`, `bz_crystalline_flower_data/`, `bz_potion_candle_data/`, `neruina/`, `database/`, `loot_modifiers/` | 68 + 13 + 6 + 3 + 3 + 3 | custom data, not ported |
| `jukebox_song/`, `banner_pattern/`, `damage_type/`, `enchantment/` | 15 + 9 + 8 + 5 | not ported |
| **`dimension/`** | **2** | **the id this slice registers** (`the_bumblezone.json`) |
| **`dimension_type/`** | **2** | **the values this slice registers** (`the_bumblezone.json`) |

`assets/the_bumblezone/`: 884 textures, 864 models, 100 blockstates, 36 sounds,
14 langs, 13 particles, 5 shaders, 2 logos. **None carried** (ARR). Extra
`resourcepacks/{shader_emissive,anti_tropophobia}` and eight `datapacks/*_compat`
folders are not packed.

### The carried contract (ids and values, read from the artefact)

`data/the_bumblezone/dimension/the_bumblezone.json`:
`type = the_bumblezone:the_bumblezone`,
`generator.type = the_bumblezone:chunk_generator`,
`generator.settings = the_bumblezone:bz_noise_settings`,
`biome_source.type = the_bumblezone:biome_source`,
`main_biomes = #the_bumblezone:the_bumblezone`,
`blob_biomes = #the_bumblezone:blob_biomes`,
`rare_blob_biomes = #the_bumblezone:rare_blob_biomes`.

The two generator/biome-source ids are confirmed in the constant pool of
`com/telepathicgrunt/the_bumblezone/modinit/BzDimension.class` (tokens
`chunk_generator`, `biome_source`, `the_bumblezone`, `BZ_CHUNK_GENERATOR`,
`BZ_BIOME_SOURCE`) — read as strings only, no decompilation.

`data/the_bumblezone/dimension_type/the_bumblezone.json`, verbatim values:
`min_y 0`, `height 256`, `logical_height 256`, `ambient_light 0.15`,
`has_skylight false`, `has_ceiling true`,
`infiniburn #minecraft:infiniburn_overworld`, `coordinate_scale 4.0`,
`monster_spawn_block_light_limit 12`, `monster_spawn_light_level 11`,
`has_raids false`, `respawn_anchor_works false`, `bed_works true`,
`fixed_time 6000`, `piglin_safe true`, `ultrawarm false`, `natural true`,
`effects the_bumblezone:dimension_special_effects`.

These are transcribed as **values** into `src/runtime.rs` and recorded in
`tools/dimension_contract.json`; `tools/carry.py --check` compares the snapshot,
the embedded Rust contract and the jar. No JSON byte is copied. The SDK has no
field for the fixed time value, `natural`, or the client `effects` id: the port
uses `has_fixed_time = true` + `day_cycle = "none"` and records the 6000 here.

## Gaps / blocked symbols (measured, not guessed)

Evidence for both rows: `ports/_shared/evidence/the_bumblezone/pin-gaps.log`.

- **`blocked(configured-feature-not-registered)`** — the pin's `surface.json` has
  **0** matches each for `configured_feature`, `register_feature`, `FeatureType`
  and `PlacementModifier`; it does have `Registrar::register_structure_type`,
  `register_density_function_type`, `register_biome_source` and
  `register_chunk_generator`. The jar's configured/placed features therefore
  cannot be registered, and no stand-in data is packed. (The same measurement the
  handover records for actuallyadditions and tectonic.)
- **`blocked(level_reset:R11)`** — `surface.json` has **0** matches for
  `levels_reset` and `reset_vanilla`; the additive nightly that adds them is not
  our pin, and `var/autopilot/PLAN.md:199-200` records that registry rule R11
  refuses the required `[native] level_reset` manifest key. So no vanilla level
  or terrain override is packed: carrying one would rewrite a saved world's
  terrain on reload.
- **Deep-slice gaps (not faked):** the Java `BzChunkGenerator` terrain layers and
  `BzBiomeSource`'s nine layer classes are not ported, so the registered generator
  leaves the chunk as air and the registered source answers with four vanilla
  biomes the level can resolve (the mod's ten biome JSON files are ARR data and
  the 26.3 biome codec has not been measured). The `effects` client id has no
  `DimensionSettings` field. None of these is claimed as faithful behaviour.
- **Not in scope of the artefact's first slice:** 906 structures, 548 tags, 204
  recipes, 173 loot tables, 162 advancements, songs, patterns and custom data
  families are recorded in the inventory above for later slices.

## Coverage table

| Family in the jar | State | Evidence / reason |
|---|---|---|
| `the_bumblezone:the_bumblezone` dimension | registered | `src/lib.rs::register` → `Registrar::register_dimension("the_bumblezone", settings())`; in-game readout + `execute in` |
| `the_bumblezone:chunk_generator` id | registered | `register_chunk_generator("chunk_generator", …)`; terrain shaping not ported (deep slice) |
| `the_bumblezone:biome_source` id | registered | `register_biome_source("biome_source", …)`; the mod's ten biomes/layer stack not carried |
| `dimension_type` values (17 fields) | carried as values | `tools/dimension_contract.json`, Rust data-contract test, in-game readout |
| fixed time 6000, `natural`, client `effects` | deviation recorded | no SDK field; `has_fixed_time` + `day_cycle = none` |
| configured / placed features | `blocked(configured-feature-not-registered)` | 0 registrar symbols in pin surface |
| vanilla terrain / level reset | `blocked(level_reset:R11)` | 0 surface symbols + registry rule |
| structures, tags, recipes, loot, advancements, songs, custom data | not ported (later slices) | inventory above; no fake data |
| all 1,959 `assets/**` files | not ported; regenerated monogram only | `LICENSE_ASSETS.txt` ARR; `check_provenance.py` zero bytes |

## Upstream defects fixed

None found in the two dimension files this slice carries. The jar's `mods.toml`
licence claim (MIT vs the LGPL/ARR files) is a documentation inconsistency, not a
behaviour defect; the specific file wins (ARCHITECTURE.md §8).

## Verification (fresh, 2026-11-24)

Package **`the_bumblezone.hmod` 29,681 B**, SHA-256
`7ce18cc9380e6a535899cedc32b3ab0a3115a7c629a8a7fdf6178a945bc9ec9c`, 9 entries;
runtime byte-identical in the tree and package (`2964e97ba080…`). Full table,
commands and transcript in [`VALIDATION.md`](VALIDATION.md).

- Red first: `tools/carry.py --check` failed with `tools/dimension_contract.json: missing`;
  a Rust unit test then failed with `assertion left == right failed: height left: 0 right: 256`.
  Green: snapshot/jar agree on 23 keys, **9 tests passed**, clippy `-D warnings` clean,
  rustfmt clean.
- `cargo horizon-mod check`: *138 API items, Made for 0a875da*. Package check:
  *0.1.0, native-1/0a875da…, 29,681 bytes, sha 7ce18cc9…*.
- `fits --build 0a875da`: **`0a875da made for`**; `fits --build 0b14f8a`:
  **`0b14f8a fits`** (exit 0 on both — the registrar surface exists in both builds).
- **Zero jar-derived asset bytes**: `tools/check_provenance.py` hashed all 5,509
  distinct jar entries, the 9 candidate tree files (71,541 B) and all 9 package
  entries; matched: 0. The icon is the deterministic monogram
  (`c78c73abbb8239dce8824867ddbdd9790bd9183c8abfd868dc2f105e86da4834`, 2,096 B).
- `verify-sdk-package.py the_bumblezone` → **1/1**; `check-e2e-logs.py
  the_bumblezone-e2e --strict` → `logs=4 ok=4 missing-exit=0 bad-exit=0`;
  `check-skills.py --notes ports/the_bumblezone --strict` → PASS.

### In-game `ctl` transcript (headless creative, seed 20261105)

```
ctl state -> dimension minecraft:overworld, screen playing          # baseline
ctl command "execute in the_bumblezone:the_bumblezone run forceload add 8 8"
  -> "Marked chunk [0, 0] in the_bumblezone:the_bumblezone to be force loaded"
ctl command "execute in the_bumblezone:the_bumblezone run setblock 8 8 8 minecraft:glass"
  -> "Changed the block at 8, 8, 8"
ctl command "execute in the_bumblezone:the_bumblezone if block 8 8 8 minecraft:glass run say BZ_GLASS_OK"
  -> "[Player16656] BZ_GLASS_OK"
ctl chat "/the_bumblezone dimension"
  -> "the_bumblezone:the_bumblezone | generator the_bumblezone:chunk_generator |
      biome source the_bumblezone:biome_source | min_y=0 height=256 ambient_light=0.15
      coordinate_scale=4 monster_spawn_light_level=11..=11"
ctl command "execute in the_bumblezone:no_such_dimension run setblock 8 8 8 minecraft:glass"
  -> "Unknown dimension 'the_bumblezone:no_such_dimension'"
PASS / EXIT:0
```

Fail-capable legs: the same commands on the **unmodded control** end
`FAIL: the_bumblezone:the_bumblezone is unknown to this game` + `EXIT:1`;
`SELFTEST=1` (inverted readout) ends `FAIL: SELFTEST: the readout named the real ids`
+ `EXIT:1`; `BLOCK_SELFTEST=1` ends `FAIL: BLOCK_SELFTEST: the glass is present` +
`EXIT:1`. Logs: `ports/_shared/evidence/the_bumblezone-e2e/e2e{,-control,-selftest,-block-selftest}.log`.

## Next slice

**Family:** carry the mod's ten `worldgen/biome` definitions and the nine
`BzBiomeSource` layer families (blob, pillar, pollinated fields, scale, merge) so
the registered `the_bumblezone:biome_source` answers the jar's biomes instead of
the four vanilla stand-ins.

**Gap line:** this document, *"the mod's ten biome JSON files are ARR data and the
26.3 biome codec has not been measured"* and the coverage row *"`the_bumblezone:biome_source`
id — registered; the mod's ten biomes/layer stack not carried"*.

**Tier fit:** `deep` worldgen — the ids and the dimension type are already shipped;
what is missing is the data codec and the deterministic layer stack. All art must be
regenerated (zero jar-derived bytes), so the slice writes its own biome JSON from
the semantics of the ten files and proves the codec accepts it in game. If the pin
still cannot decode a mod biome, stop with `blocked(<symbol>)` naming the decoder.

**Concrete `ctl` assertion:** with the ten biomes carried, run
`ctl chat "/the_bumblezone dimension 0 8 0 20261105"` and assert the `biome ...`
line names one of the mod's ids (e.g. `the_bumblezone:hive_pillar`) rather than
`minecraft:*`; the vanilla control still answers `Unknown or incomplete command`
for the readout id, and `SELFTEST=1` (expect `minecraft:plains`) ends `EXIT:1`.
