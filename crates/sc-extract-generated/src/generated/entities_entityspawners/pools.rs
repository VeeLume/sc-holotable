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

/// Pool storage for the `entities-entityspawners` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesEntityspawnersPools {
    #[serde(default)]
    pub sspawn_only_on_request: Vec<Option<SSpawnOnlyOnRequest>>,
    #[serde(default)]
    pub ssequencer_spawn_in_useable_spawner_task_params: Vec<Option<SSequencerSpawnInUseableSpawnerTaskParams>>,
}
