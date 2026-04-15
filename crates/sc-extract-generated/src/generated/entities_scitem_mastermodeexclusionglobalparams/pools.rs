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

/// Pool storage for the `entities-scitem-mastermodeexclusionglobalparams` feature.
#[derive(Default)]
pub struct EntitiesScitemMastermodeexclusionglobalparamsPools {
    pub master_mode_exclusion: Vec<Option<MasterModeExclusion>>,
    pub master_mode_exclusion_global_params: Vec<Option<MasterModeExclusionGlobalParams>>,
}
