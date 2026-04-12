// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `procbreathing` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcbreathingIndex {
    #[serde(default)]
    pub proc_breathing_curve: HashMap<CigGuid, Handle<ProcBreathingCurve>>,
    #[serde(default)]
    pub proc_breathing_curve_database: HashMap<CigGuid, Handle<ProcBreathingCurveDatabase>>,
    #[serde(default)]
    pub proc_breathing_setup: HashMap<CigGuid, Handle<ProcBreathingSetup>>,
}

impl ProcbreathingIndex {
    pub fn len(&self) -> usize {
        self.proc_breathing_curve.len()
            + self.proc_breathing_curve_database.len()
            + self.proc_breathing_setup.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
