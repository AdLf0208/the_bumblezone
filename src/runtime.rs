//! The Bumblezone's pure slice-1 data contract: registered ids and the values of
//! the jar's `dimension_type`. No `mod_api` types live here — only the numbers
//! the typed glue in `lib.rs` hands to `Registrar`.
//!
//! Evidence: `tools/dimension_contract.json`, authored from
//! `data/the_bumblezone/dimension{,_type}/the_bumblezone.json` and the constant
//! pool of `BzDimension.class` by `tools/carry.py`; no jar byte is packaged.
//!
//! Slice 0.1.2 adds the nine biome files' mob spawner lists as compiled values
//! (`src/spawner_data.rs`, generated from `tools/spawner_contract.json`) and a
//! parsed readout. The 26.3 game decodes them from the authored biome JSON.

#[path = "spawner_data.rs"]
mod spawner_data;

/// The dimension id, namespaced by the registrar from the short name
/// `the_bumblezone` (the package owns that namespace).
pub const DIMENSION_ID: &str = "the_bumblezone:the_bumblezone";
/// The Java name in `data/the_bumblezone/dimension/the_bumblezone.json`.
pub const CHUNK_GENERATOR_ID: &str = "the_bumblezone:chunk_generator";
/// The Java name in that dimension's `biome_source` object.
pub const BIOME_SOURCE_ID: &str = "the_bumblezone:biome_source";
/// The `settings` id the dimension file names.
pub const NOISE_SETTINGS_ID: &str = "the_bumblezone:bz_noise_settings";

/// The nine biomes of the artefact's `data/the_bumblezone/worldgen/biome/`,
/// in the order of its `tags/worldgen/biome/the_bumblezone.json`. They are
/// authored as 26.3-shape JSON by `tools/gen_biomes.py` from the values in
/// `tools/biome_contract.json` (no jar byte is carried). The registered source
/// answers these instead of four vanilla stand-ins.
pub const BIOMES: [&str; 9] = [
    "the_bumblezone:hive_pillar",
    "the_bumblezone:hive_wall",
    "the_bumblezone:pollinated_fields",
    "the_bumblezone:pollinated_pillar",
    "the_bumblezone:sugar_water_floor",
    "the_bumblezone:crystal_canyon",
    "the_bumblezone:floral_meadow",
    "the_bumblezone:howling_constructs",
    "the_bumblezone:bumbling_beepartments",
];

/// The seven `BzBiomeSource` layer classes, named from the artefact's
/// `BzBiomeSource.class` constant pool (read as strings, no decompilation).
/// The deterministic selector below composes their behaviour in the same
/// order: base → pillar → scale → blob → pollinated pillar → pollinated
/// fields → merge. It is a clean-room behavioural shape, not a Java port.
pub const LAYER_FAMILIES: [&str; 7] = [
    "BzBiomeLayer",
    "BzBiomePillarLayer",
    "BzBiomeScaleLayer",
    "BzBiomeBlobLayer",
    "BzBiomePollinatedPillarLayer",
    "BzBiomePollinatedFieldsLayer",
    "BzBiomeMergeLayer",
];

/// One spawn entry from a biome's `spawners` object (jar values, authored as
/// data). `min_count`/`max_count` become the 26.3 range count
/// `{"min_inclusive":…, "max_inclusive":…}`, so neither bound is lost.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpawnEntry {
    pub entity: &'static str,
    pub weight: u32,
    pub min_count: u32,
    pub max_count: u32,
}

/// One biome's non-empty categories: `(category, entries)`.
pub type SpawnCategories = [(&'static str, &'static [SpawnEntry])];
/// The compiled value contract: `(biome id, its categories)` for nine biomes.
pub type SpawnTable = [(&'static str, &'static SpawnCategories)];

/// The eight spawn categories of the 26.3 `natural_mob_spawns` attribute, in
/// the artefact's own key order.
pub const SPAWN_CATEGORIES: [&str; 8] = [
    "ambient",
    "axolotls",
    "creature",
    "misc",
    "monster",
    "underground_water_creature",
    "water_ambient",
    "water_creature",
];

