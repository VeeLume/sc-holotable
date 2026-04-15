// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-inputdeflectiontime`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SInputDeflectionTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInputDeflectionTimeParams {
    /// `minDeflectionTime` (Single)
    #[serde(default)]
    pub min_deflection_time: f32,
    /// `maxDeflectionTime` (Single)
    #[serde(default)]
    pub max_deflection_time: f32,
    /// `penaltyMapping` (Class)
    #[serde(default)]
    pub penalty_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for SInputDeflectionTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_inputdeflectiontime.sinput_deflection_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_inputdeflectiontime.sinput_deflection_time_params }
}

impl<'a> Extract<'a> for SInputDeflectionTimeParams {
    const TYPE_NAME: &'static str = "SInputDeflectionTimeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_deflection_time: inst.get_f32("minDeflectionTime").unwrap_or_default(),
            max_deflection_time: inst.get_f32("maxDeflectionTime").unwrap_or_default(),
            penalty_mapping: match inst.get("penaltyMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `IfcsInputDeflectionTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfcsInputDeflectionTimeParams {
    /// `linearMovement` (Class)
    #[serde(default)]
    pub linear_movement: Option<Handle<SInputDeflectionTimeParams>>,
    /// `angularMovement` (Class)
    #[serde(default)]
    pub angular_movement: Option<Handle<SInputDeflectionTimeParams>>,
}

impl Pooled for IfcsInputDeflectionTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_inputdeflectiontime.ifcs_input_deflection_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_inputdeflectiontime.ifcs_input_deflection_time_params }
}

impl<'a> Extract<'a> for IfcsInputDeflectionTimeParams {
    const TYPE_NAME: &'static str = "IfcsInputDeflectionTimeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            linear_movement: match inst.get("linearMovement") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInputDeflectionTimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            angular_movement: match inst.get("angularMovement") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInputDeflectionTimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TurretInputDeflectionTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurretInputDeflectionTimeParams {
    /// `angularMovement` (Class)
    #[serde(default)]
    pub angular_movement: Option<Handle<SInputDeflectionTimeParams>>,
}

impl Pooled for TurretInputDeflectionTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_inputdeflectiontime.turret_input_deflection_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_inputdeflectiontime.turret_input_deflection_time_params }
}

impl<'a> Extract<'a> for TurretInputDeflectionTimeParams {
    const TYPE_NAME: &'static str = "TurretInputDeflectionTimeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            angular_movement: match inst.get("angularMovement") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInputDeflectionTimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

