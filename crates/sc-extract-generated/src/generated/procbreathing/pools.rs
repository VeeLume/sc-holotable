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

/// Pool storage for the `procbreathing` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcbreathingPools {
    #[serde(default)]
    pub proc_breathing_curve: Vec<Option<ProcBreathingCurve>>,
    #[serde(default)]
    pub proc_breathing_curve_database: Vec<Option<ProcBreathingCurveDatabase>>,
    #[serde(default)]
    pub proc_breathing_graph: Vec<Option<ProcBreathingGraph>>,
    #[serde(default)]
    pub proc_breathing_graph_entry: Vec<Option<ProcBreathingGraphEntry>>,
    #[serde(default)]
    pub proc_breathing_exertion: Vec<Option<ProcBreathingExertion>>,
    #[serde(default)]
    pub proc_breathing_hold_breath_noise: Vec<Option<ProcBreathingHoldBreathNoise>>,
    #[serde(default)]
    pub proc_breathing_setup: Vec<Option<ProcBreathingSetup>>,
}
