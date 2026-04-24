// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `megamap` feature.
#[derive(Default)]
pub struct MegamapPools {
    pub arena_commander_location_object_containers_params:
        Vec<Option<ArenaCommanderLocationObjectContainersParams>>,
    pub arena_commander_planet_override_params: Vec<Option<ArenaCommanderPlanetOverrideParams>>,
    pub arena_commander_scenario_params: Vec<Option<ArenaCommanderScenarioParams>>,
    pub smega_map_solar_system: Vec<Option<SMegaMapSolarSystem>>,
    pub mega_map: Vec<Option<MegaMap>>,
}
