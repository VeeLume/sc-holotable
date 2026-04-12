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

/// DCB type: `ToxicGasDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicGasDef {
    /// DCB field: `toxicGas` (Reference)
    #[serde(default)]
    pub toxic_gas: Option<CigGuid>,
    /// DCB field: `minPressureThreshold` (Single)
    #[serde(default)]
    pub min_pressure_threshold: f32,
    /// DCB field: `maxPressureThreshold` (Single)
    #[serde(default)]
    pub max_pressure_threshold: f32,
    /// DCB field: `minBDLBuildupRate` (Single)
    #[serde(default)]
    pub min_bdlbuildup_rate: f32,
    /// DCB field: `maxBDLBuildupRate` (Single)
    #[serde(default)]
    pub max_bdlbuildup_rate: f32,
}

impl Pooled for ToxicGasDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_to.toxic_gas_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_to.toxic_gas_def }
}

impl<'a> Extract<'a> for ToxicGasDef {
    const TYPE_NAME: &'static str = "ToxicGasDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            toxic_gas: inst.get("toxicGas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            min_pressure_threshold: inst.get_f32("minPressureThreshold").unwrap_or_default(),
            max_pressure_threshold: inst.get_f32("maxPressureThreshold").unwrap_or_default(),
            min_bdlbuildup_rate: inst.get_f32("minBDLBuildupRate").unwrap_or_default(),
            max_bdlbuildup_rate: inst.get_f32("maxBDLBuildupRate").unwrap_or_default(),
        }
    }
}

/// DCB type: `ToxiInputModifierAxis`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxiInputModifierAxis {
    /// DCB field: `strength` (Single)
    #[serde(default)]
    pub strength: f32,
    /// DCB field: `triggerAxes` (Class)
    #[serde(default)]
    pub trigger_axes: Option<Handle<Vec3>>,
}

impl Pooled for ToxiInputModifierAxis {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_to.toxi_input_modifier_axis }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_to.toxi_input_modifier_axis }
}

impl<'a> Extract<'a> for ToxiInputModifierAxis {
    const TYPE_NAME: &'static str = "ToxiInputModifierAxis";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            strength: inst.get_f32("strength").unwrap_or_default(),
            trigger_axes: match inst.get("triggerAxes") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ToxiInputModifierDistortion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxiInputModifierDistortion {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `amplitudeInterpolant` (Class)
    #[serde(default)]
    pub amplitude_interpolant: Option<Handle<BezierCurve>>,
    /// DCB field: `axesInputScale` (Class)
    #[serde(default)]
    pub axes_input_scale: Option<Handle<BezierCurve>>,
    /// DCB field: `minAmplitude` (Single)
    #[serde(default)]
    pub min_amplitude: f32,
    /// DCB field: `maxAmplitude` (Single)
    #[serde(default)]
    pub max_amplitude: f32,
    /// DCB field: `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
}

impl Pooled for ToxiInputModifierDistortion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_to.toxi_input_modifier_distortion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_to.toxi_input_modifier_distortion }
}

impl<'a> Extract<'a> for ToxiInputModifierDistortion {
    const TYPE_NAME: &'static str = "ToxiInputModifierDistortion";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            amplitude_interpolant: match inst.get("amplitudeInterpolant") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            axes_input_scale: match inst.get("axesInputScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_amplitude: inst.get_f32("minAmplitude").unwrap_or_default(),
            max_amplitude: inst.get_f32("maxAmplitude").unwrap_or_default(),
            frequency: inst.get_f32("frequency").unwrap_or_default(),
        }
    }
}

/// DCB type: `ToxiInputModifierDelay`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxiInputModifierDelay {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `minSpeed` (Single)
    #[serde(default)]
    pub min_speed: f32,
    /// DCB field: `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
}

impl Pooled for ToxiInputModifierDelay {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_to.toxi_input_modifier_delay }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_to.toxi_input_modifier_delay }
}

impl<'a> Extract<'a> for ToxiInputModifierDelay {
    const TYPE_NAME: &'static str = "ToxiInputModifierDelay";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            min_speed: inst.get_f32("minSpeed").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
        }
    }
}

