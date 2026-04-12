// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-actortargetedparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ActorTargetedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorTargetedParams {
    /// `frustumScaleW` (Single)
    #[serde(default)]
    pub frustum_scale_w: f32,
    /// `frustumScaleH` (Single)
    #[serde(default)]
    pub frustum_scale_h: f32,
    /// `lookFovAngle` (Single)
    #[serde(default)]
    pub look_fov_angle: f32,
    /// `frustumFar` (Single)
    #[serde(default)]
    pub frustum_far: f32,
}

impl Pooled for ActorTargetedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actortargetedparams.actor_targeted_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actortargetedparams.actor_targeted_params }
}

impl<'a> Extract<'a> for ActorTargetedParams {
    const TYPE_NAME: &'static str = "ActorTargetedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frustum_scale_w: inst.get_f32("frustumScaleW").unwrap_or_default(),
            frustum_scale_h: inst.get_f32("frustumScaleH").unwrap_or_default(),
            look_fov_angle: inst.get_f32("lookFovAngle").unwrap_or_default(),
            frustum_far: inst.get_f32("frustumFar").unwrap_or_default(),
        }
    }
}

