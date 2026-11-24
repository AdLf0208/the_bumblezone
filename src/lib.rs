//! The Bumblezone for Horizon 26.3 — the one `impl Hooks` at the crate root.
//!
//! Slice 1 registers the dimension the jar declares (`the_bumblezone:the_bumblezone`)
//! and its two Java-named ids (`the_bumblezone:chunk_generator`,
//! `the_bumblezone:biome_source`) with the jar's `dimension_type` values. All pure
//! values, deterministic answers and the readout text live in [`runtime`]; this
//! file is only typed glue to the SDK.
//!
//! Explicit gaps (measured, never faked): the Java terrain shape and the mod's
//! ten biome definitions are a later deep slice, and configured features cannot
//! be registered at this pin. `notes.md` records both with their symbols.
use mod_api::{
    self, BiomeSource, ChunkAccess, ChunkGenerator, Command, CommandArgumentValue,
    DimensionSettings, Flow, Hooks, RegisterCommandsEvent, Registrar, RegistrationContext,
    RunModCommandEvent, ServerHost, Text, VanillaStages,
};

mod runtime;

/// The mod's biome source id, whose `possible_biomes` must name biomes the
/// level can resolve. Until the mod's ten biome definitions are carried, the
/// source answers with vanilla biomes (`runtime::RESOLVED_BIOMES`).
#[derive(Default)]
struct BzBiomeSource;

impl BiomeSource for BzBiomeSource {
    fn possible_biomes(&self) -> Vec<String> {
        runtime::RESOLVED_BIOMES
            .iter()
            .map(|id| (*id).to_owned())
            .collect()
    }

    fn noise_biome(&self, quart: [i32; 3], seed: i64) -> u16 {
        runtime::biome_index(quart, seed)
    }
}

/// The registered generator. The jar's `BzChunkGenerator` builds the Bumblezone's
/// honeycomb terrain; that terrain is a deep slice (no density-function types or
/// Java layer stack at this pin), so this generator leaves the chunk as air.
/// The level still exists, opens and accepts blocks — which is what the readout
/// and the in-game leg prove. `notes.md` records the gap.
struct BzChunkGenerator;

impl ChunkGenerator for BzChunkGenerator {
    fn build_terrain(&self, _chunk: &mut dyn ChunkAccess) {}

    fn vanilla_stages(&self) -> VanillaStages {
        VanillaStages::default()
    }
}

/// The jar's `dimension_type`, field for field where the SDK has a field.
///
/// The jar's `fixed_time: 6000` has no value field here: `has_fixed_time` plus
/// `day_cycle: "none"` keeps the clock still (the exact 6000 is recorded in
/// `notes.md`). `effects`, `natural` and the `main_biomes`/`blob_biomes` tags
/// have no counterpart on `DimensionSettings`.
fn settings() -> DimensionSettings {
    let v = runtime::dimension_values();
    DimensionSettings {
        min_y: v.min_y,
        height: v.height,
        logical_height: v.logical_height,
        has_skylight: v.has_skylight,
        has_ceiling: v.has_ceiling,
        ambient_light: v.ambient_light,
        coordinate_scale: v.coordinate_scale,
        monster_spawn_block_light_limit: v.monster_spawn_block_light_limit,
        monster_spawn_light_level: v.monster_spawn_light_level,
        can_start_raid: v.can_start_raid,
        respawn_anchor_works: v.respawn_anchor_works,
        piglins_zombify: v.piglins_zombify,
        has_fixed_time: v.fixed_time > 0,
        day_cycle: "none".to_owned(),
        // `bed_works: true`: sleep at night and set the respawn point, as the Overworld.
        bed_can_sleep: "when_dark".to_owned(),
        bed_can_set_spawn: "always".to_owned(),
        water_evaporates: v.ultrawarm,
        generator: runtime::CHUNK_GENERATOR_ID.to_owned(),
        noise_settings: runtime::NOISE_SETTINGS_ID.to_owned(),
        biome: "minecraft:plains".to_owned(),
        mod_biome_source: Some(runtime::BIOME_SOURCE_ID.to_owned()),
        infiniburn: "#minecraft:infiniburn_overworld".to_owned(),
        ..DimensionSettings::default()
    }
}

/// Once at startup: the source and generator first, then the level that names
/// both. The short names are namespaced by the registrar to the jar's ids.
pub fn register(r: &mut dyn Registrar) {
    r.register_biome_source("biome_source", BzBiomeSource);
    r.register_chunk_generator("chunk_generator", BzChunkGenerator);
    r.register_dimension("the_bumblezone", settings());
}

