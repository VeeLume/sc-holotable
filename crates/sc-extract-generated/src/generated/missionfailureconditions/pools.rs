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

/// Pool storage for the `missionfailureconditions` feature.
#[derive(Default)]
pub struct MissionfailureconditionsPools {
    pub mission_fail_condition_params: Vec<Option<MissionFailConditionParams>>,
    pub mission_fail_conditions_list: Vec<Option<MissionFailConditionsList>>,
}
