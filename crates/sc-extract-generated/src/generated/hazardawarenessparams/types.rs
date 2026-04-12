// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `hazardawarenessparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `HazardAwarenessParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HazardAwarenessParams {
    /// `windHazardMinSpeed` (Single)
    #[serde(default)]
    pub wind_hazard_min_speed: f32,
    /// `fireHandRaiseDistance` (Single)
    #[serde(default)]
    pub fire_hand_raise_distance: f32,
    /// `handRaiseYawMin` (Single)
    #[serde(default)]
    pub hand_raise_yaw_min: f32,
    /// `handRaiseYawMax` (Single)
    #[serde(default)]
    pub hand_raise_yaw_max: f32,
    /// `handRaiseYawHysteresis` (Single)
    #[serde(default)]
    pub hand_raise_yaw_hysteresis: f32,
    /// `handFollowYawMin` (Single)
    #[serde(default)]
    pub hand_follow_yaw_min: f32,
    /// `handFollowYawMax` (Single)
    #[serde(default)]
    pub hand_follow_yaw_max: f32,
    /// `handFollowYawSmoothing` (Single)
    #[serde(default)]
    pub hand_follow_yaw_smoothing: f32,
    /// `handPitchOffset` (Single)
    #[serde(default)]
    pub hand_pitch_offset: f32,
    /// `handIKDistanceOffset` (Single)
    #[serde(default)]
    pub hand_ikdistance_offset: f32,
    /// `handIKStrength` (Single)
    #[serde(default)]
    pub hand_ikstrength: f32,
    /// `handResumeDelay` (Single)
    #[serde(default)]
    pub hand_resume_delay: f32,
    /// `handRaiseDelay` (Single)
    #[serde(default)]
    pub hand_raise_delay: f32,
    /// `handSwitchDelay` (Single)
    #[serde(default)]
    pub hand_switch_delay: f32,
    /// `handLowerDelay` (Single)
    #[serde(default)]
    pub hand_lower_delay: f32,
    /// `handHazardStopDelay` (Single)
    #[serde(default)]
    pub hand_hazard_stop_delay: f32,
}

impl Pooled for HazardAwarenessParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hazardawarenessparams.hazard_awareness_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hazardawarenessparams.hazard_awareness_params }
}

impl<'a> Extract<'a> for HazardAwarenessParams {
    const TYPE_NAME: &'static str = "HazardAwarenessParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wind_hazard_min_speed: inst.get_f32("windHazardMinSpeed").unwrap_or_default(),
            fire_hand_raise_distance: inst.get_f32("fireHandRaiseDistance").unwrap_or_default(),
            hand_raise_yaw_min: inst.get_f32("handRaiseYawMin").unwrap_or_default(),
            hand_raise_yaw_max: inst.get_f32("handRaiseYawMax").unwrap_or_default(),
            hand_raise_yaw_hysteresis: inst.get_f32("handRaiseYawHysteresis").unwrap_or_default(),
            hand_follow_yaw_min: inst.get_f32("handFollowYawMin").unwrap_or_default(),
            hand_follow_yaw_max: inst.get_f32("handFollowYawMax").unwrap_or_default(),
            hand_follow_yaw_smoothing: inst.get_f32("handFollowYawSmoothing").unwrap_or_default(),
            hand_pitch_offset: inst.get_f32("handPitchOffset").unwrap_or_default(),
            hand_ikdistance_offset: inst.get_f32("handIKDistanceOffset").unwrap_or_default(),
            hand_ikstrength: inst.get_f32("handIKStrength").unwrap_or_default(),
            hand_resume_delay: inst.get_f32("handResumeDelay").unwrap_or_default(),
            hand_raise_delay: inst.get_f32("handRaiseDelay").unwrap_or_default(),
            hand_switch_delay: inst.get_f32("handSwitchDelay").unwrap_or_default(),
            hand_lower_delay: inst.get_f32("handLowerDelay").unwrap_or_default(),
            hand_hazard_stop_delay: inst.get_f32("handHazardStopDelay").unwrap_or_default(),
        }
    }
}

