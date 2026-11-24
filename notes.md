# The Bumblezone — port notes (v0.1.1: nine biomes + the layer stack)

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

## Slice 0.1.1 — the nine biomes and the `BzBiomeSource` layer stack

**Artefact count corrected by measurement.** The 0.1.0 gap line and the 0.1.1
brief said *ten* biome JSON files and *nine* layer families. The jar actually
ships **nine** `data/the_bumblezone/worldgen/biome/*.json` files
(`bumbling_beepartments`, `crystal_canyon`, `floral_meadow`, `hive_pillar`,
`hive_wall`, `howling_constructs`, `pollinated_fields`, `pollinated_pillar`,
`sugar_water_floor`) and **seven** `BzBiome*Layer` classes (base, pillar, scale,
blob, pollinated-pillar, pollinated-fields, merge). The artefact wins over the
inherited note (AGENTS.md "a gap line is not evidence"); both counts are
recorded by `tools/biome_contract.py --check` against the jar.

**Codec gate (measured, not assumed):** `evidence/the_bumblezone/biome-codec-gate.log`.
The SDK surface has no `register_biome` / `BiomeDefinition` decoder — only
`BiomeSource::possible_biomes`/`noise_biome`, biome *modifiers* and biome
*regions* (which require the biomes to already exist). `declare_datapack_registry`
refuses a vanilla registry key. The game's 26.3 biome decoder is
`horizon-client-render/src/biome.rs::BiomeInfo::from_json`, applied to data-pack
files by `Biomes::with_packs`; it is lenient (defaults for absent fields) and
adds new ids. The mod content loader reads `data/<ns>/worldgen/biome/*.json`.
So the codec **accepts** a mod biome definition: the nine are carried as
authored 26.3-shape JSON (`tools/biome_contract.json` values → `tools/gen_biomes.py`),
with zero jar-derived bytes. One shape trap was found by the gate and fixed: a
biome's `carvers` must be a **list** (`[]`); an empty object is decoded as one
carver missing its `type` and the world refuses to open. The generator emits
`[]`, and the full failed log is named in `VALIDATION.md`. The worldgen generator
still logs that new biomes are "left out" of terrain generation at this pin; the
fail-capable proof is the source's own readout, not shaped terrain.

**Carry:** `src/runtime.rs` declares the nine ids in the `the_bumblezone.json`
tag's registry order and composes the seven layer families as a pure,
deterministic `biome_index(quart, seed)` using the jar's biome tags. Mob
spawners, carvers and configured features are intentionally empty: the
configured-feature registrar is absent at this pin. Every art asset remains
regenerated (`check_provenance.py` reports **0** jar-derived bytes across 5,509
jar hashes and now 18 tree candidates).

**Red first:** `evidence/the_bumblezone/unit-test-red-011.log` ends with
`the_source_answers_the_jars_biomes ... FAILED`, `left: ["minecraft:plains",
"minecraft:forest", "minecraft:taiga", "minecraft:desert"]`, right: the nine
mod ids; `test result: FAILED. 9 passed; 1 failed`. Green is 12/12 in
`sdk-test.log`.

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
- **Deep-slice gaps (not faked):** the Java `BzChunkGenerator` terrain layers are
  not ported, so the generator still leaves the chunk as air (the level opens and
  accepts blocks, proved in 0.1.0). Slice 0.1.1 **does** carry the nine biomes and
  composes the seven layer families, so the source no longer answers four vanilla
  stand-ins. Not carried: the biome files' mob spawner lists, carvers and the
  configured features they reference (`configured-feature-not-registered`), and
  the client `effects` id (no `DimensionSettings` field). The terrain itself is
  still air, so the carried biome selection is proved through the source's own
  readout rather than through shaped terrain.
- **Not in scope of the artefact's first slice:** 906 structures, 548 tags, 204
  recipes, 173 loot tables, 162 advancements, songs, patterns and custom data
  families are recorded in the inventory above for later slices.

## Coverage table

| Family in the jar | State | Evidence / reason |
|---|---|---|
| `the_bumblezone:the_bumblezone` dimension | registered | `src/lib.rs::register` → `Registrar::register_dimension("the_bumblezone", settings())`; in-game readout + `execute in` |
| `the_bumblezone:chunk_generator` id | registered | `register_chunk_generator("chunk_generator", …)`; terrain shaping not ported (deep slice) |
| `the_bumblezone:biome_source` id | registered | `register_biome_source("biome_source", …)`; answers the nine carried biomes (`runtime::BIOMES`) through the seven-layer stack |
| nine `worldgen/biome` definitions | carried (authored) | `tools/biome_contract.json` → `tools/gen_biomes.py`; 9 files, 6,705 B, zero jar-derived bytes; `gen_biomes.py --check` |
| seven `BzBiomeSource` layer families | carried (pure logic) | `runtime::LAYER_FAMILIES` + `biome_index`; coverage test reaches all nine ids; probed cell readout in game |
| biome spawners / carvers / features | `blocked(configured-feature-not-registered)` | pin surface has 0 configured-feature symbols; empty in the authored JSON |
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

