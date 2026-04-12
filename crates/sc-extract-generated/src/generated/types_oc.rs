// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `OcclusionCheckSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcclusionCheckSharedParams {
    /// DCB field: `enablePassiveDetectionOcclusion` (Boolean)
    #[serde(default)]
    pub enable_passive_detection_occlusion: bool,
    /// DCB field: `enableActiveDetectionOcclusion` (Boolean)
    #[serde(default)]
    pub enable_active_detection_occlusion: bool,
    /// DCB field: `activeOcclusionIgnoreRange` (Single)
    #[serde(default)]
    pub active_occlusion_ignore_range: f32,
}

impl Pooled for OcclusionCheckSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_oc.occlusion_check_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_oc.occlusion_check_shared_params }
}

impl<'a> Extract<'a> for OcclusionCheckSharedParams {
    const TYPE_NAME: &'static str = "OcclusionCheckSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_passive_detection_occlusion: inst.get_bool("enablePassiveDetectionOcclusion").unwrap_or_default(),
            enable_active_detection_occlusion: inst.get_bool("enableActiveDetectionOcclusion").unwrap_or_default(),
            active_occlusion_ignore_range: inst.get_f32("activeOcclusionIgnoreRange").unwrap_or_default(),
        }
    }
}

