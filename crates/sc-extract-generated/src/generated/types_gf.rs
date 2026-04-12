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

/// DCB type: `GForceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GForceParams {
    /// DCB field: `tolerance` (Single)
    #[serde(default)]
    pub tolerance: f32,
    /// DCB field: `maxGees` (Single)
    #[serde(default)]
    pub max_gees: f32,
    /// DCB field: `safeGStress` (Single)
    #[serde(default)]
    pub safe_gstress: f32,
    /// DCB field: `stressMaxoutTime` (Single)
    #[serde(default)]
    pub stress_maxout_time: f32,
    /// DCB field: `stressRecoveryTime` (Single)
    #[serde(default)]
    pub stress_recovery_time: f32,
    /// DCB field: `stressEffects` (Class (array))
    #[serde(default)]
    pub stress_effects: Vec<Handle<StatusEffectBuffMacro>>,
}

impl Pooled for GForceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gf.gforce_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gf.gforce_params }
}

impl<'a> Extract<'a> for GForceParams {
    const TYPE_NAME: &'static str = "GForceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tolerance: inst.get_f32("tolerance").unwrap_or_default(),
            max_gees: inst.get_f32("maxGees").unwrap_or_default(),
            safe_gstress: inst.get_f32("safeGStress").unwrap_or_default(),
            stress_maxout_time: inst.get_f32("stressMaxoutTime").unwrap_or_default(),
            stress_recovery_time: inst.get_f32("stressRecoveryTime").unwrap_or_default(),
            stress_effects: inst.get_array("stressEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatusEffectBuffMacro>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatusEffectBuffMacro>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

