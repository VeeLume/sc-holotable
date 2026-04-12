// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `dynamiclightingrig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCDynamicRigIntensityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDynamicRigIntensityParams {
    /// `referenceLightMinIntensity` (Single)
    #[serde(default)]
    pub reference_light_min_intensity: f32,
    /// `referenceLightMaxIntensity` (Single)
    #[serde(default)]
    pub reference_light_max_intensity: f32,
    /// `rigLightMinIntensity` (Single)
    #[serde(default)]
    pub rig_light_min_intensity: f32,
    /// `rigLightMaxIntensity` (Single)
    #[serde(default)]
    pub rig_light_max_intensity: f32,
    /// `backupIntensity` (Single)
    #[serde(default)]
    pub backup_intensity: f32,
    /// `maxSaturation` (Single)
    #[serde(default)]
    pub max_saturation: f32,
}

impl Pooled for SCDynamicRigIntensityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiclightingrig.scdynamic_rig_intensity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiclightingrig.scdynamic_rig_intensity_params }
}

impl<'a> Extract<'a> for SCDynamicRigIntensityParams {
    const TYPE_NAME: &'static str = "SCDynamicRigIntensityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reference_light_min_intensity: inst.get_f32("referenceLightMinIntensity").unwrap_or_default(),
            reference_light_max_intensity: inst.get_f32("referenceLightMaxIntensity").unwrap_or_default(),
            rig_light_min_intensity: inst.get_f32("rigLightMinIntensity").unwrap_or_default(),
            rig_light_max_intensity: inst.get_f32("rigLightMaxIntensity").unwrap_or_default(),
            backup_intensity: inst.get_f32("backupIntensity").unwrap_or_default(),
            max_saturation: inst.get_f32("maxSaturation").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCDynamicRigLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDynamicRigLightParams {
    /// `lightRadius` (Single)
    #[serde(default)]
    pub light_radius: f32,
    /// `bulbRadius` (Single)
    #[serde(default)]
    pub bulb_radius: f32,
    /// `FOV` (Single)
    #[serde(default)]
    pub fov: f32,
    /// `importance` (EnumChoice)
    #[serde(default)]
    pub importance: String,
    /// `shadowCast` (Boolean)
    #[serde(default)]
    pub shadow_cast: bool,
    /// `maxVisDistance` (Single)
    #[serde(default)]
    pub max_vis_distance: f32,
    /// `distanceFadeRange` (Single)
    #[serde(default)]
    pub distance_fade_range: f32,
    /// `maxShadowCastDistance` (Single)
    #[serde(default)]
    pub max_shadow_cast_distance: f32,
    /// `focusOffset` (Class)
    #[serde(default)]
    pub focus_offset: Option<Handle<Vec3>>,
    /// `intensity` (Class)
    #[serde(default)]
    pub intensity: Option<Handle<SCDynamicRigIntensityParams>>,
}

impl Pooled for SCDynamicRigLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiclightingrig.scdynamic_rig_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiclightingrig.scdynamic_rig_light_params }
}

impl<'a> Extract<'a> for SCDynamicRigLightParams {
    const TYPE_NAME: &'static str = "SCDynamicRigLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_radius: inst.get_f32("lightRadius").unwrap_or_default(),
            bulb_radius: inst.get_f32("bulbRadius").unwrap_or_default(),
            fov: inst.get_f32("FOV").unwrap_or_default(),
            importance: inst.get_str("importance").map(String::from).unwrap_or_default(),
            shadow_cast: inst.get_bool("shadowCast").unwrap_or_default(),
            max_vis_distance: inst.get_f32("maxVisDistance").unwrap_or_default(),
            distance_fade_range: inst.get_f32("distanceFadeRange").unwrap_or_default(),
            max_shadow_cast_distance: inst.get_f32("maxShadowCastDistance").unwrap_or_default(),
            focus_offset: match inst.get("focusOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            intensity: match inst.get("intensity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCDynamicRigIntensityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCDynamicRigIntensityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCDynamicLightingRigGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDynamicLightingRigGlobalParams {
    /// `projectorTexture` (String)
    #[serde(default)]
    pub projector_texture: String,
    /// `offsetPerAxis` (Class)
    #[serde(default)]
    pub offset_per_axis: Option<Handle<Vec3>>,
    /// `frameOfReferenceVerticalOffset` (Single)
    #[serde(default)]
    pub frame_of_reference_vertical_offset: f32,
    /// `colorLerpTime` (Single)
    #[serde(default)]
    pub color_lerp_time: f32,
    /// `positionLerpTime` (Single)
    #[serde(default)]
    pub position_lerp_time: f32,
    /// `minIntensityScaleWhileLerping` (Single)
    #[serde(default)]
    pub min_intensity_scale_while_lerping: f32,
    /// `minRepositionDistanceFromPlayer` (Single)
    #[serde(default)]
    pub min_reposition_distance_from_player: f32,
    /// `minRepositionMovementDistance` (Single)
    #[serde(default)]
    pub min_reposition_movement_distance: f32,
    /// `extendedProjectorFOVScale` (Single)
    #[serde(default)]
    pub extended_projector_fovscale: f32,
    /// `allowRigWithHelmetOn` (Boolean)
    #[serde(default)]
    pub allow_rig_with_helmet_on: bool,
    /// `lights` (Class)
    #[serde(default)]
    pub lights: Option<Handle<SCDynamicRigLightParams>>,
}

impl Pooled for SCDynamicLightingRigGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiclightingrig.scdynamic_lighting_rig_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiclightingrig.scdynamic_lighting_rig_global_params }
}

impl<'a> Extract<'a> for SCDynamicLightingRigGlobalParams {
    const TYPE_NAME: &'static str = "SCDynamicLightingRigGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            projector_texture: inst.get_str("projectorTexture").map(String::from).unwrap_or_default(),
            offset_per_axis: match inst.get("offsetPerAxis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            frame_of_reference_vertical_offset: inst.get_f32("frameOfReferenceVerticalOffset").unwrap_or_default(),
            color_lerp_time: inst.get_f32("colorLerpTime").unwrap_or_default(),
            position_lerp_time: inst.get_f32("positionLerpTime").unwrap_or_default(),
            min_intensity_scale_while_lerping: inst.get_f32("minIntensityScaleWhileLerping").unwrap_or_default(),
            min_reposition_distance_from_player: inst.get_f32("minRepositionDistanceFromPlayer").unwrap_or_default(),
            min_reposition_movement_distance: inst.get_f32("minRepositionMovementDistance").unwrap_or_default(),
            extended_projector_fovscale: inst.get_f32("extendedProjectorFOVScale").unwrap_or_default(),
            allow_rig_with_helmet_on: inst.get_bool("allowRigWithHelmetOn").unwrap_or_default(),
            lights: match inst.get("lights") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCDynamicRigLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCDynamicRigLightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

