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

/// Pool storage for the `entities-scitem-airlocks` feature.
#[derive(Default)]
pub struct EntitiesScitemAirlocksPools {
    pub airlock_greenzone_params: Vec<Option<AirlockGreenzoneParams>>,
    pub airlock_area_params: Vec<Option<AirlockAreaParams>>,
    pub scitem_airlock_params: Vec<Option<SCItemAirlockParams>>,
}
