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

/// Pool storage for the `entities-maglaunch` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesMaglaunchPools {
    #[serde(default)]
    pub smag_launch_recovery_noise_params: Vec<Option<SMagLaunchRecoveryNoiseParams>>,
    #[serde(default)]
    pub smag_launch_motion_params: Vec<Option<SMagLaunchMotionParams>>,
    #[serde(default)]
    pub mag_launch_params: Vec<Option<MagLaunchParams>>,
}
