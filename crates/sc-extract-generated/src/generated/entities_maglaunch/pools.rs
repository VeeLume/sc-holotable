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

/// Pool storage for the `entities-maglaunch` feature.
#[derive(Default)]
pub struct EntitiesMaglaunchPools {
    pub smag_launch_recovery_noise_params: Vec<Option<SMagLaunchRecoveryNoiseParams>>,
    pub smag_launch_motion_params: Vec<Option<SMagLaunchMotionParams>>,
    pub mag_launch_params: Vec<Option<MagLaunchParams>>,
}
