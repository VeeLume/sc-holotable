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

/// Pool storage for the `entities-scitem-carryables` feature.
#[derive(Default)]
pub struct EntitiesScitemCarryablesPools {
    pub ssequencer_change_stance_carryable_task_params:
        Vec<Option<SSequencerChangeStanceCarryableTaskParams>>,
    pub ssimulation_params_spring_ellipsoid: Vec<Option<SSimulationParamsSpringEllipsoid>>,
}
