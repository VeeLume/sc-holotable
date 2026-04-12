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

/// DCB type: `SQuantumDriveEffectParams_LEGACY`
///
/// Inherits from: `SQuantumDriveEffectBaseParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumDriveEffectParams_LEGACY {
    /// DCB field: `alignEffectTag` (Reference)
    #[serde(default)]
    pub align_effect_tag: Option<CigGuid>,
    /// DCB field: `pinchEffectTag` (Reference)
    #[serde(default)]
    pub pinch_effect_tag: Option<CigGuid>,
    /// DCB field: `travelEffectTag` (Reference)
    #[serde(default)]
    pub travel_effect_tag: Option<CigGuid>,
    /// DCB field: `enterFlashEffectTag` (Reference)
    #[serde(default)]
    pub enter_flash_effect_tag: Option<CigGuid>,
    /// DCB field: `exitFlashEffectTag` (Reference)
    #[serde(default)]
    pub exit_flash_effect_tag: Option<CigGuid>,
    /// DCB field: `spoolEffectTag` (Reference)
    #[serde(default)]
    pub spool_effect_tag: Option<CigGuid>,
    /// DCB field: `pinchStrengthTag` (Reference)
    #[serde(default)]
    pub pinch_strength_tag: Option<CigGuid>,
    /// DCB field: `spoolStrengthTag` (Reference)
    #[serde(default)]
    pub spool_strength_tag: Option<CigGuid>,
    /// DCB field: `trailsTag` (Reference)
    #[serde(default)]
    pub trails_tag: Option<CigGuid>,
    /// DCB field: `trailsStrTag` (Reference)
    #[serde(default)]
    pub trails_str_tag: Option<CigGuid>,
    /// DCB field: `interdictionEffectTag` (Reference)
    #[serde(default)]
    pub interdiction_effect_tag: Option<CigGuid>,
    /// DCB field: `interdictionExitFlashTag` (Reference)
    #[serde(default)]
    pub interdiction_exit_flash_tag: Option<CigGuid>,
}

impl Pooled for SQuantumDriveEffectParams_LEGACY {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sq.squantum_drive_effect_params_legacy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sq.squantum_drive_effect_params_legacy }
}

impl<'a> Extract<'a> for SQuantumDriveEffectParams_LEGACY {
    const TYPE_NAME: &'static str = "SQuantumDriveEffectParams_LEGACY";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            align_effect_tag: inst.get("alignEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pinch_effect_tag: inst.get("pinchEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            travel_effect_tag: inst.get("travelEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            enter_flash_effect_tag: inst.get("enterFlashEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            exit_flash_effect_tag: inst.get("exitFlashEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            spool_effect_tag: inst.get("spoolEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pinch_strength_tag: inst.get("pinchStrengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            spool_strength_tag: inst.get("spoolStrengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trails_tag: inst.get("trailsTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trails_str_tag: inst.get("trailsStrTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interdiction_effect_tag: inst.get("interdictionEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interdiction_exit_flash_tag: inst.get("interdictionExitFlashTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SQuantumDriveEffectTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumDriveEffectTemplate {
    /// DCB field: `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<SQuantumDriveEffectParams_LEGACY>>,
}

impl Pooled for SQuantumDriveEffectTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sq.squantum_drive_effect_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sq.squantum_drive_effect_template }
}

impl<'a> Extract<'a> for SQuantumDriveEffectTemplate {
    const TYPE_NAME: &'static str = "SQuantumDriveEffectTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SQuantumDriveEffectParams_LEGACY>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SQuantumDriveEffectParams_LEGACY>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SQuantumCameraStateMappingDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumCameraStateMappingDef {
    /// DCB field: `strengthMapping` (Class)
    #[serde(default)]
    pub strength_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for SQuantumCameraStateMappingDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sq.squantum_camera_state_mapping_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sq.squantum_camera_state_mapping_def }
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
    /// DCB field: `angleOuter` (Single)
    #[serde(default)]
    pub angle_outer: f32,
    /// DCB field: `angleInner` (Single)
    #[serde(default)]
    pub angle_inner: f32,
    /// DCB field: `fovScale` (Single)
    #[serde(default)]
    pub fov_scale: f32,
    /// DCB field: `focusDistance` (Single)
    #[serde(default)]
    pub focus_distance: f32,
    /// DCB field: `genericModifiers` (Class)
    #[serde(default)]
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
    /// DCB field: `customMapping` (StrongPointer)
    #[serde(default)]
    pub custom_mapping: Option<Handle<SQuantumCameraStateMappingDef>>,
}

impl Pooled for SQuantumCameraStateEffectsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sq.squantum_camera_state_effects_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sq.squantum_camera_state_effects_def }
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
    /// DCB field: `cameraByState` (StrongPointer)
    #[serde(default)]
    pub camera_by_state: Option<Handle<SQuantumCameraStateEffectsDef>>,
    /// DCB field: `smoothingFallback` (Single)
    #[serde(default)]
    pub smoothing_fallback: f32,
}

impl Pooled for SQuantumCameraEffectsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sq.squantum_camera_effects_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sq.squantum_camera_effects_def }
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