/// The jar's `dimension_type` values, field for field where the SDK has a field.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use]
pub struct DimensionValues {
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub ambient_light: f32,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub coordinate_scale: f64,
    pub monster_spawn_block_light_limit: i32,
    pub monster_spawn_light_level: (i32, i32),
    pub fixed_time: i32,
    pub can_start_raid: bool,
    pub respawn_anchor_works: bool,
    pub piglins_zombify: bool,
    pub bed_works: bool,
    pub ultrawarm: bool,
}

/// The values transcribed from `data/the_bumblezone/dimension_type/the_bumblezone.json`.
/// The jar's `has_raids: false` and `piglin_safe: true` are stored as their SDK
/// inverses (`can_start_raid`, `piglins_zombify`); the unit test asserts both.
pub fn dimension_values() -> DimensionValues {
    DimensionValues {
        min_y: 0,
        height: 256,
        logical_height: 256,
        ambient_light: 0.15,
        has_skylight: false,
        has_ceiling: true,
        coordinate_scale: 4.0,
        monster_spawn_block_light_limit: 12,
        monster_spawn_light_level: (11, 11),
        fixed_time: 6000,
        can_start_raid: false,
        respawn_anchor_works: false,
        piglins_zombify: false,
        bed_works: true,
        ultrawarm: false,
    }
}

// Index groups the artefact's biome tags select from (see biome_contract.json).
const PILLARS: [u16; 2] = [0, 3]; // tags/worldgen/biome/pillars.json
const BLOBS: [u16; 3] = [5, 0, 1]; // blob_biomes.json + the hive_wall filler
const RARE_BLOBS: [u16; 3] = [6, 7, 8]; // rare_blob_biomes.json
const POLLINATED: [u16; 2] = [3, 2]; // tags/worldgen/biome/pollinated.json
const SUGAR_WATER: u16 = 4; // tags/worldgen/biome/sugar_water.json

// Independent salts keep each layer's random field uncorrelated. Values are
// arbitrary odd 64-bit constants; only their distinctness matters.
const SALT_BASE: u64 = 0x9E37_79B9_7F4A_7C15;
const SALT_PILLAR: u64 = 0xBF58_476D_1CE4_E5B9;
const SALT_BLOB: u64 = 0x94D0_49BB_1331_11EB;
const SALT_FIELD: u64 = 0xD1B5_4A32_D192_ED03;
const SALT_SUGAR: u64 = 0x27D4_EB2F_165A_4507;
const SALT_RARE: u64 = 0x5851_F42D_4C95_7F2D;

/// A stable hash of one layer's cell and seed. SplitMix-style mixing gives a
/// well-distributed field without any dependency (R9 forbids one).
fn cell_hash(cell: [i32; 3], seed: i64, salt: u64) -> u64 {
    let mut hash = (seed as u64) ^ salt;
    for (axis, coordinate) in cell.iter().enumerate() {
        hash = hash.wrapping_mul(SALT_BASE)
            ^ ((*coordinate as u64)
                .wrapping_add(axis as u64)
                .wrapping_mul(SALT_PILLAR));
    }
    // SplitMix64 finalizer: avalanches the low bits so `% 100` and `% group`
    // are uniform for neighbouring cells instead of inheriting their pattern.
    hash ^= hash >> 30;
    hash = hash.wrapping_mul(SALT_PILLAR);
    hash ^= hash >> 27;
    hash = hash.wrapping_mul(SALT_BLOB);
    hash ^ (hash >> 31)
}

/// Pick one index from a layer's tag group, deterministic for a cell and seed.
fn pick(group: &[u16], quart: [i32; 3], seed: i64, salt: u64) -> u16 {
    group[usize::try_from(cell_hash(quart, seed, salt) % group.len() as u64)
        .expect("a modulo by a non-empty group fits usize")]
}

