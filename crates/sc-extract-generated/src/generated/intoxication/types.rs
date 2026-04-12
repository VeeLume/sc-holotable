// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `intoxication`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ToxiInputModifierAxis`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxiInputModifierAxis {
    /// `strength` (Single)
    #[serde(default)]
    pub strength: f32,
    /// `triggerAxes` (Class)
    #[serde(default)]
    pub trigger_axes: Option<Handle<Vec3>>,
}

impl Pooled for ToxiInputModifierAxis {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.intoxication.toxi_input_modifier_axis }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.intoxication.toxi_input_modifier_axis }
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
    /// `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// `amplitudeInterpolant` (Class)
    #[serde(default)]
    pub amplitude_interpolant: Option<Handle<BezierCurve>>,
    /// `axesInputScale` (Class)
    #[serde(default)]
    pub axes_input_scale: Option<Handle<BezierCurve>>,
    /// `minAmplitude` (Single)
    #[serde(default)]
    pub min_amplitude: f32,
    /// `maxAmplitude` (Single)
    #[serde(default)]
    pub max_amplitude: f32,
    /// `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
}

impl Pooled for ToxiInputModifierDistortion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.intoxication.toxi_input_modifier_distortion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.intoxication.toxi_input_modifier_distortion }
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
    /// `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// `minSpeed` (Single)
    #[serde(default)]
    pub min_speed: f32,
    /// `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
}

impl Pooled for ToxiInputModifierDelay {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.intoxication.toxi_input_modifier_delay }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.intoxication.toxi_input_modifier_delay }
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

/// DCB type: `IntoxicationIFCSModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntoxicationIFCSModifierParams {
    /// `rotationDistortion` (Class)
    #[serde(default)]
    pub rotation_distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// `yaw` (Class)
    #[serde(default)]
    pub yaw: Option<Handle<ToxiInputModifierAxis>>,
    /// `pitch` (Class)
    #[serde(default)]
    pub pitch: Option<Handle<ToxiInputModifierAxis>>,
    /// `roll` (Class)
    #[serde(default)]
    pub roll: Option<Handle<ToxiInputModifierAxis>>,
    /// `rotationDelay` (Class)
    #[serde(default)]
    pub rotation_delay: Option<Handle<ToxiInputModifierDelay>>,
    /// `linearDistortion` (Class)
    #[serde(default)]
    pub linear_distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// `forward` (Class)
    #[serde(default)]
    pub forward: Option<Handle<ToxiInputModifierAxis>>,
    /// `left` (Class)
    #[serde(default)]
    pub left: Option<Handle<ToxiInputModifierAxis>>,
    /// `up` (Class)
    #[serde(default)]
    pub up: Option<Handle<ToxiInputModifierAxis>>,
    /// `linearDelay` (Class)
    #[serde(default)]
    pub linear_delay: Option<Handle<ToxiInputModifierDelay>>,
}

impl Pooled for IntoxicationIFCSModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.intoxication.intoxication_ifcsmodifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.intoxication.intoxication_ifcsmodifier_params }
}

impl<'a> Extract<'a> for IntoxicationIFCSModifierParams {
    const TYPE_NAME: &'static str = "IntoxicationIFCSModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rotation_distortion: match inst.get("rotationDistortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw: match inst.get("yaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch: match inst.get("pitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll: match inst.get("roll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_delay: match inst.get("rotationDelay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_distortion: match inst.get("linearDistortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            forward: match inst.get("forward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            left: match inst.get("left") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            up: match inst.get("up") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_delay: match inst.get("linearDelay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `IntoxicationTurretModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntoxicationTurretModifierParams {
    /// `distortion` (Class)
    #[serde(default)]
    pub distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// `delay` (Class)
    #[serde(default)]
    pub delay: Option<Handle<ToxiInputModifierDelay>>,
    /// `yaw` (Class)
    #[serde(default)]
    pub yaw: Option<Handle<ToxiInputModifierAxis>>,
    /// `pitch` (Class)
    #[serde(default)]
    pub pitch: Option<Handle<ToxiInputModifierAxis>>,
}

impl Pooled for IntoxicationTurretModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.intoxication.intoxication_turret_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.intoxication.intoxication_turret_modifier_params }
}

impl<'a> Extract<'a> for IntoxicationTurretModifierParams {
    const TYPE_NAME: &'static str = "IntoxicationTurretModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distortion: match inst.get("distortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: match inst.get("delay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw: match inst.get("yaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch: match inst.get("pitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `IntoxicationWheeledModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntoxicationWheeledModifierParams {
    /// `distortion` (Class)
    #[serde(default)]
    pub distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// `delay` (Class)
    #[serde(default)]
    pub delay: Option<Handle<ToxiInputModifierDelay>>,
    /// `yaw` (Class)
    #[serde(default)]
    pub yaw: Option<Handle<ToxiInputModifierAxis>>,
}

impl Pooled for IntoxicationWheeledModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.intoxication.intoxication_wheeled_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.intoxication.intoxication_wheeled_modifier_params }
}

impl<'a> Extract<'a> for IntoxicationWheeledModifierParams {
    const TYPE_NAME: &'static str = "IntoxicationWheeledModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distortion: match inst.get("distortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: match inst.get("delay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw: match inst.get("yaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