/// Two usages, with no optional `[…]` group: the engine drops a usage whose
/// arguments are optional, so a single `[x]` form would never be in the tree.
pub fn register_commands(event: &mut RegisterCommandsEvent<'_>) {
    event
        .commands
        .push(Command::new("the_bumblezone dimension"));
    event.commands.push(Command::new(
        "the_bumblezone dimension <x:integer> <y:integer> <z:integer> <seed:integer>",
    ));
}

/// Answer only our own `dimension` usage; let every other command through.
pub fn run_command(host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
    const NAME: &str = "the_bumblezone";
    if event.command.literals.first().map(String::as_str) != Some(NAME)
        || event.command.literals.get(1).map(String::as_str) != Some("dimension")
    {
        return Flow::Continue;
    }
    let mut line = runtime::status_line();
    if let (
        Some(CommandArgumentValue::Integer(x)),
        Some(CommandArgumentValue::Integer(y)),
        Some(CommandArgumentValue::Integer(z)),
        Some(CommandArgumentValue::Integer(seed)),
    ) = (
        event.command.argument("x"),
        event.command.argument("y"),
        event.command.argument("z"),
        event.command.argument("seed"),
    ) {
        let index = runtime::biome_index([*x >> 2, *y >> 2, *z >> 2], i64::from(*seed));
        line.push_str(&format!(
            " | biome {} ({index})",
            runtime::RESOLVED_BIOMES[usize::from(index)]
        ));
    }
    let _ = host.commands().feedback(Text::literal(line));
    Flow::Cancel
}

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _context: &RegistrationContext) {
        register(r);
    }

    fn register_commands(&self, _host: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        register_commands(event);
    }

    fn run_mod_command(
        &self,
        host: &mut dyn ServerHost,
        event: &mut RunModCommandEvent<'_>,
    ) -> Flow {
        run_command(host, event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_registered_settings_pass_the_sdks_own_validation() {
        let actual = settings();
        assert_eq!(actual.invalid_field(), None);
        assert_eq!(actual.invalid_biome_source(), None);
    }

    #[test]
    fn the_settings_are_the_jars_dimension_type() {
        let actual = settings();
        let v = runtime::dimension_values();
        assert_eq!(actual.min_y, 0);
        assert_eq!(actual.height, 256);
        assert_eq!(actual.logical_height, 256);
        assert!(!actual.has_skylight);
        assert!(actual.has_ceiling);
        assert_eq!(actual.ambient_light, v.ambient_light);
        assert_eq!(actual.coordinate_scale, 4.0);
        assert_eq!(actual.monster_spawn_light_level, (11, 11));
        assert_eq!(actual.monster_spawn_block_light_limit, 12);
        assert!(!actual.can_start_raid);
        assert!(!actual.respawn_anchor_works);
        assert!(!actual.piglins_zombify);
        assert!(actual.has_fixed_time);
        assert_eq!(actual.generator, runtime::CHUNK_GENERATOR_ID);
        assert_eq!(actual.noise_settings, runtime::NOISE_SETTINGS_ID);
        assert_eq!(
            actual.mod_biome_source.as_deref(),
            Some(runtime::BIOME_SOURCE_ID)
        );
    }

    #[test]
    fn the_source_lists_only_biomes_the_level_can_resolve() {
        let source = BzBiomeSource;
        let biomes = source.possible_biomes();
        assert_eq!(biomes, runtime::RESOLVED_BIOMES);
        for quart in [[0, 16, 0], [5, -8, 2]] {
            let index = source.noise_biome(quart, 20_261_105);
            assert!(usize::from(index) < biomes.len());
        }
    }

    #[test]
    fn the_readout_is_declared_as_two_plain_usages() {
        let mut commands = Vec::new();
        let mut event = RegisterCommandsEvent {
            commands: &mut commands,
        };
        register_commands(&mut event);
        let usages: Vec<&str> = commands.iter().map(|c| c.usage.as_str()).collect();
        assert_eq!(
            usages,
            vec![
                "the_bumblezone dimension",
                "the_bumblezone dimension <x:integer> <y:integer> <z:integer> <seed:integer>",
            ]
        );
        assert!(usages.iter().all(|usage| !usage.contains('[')));
    }
}