/// The biome index a quart cell gets from the registered source. Each branch
/// is one `BzBiomeSource` layer family, composed in the jar's order and settled
/// by the merge layer's priority. Pure and stateless, so a seed and cell always
/// answer the same biome; no game handle reaches this function.
#[must_use]
pub fn biome_index(quart: [i32; 3], seed: i64) -> u16 {
    let [x, y, z] = quart;
    // BzBiomePollinatedFieldsLayer: broad, flat fields.
    if cell_hash([x >> 5, 0, z >> 5], seed, SALT_FIELD) % 100 < 22 {
        return POLLINATED[1];
    }
    // BzBiomeBlobLayer: its rare blobs are a sparser, larger overlay.
    if cell_hash([x >> 4, y >> 3, z >> 4], seed, SALT_RARE) % 100 < 12 {
        return pick(&RARE_BLOBS, quart, seed, SALT_RARE ^ SALT_BASE);
    }
    let blob = cell_hash([x >> 3, y >> 2, z >> 3], seed, SALT_BLOB);
    if blob % 100 < 24 {
        return pick(&BLOBS, quart, seed, SALT_PILLAR);
    }
    // BzBiomePillarLayer, zoomed by BzBiomeScaleLayer; pollinated pillars split off.
    let pillar = cell_hash([x >> 2, y >> 3, z >> 2], seed, SALT_PILLAR);
    if pillar % 100 < 32 {
        return if pillar.is_multiple_of(3) {
            pick(&POLLINATED, quart, seed, SALT_FIELD)
        } else {
            pick(&PILLARS, quart, seed, SALT_BLOB)
        };
    }
    // Sugar-water floor, low in the column.
    if cell_hash([x >> 2, y, z >> 2], seed, SALT_SUGAR) % 100 < 18 {
        return SUGAR_WATER;
    }
    // BzBiomeLayer base: the dimension's main biome dominates.
    PILLARS[0]
}

/// The spawner readout for one biome, from the compiled value contract. Each
/// non-empty category prints its name, summed weight and entry count. The
/// fail-capable in-game leg parses those numbers from the game-side reply.
#[must_use]
pub fn spawners_line(biome: &str) -> String {
    let Some((_, categories)) = spawner_data::SPAWNERS.iter().find(|(id, _)| *id == biome) else {
        return format!("{biome} spawns: unknown biome");
    };
    let mut line = format!("{biome} spawns:");
    for category in SPAWN_CATEGORIES {
        if let Some((_, entries)) = categories.iter().find(|(name, _)| *name == category) {
            let weight: u32 = entries.iter().map(|entry| entry.weight).sum();
            line.push_str(&format!(
                " {category} weight={weight} ({} entries)",
                entries.len()
            ));
        }
    }
    line
}

/// One line naming the three registered ids and the dimension_type numbers.
/// The fail-capable in-game leg asserts on this exact, parsed output.
#[must_use]
pub fn status_line() -> String {
    let v = dimension_values();
    format!(
        "{DIMENSION_ID} | generator {CHUNK_GENERATOR_ID} | biome source {BIOME_SOURCE_ID} | \
         min_y={} height={} ambient_light={} coordinate_scale={} \
         monster_spawn_light_level={}..={} | layers {}",
        v.min_y,
        v.height,
        v.ambient_light,
        v.coordinate_scale,
        v.monster_spawn_light_level.0,
        v.monster_spawn_light_level.1,
        LAYER_FAMILIES.len()
    )
}

