//! The Bumblezone's pure slice-1 data contract: registered ids and the values of
//! the jar's `dimension_type`. No `mod_api` types live here — only the numbers
//! the typed glue in `lib.rs` hands to `Registrar`.
//!
//! Evidence: `tools/dimension_contract.json`, authored from
//! `data/the_bumblezone/dimension{,_type}/the_bumblezone.json` and the constant
//! pool of `BzDimension.class` by `tools/carry.py`; no jar byte is packaged.

/// The dimension id, namespaced by the registrar from the short name
/// `the_bumblezone` (the package owns that namespace).
pub const DIMENSION_ID: &str = "the_bumblezone:the_bumblezone";
/// The Java name in `data/the_bumblezone/dimension/the_bumblezone.json`.
pub const CHUNK_GENERATOR_ID: &str = "the_bumblezone:chunk_generator";
/// The Java name in that dimension's `biome_source` object.
pub const BIOME_SOURCE_ID: &str = "the_bumblezone:biome_source";
/// The `settings` id the dimension file names.
pub const NOISE_SETTINGS_ID: &str = "the_bumblezone:bz_noise_settings";

/// Biomes this slice can resolve. The mod's ten biome definitions are not
/// carried (their art is ARR and the 26.3 biome codec has not been measured), so
/// the registered source answers with vanilla biomes the world already has;
/// `possible_biomes` must only name biomes the level can resolve.
pub const RESOLVED_BIOMES: [&str; 4] = [
    "minecraft:plains",
    "minecraft:forest",
    "minecraft:taiga",
    "minecraft:desert",
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

/// The deterministic biome index a quart cell gets from the registered source.
/// The mod's layer stack is a later slice; this keeps the source stateless and
/// the answer reproducible for a seed and cell.
#[must_use]
pub fn biome_index(quart: [i32; 3], seed: i64) -> u16 {
    let mut hash = (seed as u64) ^ 0x9e37_79b9_7f4a_7c15;
    for coordinate in quart {
        hash = hash
            .wrapping_mul(0x9e37_79b9_7f4a_7c15)
            .wrapping_add((coordinate as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9));
    }
    (hash % RESOLVED_BIOMES.len() as u64) as u16
}

/// One line naming the three registered ids and the dimension_type numbers.
/// The fail-capable in-game leg asserts on this exact, parsed output.
#[must_use]
pub fn status_line() -> String {
    let v = dimension_values();
    format!(
        "{DIMENSION_ID} | generator {CHUNK_GENERATOR_ID} | biome source {BIOME_SOURCE_ID} | \
         min_y={} height={} ambient_light={} coordinate_scale={} \
         monster_spawn_light_level={}..={}",
        v.min_y,
        v.height,
        v.ambient_light,
        v.coordinate_scale,
        v.monster_spawn_light_level.0,
        v.monster_spawn_light_level.1
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
    fn biome_answers_are_deterministic_and_in_range() {
        for quart in [[0, 16, 0], [3, -4, 7], [-12, 0, 33]] {
            let first = biome_index(quart, 20_261_105);
            assert_eq!(first, biome_index(quart, 20_261_105));
            assert!(usize::from(first) < RESOLVED_BIOMES.len());
            assert_ne!(first, biome_index(quart, 20_261_106));
        }
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
