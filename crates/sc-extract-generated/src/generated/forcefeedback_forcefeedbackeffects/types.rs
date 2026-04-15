// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `forcefeedback_forcefeedbackeffects`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ForceFeedback`
pub struct ForceFeedback {
    /// `Patterns` (Class (array))
    pub patterns: Vec<Handle<ForceFeedbackPattern>>,
    /// `Envelopes` (Class (array))
    pub envelopes: Vec<Handle<ForceFeedbackEnvelope>>,
    /// `Effects` (Class (array))
    pub effects: Vec<Handle<ForceFeedbackEffect>>,
}

impl Pooled for ForceFeedback {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.forcefeedback_forcefeedbackeffects.force_feedback }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.forcefeedback_forcefeedbackeffects.force_feedback }
}

impl<'a> Extract<'a> for ForceFeedback {
    const TYPE_NAME: &'static str = "ForceFeedback";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            patterns: inst.get_array("Patterns")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ForceFeedbackPattern>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ForceFeedbackPattern>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            envelopes: inst.get_array("Envelopes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ForceFeedbackEnvelope>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ForceFeedbackEnvelope>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            effects: inst.get_array("Effects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ForceFeedbackEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ForceFeedbackEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ForceFeedbackPattern`
pub struct ForceFeedbackPattern {
    /// `name` (String)
    pub name: String,
    /// `samples` (String)
    pub samples: String,
}

impl Pooled for ForceFeedbackPattern {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.forcefeedback_forcefeedbackeffects.force_feedback_pattern }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.forcefeedback_forcefeedbackeffects.force_feedback_pattern }
}

impl<'a> Extract<'a> for ForceFeedbackPattern {
    const TYPE_NAME: &'static str = "ForceFeedbackPattern";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            samples: inst.get_str("samples").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ForceFeedbackEnvelope`
pub struct ForceFeedbackEnvelope {
    /// `name` (String)
    pub name: String,
    /// `samples` (String)
    pub samples: String,
}

impl Pooled for ForceFeedbackEnvelope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.forcefeedback_forcefeedbackeffects.force_feedback_envelope }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.forcefeedback_forcefeedbackeffects.force_feedback_envelope }
}

impl<'a> Extract<'a> for ForceFeedbackEnvelope {
    const TYPE_NAME: &'static str = "ForceFeedbackEnvelope";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            samples: inst.get_str("samples").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ForceFeedbackEffect`
pub struct ForceFeedbackEffect {
    /// `name` (String)
    pub name: String,
    /// `time` (Single)
    pub time: f32,
    /// `MotorAB` (StrongPointer)
    pub motor_ab: Option<Handle<ForceFeedbackMotor>>,
    /// `MotorA` (StrongPointer)
    pub motor_a: Option<Handle<ForceFeedbackMotor>>,
    /// `MotorB` (StrongPointer)
    pub motor_b: Option<Handle<ForceFeedbackMotor>>,
}

impl Pooled for ForceFeedbackEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.forcefeedback_forcefeedbackeffects.force_feedback_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.forcefeedback_forcefeedbackeffects.force_feedback_effect }
}

impl<'a> Extract<'a> for ForceFeedbackEffect {
    const TYPE_NAME: &'static str = "ForceFeedbackEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            time: inst.get_f32("time").unwrap_or_default(),
            motor_ab: match inst.get("MotorAB") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ForceFeedbackMotor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            motor_a: match inst.get("MotorA") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ForceFeedbackMotor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            motor_b: match inst.get("MotorB") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ForceFeedbackMotor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ForceFeedbackMotor`
pub struct ForceFeedbackMotor {
    /// `frequency` (Single)
    pub frequency: f32,
    /// `pattern` (String)
    pub pattern: String,
    /// `envelope` (String)
    pub envelope: String,
}

impl Pooled for ForceFeedbackMotor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.forcefeedback_forcefeedbackeffects.force_feedback_motor }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.forcefeedback_forcefeedbackeffects.force_feedback_motor }
}

impl<'a> Extract<'a> for ForceFeedbackMotor {
    const TYPE_NAME: &'static str = "ForceFeedbackMotor";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            pattern: inst.get_str("pattern").map(String::from).unwrap_or_default(),
            envelope: inst.get_str("envelope").map(String::from).unwrap_or_default(),
        }
    }
}

