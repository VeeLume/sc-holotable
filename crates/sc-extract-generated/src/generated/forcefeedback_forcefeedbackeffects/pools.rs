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

/// Pool storage for the `forcefeedback_forcefeedbackeffects` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Forcefeedback_forcefeedbackeffectsPools {
    #[serde(default)]
    pub force_feedback: Vec<Option<ForceFeedback>>,
    #[serde(default)]
    pub force_feedback_pattern: Vec<Option<ForceFeedbackPattern>>,
    #[serde(default)]
    pub force_feedback_envelope: Vec<Option<ForceFeedbackEnvelope>>,
    #[serde(default)]
    pub force_feedback_effect: Vec<Option<ForceFeedbackEffect>>,
    #[serde(default)]
    pub force_feedback_motor: Vec<Option<ForceFeedbackMotor>>,
}