/// The checked-in contract, authored by `tools/carry.py` from the jar. Embedded
/// in the source because `tools/` is not part of the delivered package, and
/// cross-checked against both the jar and `tools/dimension_contract.json` by
/// `tools/carry.py --check`.
#[cfg(test)]
const CONTRACT_JSON: &str = r##"{
  "ambient_light": 0.15,
  "bed_works": true,
  "biome_source": "the_bumblezone:biome_source",
  "chunk_generator": "the_bumblezone:chunk_generator",
  "coordinate_scale": 4.0,
  "dimension": "the_bumblezone:the_bumblezone",
  "fixed_time": 6000,
  "has_ceiling": true,
  "has_raids": false,
  "has_skylight": false,
  "height": 256,
  "infiniburn": "#minecraft:infiniburn_overworld",
  "jar_sha256": "859d87b44d3d17c63f38317ac34d124e30b6de8c3043fa61ee987ff8b1f236d6",
  "logical_height": 256,
  "main_biomes": "#the_bumblezone:the_bumblezone",
  "min_y": 0,
  "monster_spawn_block_light_limit": 12,
  "monster_spawn_light_level": 11,
  "natural": true,
  "noise_settings": "the_bumblezone:bz_noise_settings",
  "piglin_safe": true,
  "respawn_anchor_works": false,
  "ultrawarm": false
}"##;

#[cfg(test)]
mod tests {
    use super::*;

    /// The embedded snapshot is the jar's values, authored by `tools/carry.py`.
    fn contract() -> serde_json_like::Values {
        serde_json_like::parse_dimension(CONTRACT_JSON)
    }

    #[test]
    fn embedded_contract_matches_the_checked_in_snapshot() {
        let expected = include_str!("../tools/dimension_contract.json");
        let canon = |text: &str| -> String {
            text.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n")
        };
        // The snapshot carries one extra provenance note; compare its data lines only.
        let expected: Vec<&str> = expected
            .lines()
            .filter(|line| !line.contains("\"note\""))
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();
        let canonical = canon(CONTRACT_JSON);
        let actual: Vec<&str> = canonical.lines().collect();
        assert_eq!(
            actual, expected,
            "embedded contract drifted from tools/dimension_contract.json"
        );
    }

    /// RED FIRST: every field is zero in the stub, and the assertion names the
    /// first field that differs from the artefact.
    #[test]
    fn dimension_values_match_the_jar() {
        let want = contract();
        let got = dimension_values();
        assert_eq!(got.min_y, want.min_y, "min_y");
        assert_eq!(got.height, want.height, "height");
        assert_eq!(got.logical_height, want.logical_height, "logical_height");
        assert_eq!(got.ambient_light, want.ambient_light, "ambient_light");
        assert_eq!(got.has_skylight, want.has_skylight, "has_skylight");
        assert_eq!(got.has_ceiling, want.has_ceiling, "has_ceiling");
        assert_eq!(
            got.coordinate_scale, want.coordinate_scale,
            "coordinate_scale"
        );
        assert_eq!(
            got.monster_spawn_block_light_limit, want.monster_spawn_block_light_limit,
            "monster_spawn_block_light_limit"
        );
        assert_eq!(
            got.monster_spawn_light_level,
            (
                want.monster_spawn_light_level,
                want.monster_spawn_light_level
            ),
            "monster_spawn_light_level"
        );
        assert_eq!(got.fixed_time, want.fixed_time, "fixed_time");
        assert_eq!(
            got.can_start_raid, want.has_raids,
            "has_raids -> can_start_raid"
        );
        assert_eq!(
            got.respawn_anchor_works, want.respawn_anchor_works,
            "respawn_anchor_works"
        );
        assert_eq!(
            got.piglins_zombify, !want.piglin_safe,
            "piglin_safe -> piglins_zombify"
        );
        assert_eq!(got.bed_works, want.bed_works, "bed_works");
        assert_eq!(got.ultrawarm, want.ultrawarm, "ultrawarm");
    }

    #[test]
    fn the_registered_ids_are_the_jars() {
        let want = contract();
        assert_eq!(DIMENSION_ID, want.dimension);
        assert_eq!(CHUNK_GENERATOR_ID, want.chunk_generator);
        assert_eq!(BIOME_SOURCE_ID, want.biome_source);
        assert_eq!(NOISE_SETTINGS_ID, want.noise_settings);
    }

    #[test]
    fn the_readout_names_all_three_registered_ids() {
        let line = status_line();
        assert!(line.starts_with(DIMENSION_ID), "{line}");
        assert!(line.contains(CHUNK_GENERATOR_ID), "{line}");
        assert!(line.contains(BIOME_SOURCE_ID), "{line}");
    }

