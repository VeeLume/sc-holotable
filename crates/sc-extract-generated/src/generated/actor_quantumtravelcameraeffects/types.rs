// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-quantumtravelcameraeffects`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SQuantumCameraStateMappingDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumCameraStateMappingDef {
    /// `strengthMapping` (Class)
    #[serde(default)]
    pub strength_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for SQuantumCameraStateMappingDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_quantumtravelcameraeffects.squantum_camera_state_mapping_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_quantumtravelcameraeffects.squantum_camera_state_mapping_def }
}

impl<'a> Extract<'a> for SQuantumCameraStateMappingDef {
    const TYPE_NAME: &'static str = "SQuantumCameraStateMappingDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            strength_mapping: match inst.get("strengthMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SQuantumCameraStateEffectsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumCameraStateEffectsDef {
    /// `angleOuter` (Single)
    #[serde(default)]
    pub angle_outer: f32,
    /// `angleInner` (Single)
    #[serde(default)]
    pub angle_inner: f32,
    /// `fovScale` (Single)
    #[serde(default)]
    pub fov_scale: f32,
    /// `focusDistance` (Single)
    #[serde(default)]
    pub focus_distance: f32,
    /// `genericModifiers` (Class)
    #[serde(default)]
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
    /// `customMapping` (StrongPointer)
    #[serde(default)]
    pub custom_mapping: Option<Handle<SQuantumCameraStateMappingDef>>,
}

impl Pooled for SQuantumCameraStateEffectsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_quantumtravelcameraeffects.squantum_camera_state_effects_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_quantumtravelcameraeffects.squantum_camera_state_effects_def }
}

impl<'a> Extract<'a> for SQuantumCameraStateEffectsDef {
    const TYPE_NAME: &'static str = "SQuantumCameraStateEffectsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            angle_outer: inst.get_f32("angleOuter").unwrap_or_default(),
            angle_inner: inst.get_f32("angleInner").unwrap_or_default(),
            fov_scale: inst.get_f32("fovScale").unwrap_or_default(),
            focus_distance: inst.get_f32("focusDistance").unwrap_or_default(),
            generic_modifiers: match inst.get("genericModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraEffectsModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraEffectsModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            custom_mapping: match inst.get("customMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SQuantumCameraStateMappingDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SQuantumCameraStateMappingDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SQuantumCameraEffectsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumCameraEffectsDef {
    /// `cameraByState` (StrongPointer)
    #[serde(default)]
    pub camera_by_state: Option<Handle<SQuantumCameraStateEffectsDef>>,
    /// `smoothingFallback` (Single)
    #[serde(default)]
    pub smoothing_fallback: f32,
}

impl Pooled for SQuantumCameraEffectsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_quantumtravelcameraeffects.squantum_camera_effects_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_quantumtravelcameraeffects.squantum_camera_effects_def }
}

impl<'a> Extract<'a> for SQuantumCameraEffectsDef {
    const TYPE_NAME: &'static str = "SQuantumCameraEffectsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera_by_state: match inst.get("cameraByState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SQuantumCameraStateEffectsDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SQuantumCameraStateEffectsDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            smoothing_fallback: inst.get_f32("smoothingFallback").unwrap_or_default(),
        }
    }
}

