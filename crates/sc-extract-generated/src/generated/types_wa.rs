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

/// DCB type: `WalkToAlignParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkToAlignParams {
    /// DCB field: `reachDistance` (Single)
    #[serde(default)]
    pub reach_distance: f32,
    /// DCB field: `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// DCB field: `maxWalkToTakeTime` (Single)
    #[serde(default)]
    pub max_walk_to_take_time: f32,
    /// DCB field: `maxStuckTakeTime` (Single)
    #[serde(default)]
    pub max_stuck_take_time: f32,
    /// DCB field: `minimumLookAtTargetDistance` (Single)
    #[serde(default)]
    pub minimum_look_at_target_distance: f32,
}

impl Pooled for WalkToAlignParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_wa.walk_to_align_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_wa.walk_to_align_params }
}

impl<'a> Extract<'a> for WalkToAlignParams {
    const TYPE_NAME: &'static str = "WalkToAlignParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reach_distance: inst.get_f32("reachDistance").unwrap_or_default(),
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
            max_walk_to_take_time: inst.get_f32("maxWalkToTakeTime").unwrap_or_default(),
            max_stuck_take_time: inst.get_f32("maxStuckTakeTime").unwrap_or_default(),
            minimum_look_at_target_distance: inst.get_f32("minimumLookAtTargetDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `WaterInteractionEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterInteractionEffectParams {
    /// DCB field: `effect` (Class)
    #[serde(default)]
    pub effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `maxDuration` (Single)
    #[serde(default)]
    pub max_duration: f32,
    /// DCB field: `velocityRange` (Class)
    #[serde(default)]
    pub velocity_range: Option<Handle<Range>>,
}

impl Pooled for WaterInteractionEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_wa.water_interaction_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_wa.water_interaction_effect_params }
}

impl<'a> Extract<'a> for WaterInteractionEffectParams {
    const TYPE_NAME: &'static str = "WaterInteractionEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect: match inst.get("effect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_duration: inst.get_f32("maxDuration").unwrap_or_default(),
            velocity_range: match inst.get("velocityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WaterEffectsGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterEffectsGlobalParams {
    /// DCB field: `exitWaterEffect` (Class)
    #[serde(default)]
    pub exit_water_effect: Option<Handle<WaterInteractionEffectParams>>,
    /// DCB field: `enterWaterEffect` (Class)
    #[serde(default)]
    pub enter_water_effect: Option<Handle<WaterInteractionEffectParams>>,
}

impl Pooled for WaterEffectsGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_wa.water_effects_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_wa.water_effects_global_params }
}

impl<'a> Extract<'a> for WaterEffectsGlobalParams {
    const TYPE_NAME: &'static str = "WaterEffectsGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exit_water_effect: match inst.get("exitWaterEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WaterInteractionEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WaterInteractionEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enter_water_effect: match inst.get("enterWaterEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WaterInteractionEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WaterInteractionEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

