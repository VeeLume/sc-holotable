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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SQuantumCameraStateMappingDef`
pub struct SQuantumCameraStateMappingDef {
    /// `strengthMapping` (Class)
    pub strength_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for SQuantumCameraStateMappingDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_quantumtravelcameraeffects
            .squantum_camera_state_mapping_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_quantumtravelcameraeffects
            .squantum_camera_state_mapping_def
    }
}

impl<'a> Extract<'a> for SQuantumCameraStateMappingDef {
    const TYPE_NAME: &'static str = "SQuantumCameraStateMappingDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            strength_mapping: match inst.get("strengthMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `SQuantumCameraStateEffectsDef`
pub struct SQuantumCameraStateEffectsDef {
    /// `angleOuter` (Single)
    pub angle_outer: f32,
    /// `angleInner` (Single)
    pub angle_inner: f32,
    /// `fovScale` (Single)
    pub fov_scale: f32,
    /// `focusDistance` (Single)
    pub focus_distance: f32,
    /// `genericModifiers` (Class)
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
    /// `customMapping` (StrongPointer)
    pub custom_mapping: Option<SQuantumCameraStateMappingDefPtr>,
}

impl Pooled for SQuantumCameraStateEffectsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_quantumtravelcameraeffects
            .squantum_camera_state_effects_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_quantumtravelcameraeffects
            .squantum_camera_state_effects_def
    }
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
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CameraEffectsModifiers>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            custom_mapping: match inst.get("customMapping") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SQuantumCameraStateMappingDefPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SQuantumCameraEffectsDef`
pub struct SQuantumCameraEffectsDef {
    /// `cameraByState` (StrongPointer)
    pub camera_by_state: Option<Handle<SQuantumCameraStateEffectsDef>>,
    /// `smoothingFallback` (Single)
    pub smoothing_fallback: f32,
}

impl Pooled for SQuantumCameraEffectsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_quantumtravelcameraeffects
            .squantum_camera_effects_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_quantumtravelcameraeffects
            .squantum_camera_effects_def
    }
}

impl<'a> Extract<'a> for SQuantumCameraEffectsDef {
    const TYPE_NAME: &'static str = "SQuantumCameraEffectsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera_by_state: match inst.get("cameraByState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SQuantumCameraStateEffectsDef>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            smoothing_fallback: inst.get_f32("smoothingFallback").unwrap_or_default(),
        }
    }
}
