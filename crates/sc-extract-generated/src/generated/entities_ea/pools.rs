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

/// Pool storage for the `entities-ea` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesEaPools {
    #[serde(default)]
    pub eascenario_component_params: Vec<Option<EAScenarioComponentParams>>,
    #[serde(default)]
    pub eatransport_transition_group_params: Vec<Option<EATransportTransitionGroupParams>>,
    #[serde(default)]
    pub eatransport_controller_component_params: Vec<Option<EATransportControllerComponentParams>>,
}