## Verification (fresh, 2026-11-24, version 0.1.1)

Package **`the_bumblezone.hmod` 37,299 B**, SHA-256
`d9b8f63c7d5438fe6b719af08efa5eb33aab87d34e1801c910e360385a046907`, 18 entries;
runtime byte-identical in the tree and package (`1d5ae47cd8d9…`). Full table,
commands and transcript in [`VALIDATION.md`](VALIDATION.md).

- Red first: `the_source_answers_the_jars_biomes` failed with
  `left: ["minecraft:plains", "minecraft:forest", "minecraft:taiga", "minecraft:desert"]`,
  right: the nine mod ids (`unit-test-red-011.log`). Green: **12 tests passed**,
  clippy `-D warnings` clean, rustfmt clean; contract tools green (23 dimension
  keys, 9 biomes / 8 tags / 7 families, 9 generated files).
- `cargo horizon-mod check`: *142 API items, Made for 0a875da*. Package check:
  *0.1.1, native-1/0a875da…, 37,299 bytes, sha d9b8f63c…*.
- `fits --build 0a875da`: **`0a875da made for`**; `fits --build 0b14f8a`:
  **`0b14f8a fits`** (exit 0 on both).
- **Zero jar-derived asset bytes**: `tools/check_provenance.py` hashed all 5,509
  distinct jar entries, the 18 candidate tree files (86,544 B) and all 18 package
  entries; matched: 0. The icon is the deterministic monogram
  (`c78c73abbb8239dce8824867ddbdd9790bd9183c8abfd868dc2f105e86da4834`, 2,096 B).
- `verify-sdk-package.py the_bumblezone` → **1/1** (including *package matches the
  tree*); `check-e2e-logs.py the_bumblezone-e2e --strict` →
  `logs=4 ok=4 missing-exit=0 bad-exit=0`;
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
      coordinate_scale=4 monster_spawn_light_level=11..=11 | layers 7"
ctl chat "/the_bumblezone dimension 0 8 0 20261105"
  -> "... | layers 7 | biome the_bumblezone:sugar_water_floor (4)"
ctl command "execute in the_bumblezone:no_such_dimension run setblock 8 8 8 minecraft:glass"
  -> "Unknown dimension 'the_bumblezone:no_such_dimension'"
PASS / EXIT:0
```

Fail-capable legs: the same commands on the **unmodded control** end
`FAIL: the_bumblezone:the_bumblezone is unknown to this game` + `EXIT:1`;
`SELFTEST=1` (inverted readout) ends `FAIL: SELFTEST: the readout named the real ids`
+ `EXIT:1`; `BLOCK_SELFTEST=1` ends `FAIL: BLOCK_SELFTEST: the glass is present` +
`EXIT:1`. Logs: `ports/_shared/evidence/the_bumblezone-e2e/e2e{,-control,-selftest,-block-selftest}.log`.

## Next slice (proposed after 0.1.1)

**Family:** the nine biomes' **mob spawner lists** — translate each
`data/the_bumblezone/worldgen/biome/*.json` `spawners` object into the 26.3
`minecraft:gameplay/natural_mob_spawns` `spawns_by_category` shape (currently
authored empty in `tools/gen_biomes.py`), regenerated from values with zero jar
bytes. `the_bumblezone:biome_source` and the nine biomes are already shipped, so
this is the next honest gap in the same family.

**Gap line:** this document, *"Not carried: the biome files' mob spawner lists …
(the configured-feature registrar is absent at this pin)"* and the coverage row
*`biome spawners / carvers / features | blocked(...)`*.

**Tier fit:** `deep` worldgen. The spawner data is a value family in the carried
biome JSON; no new registrar is needed. It must be authored from the nine files'
semantics (mob id, category, weight, min/max count), never copied as bytes, and
the zero-byte provenance proof must stay green.

**Concrete `ctl` assertion:** add a pure-Rust readout
`/the_bumblezone spawners <biome>` (e.g. `hive_pillar`) naming the parsed spawn
categories and total weight from the authored contract; assert in game that
`ctl chat "/the_bumblezone spawners the_bumblezone:hive_pillar"` prints e.g.
`monster weight=… creature weight=…`, that the unmodded control answers
`Unknown or incomplete command`, and that `SELFTEST=1` (expecting an empty
vanilla spawner list) ends `EXIT:1`. Keep the five 0.1.1 e2e legs green.