    #[test]
    fn biome_answers_are_deterministic_and_seed_sensitive() {
        for quart in [[0, 16, 0], [3, -4, 7], [-12, 0, 33]] {
            let first = biome_index(quart, 20_261_105);
            assert_eq!(first, biome_index(quart, 20_261_105));
            assert!(usize::from(first) < BIOMES.len());
        }
        // The seed must matter somewhere in the stack, but a given cell can
        // legitimately land in the same base biome for two nearby seeds.
        let mut differs = false;
        for x in -64..64 {
            for z in -64..64 {
                if biome_index([x, 2, z], 20_261_105) != biome_index([x, 2, z], 20_261_106) {
                    differs = true;
                }
            }
        }
        assert!(differs, "changing the seed must change some biome");
    }

    /// Every layer family's tag group is reachable over a representative
    /// slice of the dimension, and every answer is one of the nine ids.
    #[test]
    fn the_layer_stack_reaches_all_nine_biomes() {
        let mut seen = [false; 9];
        // Wide enough for the largest region scale (`x >> 5`): a small grid
        // only sees two region cells and cannot exercise the rare blobs.
        for x in (-128..128).step_by(4) {
            for z in (-128..128).step_by(4) {
                for y in (0..32).step_by(2) {
                    let index = biome_index([x, y, z], 20_261_105);
                    assert!(
                        usize::from(index) < 9,
                        "index {index} outside the nine biomes"
                    );
                    seen[usize::from(index)] = true;
                }
            }
        }
        assert_eq!(
            seen.iter().filter(|seen| **seen).count(),
            9,
            "the layer stack must reach every biome: {seen:?}"
        );
        // The layer families named by the artefact are all declared.
        assert_eq!(LAYER_FAMILIES.len(), 7);
    }

    /// RED FIRST (slice 0.1.1): the registered source must answer the jar's
    /// `worldgen/biome` set, not four vanilla stand-ins. The nine ids are the
    /// artefact's `data/the_bumblezone/worldgen/biome/*.json` files, in the
    /// order of its `tags/worldgen/biome/the_bumblezone.json`.
    #[test]
    fn the_source_answers_the_jars_biomes() {
        let want = [
            "the_bumblezone:hive_pillar",
            "the_bumblezone:hive_wall",
            "the_bumblezone:pollinated_fields",
            "the_bumblezone:pollinated_pillar",
            "the_bumblezone:sugar_water_floor",
            "the_bumblezone:crystal_canyon",
            "the_bumblezone:floral_meadow",
            "the_bumblezone:howling_constructs",
            "the_bumblezone:bumbling_beepartments",
        ];
        assert_eq!(
            BIOMES.as_slice(),
            &want,
            "the biome source must answer the jar's nine biomes, not vanilla stand-ins"
        );
    }

    /// RED FIRST (slice 0.1.2): the nine biome files' spawner lists must be
    /// carried as values. Each pair is the sum of the jar's `spawners` weights
    /// for `creature` and `monster` (read by tools/spawner_contract.py). The
    /// empty `bumbling_beepartments` monster list has no `monster` line.
    #[test]
    fn the_spawner_contract_matches_the_jars_biomes() {
        let want: [(&str, u32, u32); 9] = [
            ("the_bumblezone:hive_pillar", 80, 26),
            ("the_bumblezone:hive_wall", 70, 34),
            ("the_bumblezone:pollinated_fields", 100, 43),
            ("the_bumblezone:pollinated_pillar", 100, 44),
            ("the_bumblezone:sugar_water_floor", 80, 23),
            ("the_bumblezone:crystal_canyon", 89, 23),
            ("the_bumblezone:floral_meadow", 85, 28),
            ("the_bumblezone:howling_constructs", 99, 20),
            ("the_bumblezone:bumbling_beepartments", 60, 0),
        ];
        for (id, creature, monster) in want {
            let line = spawners_line(id);
            assert!(
                line.contains(&format!("creature weight={creature}")),
                "{id}: expected creature weight={creature}, got {line}"
            );
            if monster > 0 {
                assert!(
                    line.contains(&format!("monster weight={monster}")),
                    "{id}: expected monster weight={monster}, got {line}"
                );
            }
        }
    }

