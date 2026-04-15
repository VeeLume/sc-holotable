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

/// Pool storage for the `aiprofile` feature.
#[derive(Default)]
pub struct AiprofilePools {
    pub aimercy_timer_settings: Vec<Option<AIMercyTimerSettings>>,
    pub stat_definitions: Vec<Option<StatDefinitions>>,
    pub stat: Vec<Option<Stat>>,
    pub stat_influence: Vec<Option<StatInfluence>>,
}
