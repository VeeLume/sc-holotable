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

/// Pool storage for the `autospawn` feature.
#[derive(Default)]
pub struct AutospawnPools {
    pub auto_spawn_target_class: Vec<Option<AutoSpawnTargetClass>>,
    pub schedule_area_assignment: Vec<Option<ScheduleAreaAssignment>>,
    pub encounter_index_entry: Vec<Option<EncounterIndexEntry>>,
    pub encounter_index: Vec<Option<EncounterIndex>>,
}