    /// The lossless part of the translation: the jar's `minCount`/`maxCount`
    /// survive as a 26.3 range, not collapsed to one number.
    #[test]
    fn spawner_entries_preserve_the_jars_ranges() {
        let (_, categories) = spawner_data::SPAWNERS
            .iter()
            .find(|(id, _)| *id == "the_bumblezone:hive_pillar")
            .expect("hive_pillar spawners");
        let (_, creature) = categories
            .iter()
            .find(|(name, _)| *name == "creature")
            .expect("hive_pillar creature spawns");
        assert_eq!(
            creature[0],
            SpawnEntry {
                entity: "minecraft:bee",
                weight: 50,
                min_count: 4,
                max_count: 8,
            },
            "the bee entry keeps its weight and both count bounds"
        );
    }
}

/// A dependency-free reader for the six values the tests need from the
/// checked-in snapshot (R9 allows no serde in the package).
#[cfg(test)]
mod serde_json_like {
    #[derive(Debug)]
    pub struct Values {
        pub min_y: i32,
        pub height: i32,
        pub logical_height: i32,
        pub ambient_light: f32,
        pub has_skylight: bool,
        pub has_ceiling: bool,
        pub coordinate_scale: f64,
        pub monster_spawn_block_light_limit: i32,
        pub monster_spawn_light_level: i32,
        pub fixed_time: i32,
        pub has_raids: bool,
        pub respawn_anchor_works: bool,
        pub piglin_safe: bool,
        pub bed_works: bool,
        pub ultrawarm: bool,
        pub dimension: String,
        pub chunk_generator: String,
        pub biome_source: String,
        pub noise_settings: String,
    }

    pub fn parse_dimension(text: &str) -> Values {
        let number = |key: &str| -> f64 {
            let needle = format!("\"{key}\": ");
            let rest = text
                .split_once(&needle)
                .unwrap_or_else(|| panic!("no {key}"))
                .1;
            rest.split([',', '\n', '}'])
                .next()
                .unwrap_or_default()
                .trim()
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("{key} is not a number"))
        };
        let boolean = |key: &str| -> bool {
            let needle = format!("\"{key}\": ");
            let rest = text
                .split_once(&needle)
                .unwrap_or_else(|| panic!("no {key}"))
                .1;
            rest.starts_with("true")
        };
        let string = |key: &str| -> String {
            let needle = format!("\"{key}\": \"");
            let rest = text
                .split_once(&needle)
                .unwrap_or_else(|| panic!("no {key}"))
                .1;
            rest.split_once('"')
                .unwrap_or_else(|| panic!("{key} unterminated"))
                .0
                .to_owned()
        };
        Values {
            min_y: number("min_y") as i32,
            height: number("height") as i32,
            logical_height: number("logical_height") as i32,
            ambient_light: number("ambient_light") as f32,
            has_skylight: boolean("has_skylight"),
            has_ceiling: boolean("has_ceiling"),
            coordinate_scale: number("coordinate_scale"),
            monster_spawn_block_light_limit: number("monster_spawn_block_light_limit") as i32,
            monster_spawn_light_level: number("monster_spawn_light_level") as i32,
            fixed_time: number("fixed_time") as i32,
            has_raids: boolean("has_raids"),
            respawn_anchor_works: boolean("respawn_anchor_works"),
            piglin_safe: boolean("piglin_safe"),
            bed_works: boolean("bed_works"),
            ultrawarm: boolean("ultrawarm"),
            dimension: string("dimension"),
            chunk_generator: string("chunk_generator"),
            biome_source: string("biome_source"),
            noise_settings: string("noise_settings"),
        }
    }
}
