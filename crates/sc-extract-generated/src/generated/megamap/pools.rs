// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `megamap` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MegamapPools {
    #[serde(default)]
    pub arena_commander_location_object_containers_params: Vec<Option<ArenaCommanderLocationObjectContainersParams>>,
    #[serde(default)]
    pub arena_commander_planet_override_params: Vec<Option<ArenaCommanderPlanetOverrideParams>>,
    #[serde(default)]
    pub arena_commander_scenario_params: Vec<Option<ArenaCommanderScenarioParams>>,
    #[serde(default)]
    pub smega_map_solar_system: Vec<Option<SMegaMapSolarSystem>>,
    #[serde(default)]
    pub mega_map: Vec<Option<MegaMap>>,
}
