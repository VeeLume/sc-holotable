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

/// Pool storage for the `forcefeedback_forcefeedbackeffects` feature.
#[derive(Default)]
pub struct Forcefeedback_forcefeedbackeffectsPools {
    pub force_feedback: Vec<Option<ForceFeedback>>,
    pub force_feedback_pattern: Vec<Option<ForceFeedbackPattern>>,
    pub force_feedback_envelope: Vec<Option<ForceFeedbackEnvelope>>,
    pub force_feedback_effect: Vec<Option<ForceFeedbackEffect>>,
    pub force_feedback_motor: Vec<Option<ForceFeedbackMotor>>,
}
