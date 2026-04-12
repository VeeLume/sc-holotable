// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `intoxication` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntoxicationPools {
    #[serde(default)]
    pub toxi_input_modifier_axis: Vec<Option<ToxiInputModifierAxis>>,
    #[serde(default)]
    pub toxi_input_modifier_distortion: Vec<Option<ToxiInputModifierDistortion>>,
    #[serde(default)]
    pub toxi_input_modifier_delay: Vec<Option<ToxiInputModifierDelay>>,
    #[serde(default)]
    pub intoxication_ifcsmodifier_params: Vec<Option<IntoxicationIFCSModifierParams>>,
    #[serde(default)]
    pub intoxication_turret_modifier_params: Vec<Option<IntoxicationTurretModifierParams>>,
    #[serde(default)]
    pub intoxication_wheeled_modifier_params: Vec<Option<IntoxicationWheeledModifierParams>>,
}
