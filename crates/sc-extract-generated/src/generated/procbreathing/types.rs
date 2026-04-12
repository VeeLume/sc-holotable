// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `procbreathing`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ProcBreathingCurve`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingCurve {
    /// `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for ProcBreathingCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procbreathing.proc_breathing_curve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procbreathing.proc_breathing_curve }
}

impl<'a> Extract<'a> for ProcBreathingCurve {
    const TYPE_NAME: &'static str = "ProcBreathingCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProcBreathingCurveDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingCurveDatabase {
    /// `breathAnimationCurves` (Reference (array))
    #[serde(default)]
    pub breath_animation_curves: Vec<CigGuid>,
}

impl Pooled for ProcBreathingCurveDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procbreathing.proc_breathing_curve_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procbreathing.proc_breathing_curve_database }
}

impl<'a> Extract<'a> for ProcBreathingCurveDatabase {
    const TYPE_NAME: &'static str = "ProcBreathingCurveDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            breath_animation_curves: inst.get_array("breathAnimationCurves")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcBreathingGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingGraph {
    /// `magnitude` (Single)
    #[serde(default)]
    pub magnitude: f32,
    /// `magnitudeADS` (Single)
    #[serde(default)]
    pub magnitude_ads: f32,
    /// `curveX` (Reference)
    #[serde(default)]
    pub curve_x: Option<CigGuid>,
    /// `curveY` (Reference)
    #[serde(default)]
    pub curve_y: Option<CigGuid>,
    /// `curveZ` (Reference)
    #[serde(default)]
    pub curve_z: Option<CigGuid>,
}

impl Pooled for ProcBreathingGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procbreathing.proc_breathing_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procbreathing.proc_breathing_graph }
}

impl<'a> Extract<'a> for ProcBreathingGraph {
    const TYPE_NAME: &'static str = "ProcBreathingGraph";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            magnitude: inst.get_f32("magnitude").unwrap_or_default(),
            magnitude_ads: inst.get_f32("magnitudeADS").unwrap_or_default(),
            curve_x: inst.get("curveX").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            curve_y: inst.get("curveY").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            curve_z: inst.get("curveZ").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ProcBreathingGraphEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingGraphEntry {
    /// `joint` (String)
    #[serde(default)]
    pub joint: String,
    /// `additionalScale` (Single)
    #[serde(default)]
    pub additional_scale: f32,
    /// `relativeToShoulder` (Boolean)
    #[serde(default)]
    pub relative_to_shoulder: bool,
    /// `firstPersonOnly` (Boolean)
    #[serde(default)]
    pub first_person_only: bool,
    /// `translation` (Class)
    #[serde(default)]
    pub translation: Option<Handle<ProcBreathingGraph>>,
    /// `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<ProcBreathingGraph>>,
}

impl Pooled for ProcBreathingGraphEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procbreathing.proc_breathing_graph_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procbreathing.proc_breathing_graph_entry }
}

impl<'a> Extract<'a> for ProcBreathingGraphEntry {
    const TYPE_NAME: &'static str = "ProcBreathingGraphEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            joint: inst.get_str("joint").map(String::from).unwrap_or_default(),
            additional_scale: inst.get_f32("additionalScale").unwrap_or_default(),
            relative_to_shoulder: inst.get_bool("relativeToShoulder").unwrap_or_default(),
            first_person_only: inst.get_bool("firstPersonOnly").unwrap_or_default(),
            translation: match inst.get("translation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProcBreathingGraph>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProcBreathingGraph>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProcBreathingGraph>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProcBreathingGraph>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProcBreathingExertion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingExertion {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `maxStamina` (Single)
    #[serde(default)]
    pub max_stamina: f32,
    /// `maxFullness` (Single)
    #[serde(default)]
    pub max_fullness: f32,
    /// `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<ProcBreathingGraphEntry>>,
}

impl Pooled for ProcBreathingExertion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procbreathing.proc_breathing_exertion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procbreathing.proc_breathing_exertion }
}

impl<'a> Extract<'a> for ProcBreathingExertion {
    const TYPE_NAME: &'static str = "ProcBreathingExertion";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            max_stamina: inst.get_f32("maxStamina").unwrap_or_default(),
            max_fullness: inst.get_f32("maxFullness").unwrap_or_default(),
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcBreathingGraphEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcBreathingGraphEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcBreathingHoldBreathNoise`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingHoldBreathNoise {
    /// `noiseAng` (Single)
    #[serde(default)]
    pub noise_ang: f32,
    /// `noiseSpeed` (Single)
    #[serde(default)]
    pub noise_speed: f32,
}

impl Pooled for ProcBreathingHoldBreathNoise {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procbreathing.proc_breathing_hold_breath_noise }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procbreathing.proc_breathing_hold_breath_noise }
}

impl<'a> Extract<'a> for ProcBreathingHoldBreathNoise {
    const TYPE_NAME: &'static str = "ProcBreathingHoldBreathNoise";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            noise_ang: inst.get_f32("noiseAng").unwrap_or_default(),
            noise_speed: inst.get_f32("noiseSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcBreathingSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingSetup {
    /// `exertionGraphs` (Class (array))
    #[serde(default)]
    pub exertion_graphs: Vec<Handle<ProcBreathingExertion>>,
    /// `holdBreathNoise` (Class)
    #[serde(default)]
    pub hold_breath_noise: Option<Handle<ProcBreathingHoldBreathNoise>>,
}

impl Pooled for ProcBreathingSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procbreathing.proc_breathing_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procbreathing.proc_breathing_setup }
}

impl<'a> Extract<'a> for ProcBreathingSetup {
    const TYPE_NAME: &'static str = "ProcBreathingSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exertion_graphs: inst.get_array("exertionGraphs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcBreathingExertion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcBreathingExertion>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            hold_breath_noise: match inst.get("holdBreathNoise") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProcBreathingHoldBreathNoise>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProcBreathingHoldBreathNoise>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

