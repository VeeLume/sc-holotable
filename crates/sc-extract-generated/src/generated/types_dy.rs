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

/// DCB type: `DynamicCameraEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCameraEffects {
    /// DCB field: `fov` (Single)
    #[serde(default)]
    pub fov: f32,
    /// DCB field: `forceFOV` (Boolean)
    #[serde(default)]
    pub force_fov: bool,
    /// DCB field: `applyFOVConversationScale` (Boolean)
    #[serde(default)]
    pub apply_fovconversation_scale: bool,
    /// DCB field: `fStop` (Single)
    #[serde(default)]
    pub f_stop: f32,
    /// DCB field: `focalDistance` (Single)
    #[serde(default)]
    pub focal_distance: f32,
    /// DCB field: `focalRange` (Single)
    #[serde(default)]
    pub focal_range: f32,
    /// DCB field: `focusRange` (Single)
    #[serde(default)]
    pub focus_range: f32,
    /// DCB field: `focusMin` (Single)
    #[serde(default)]
    pub focus_min: f32,
    /// DCB field: `focusMinScale` (Single)
    #[serde(default)]
    pub focus_min_scale: f32,
    /// DCB field: `blurAmount` (Single)
    #[serde(default)]
    pub blur_amount: f32,
    /// DCB field: `lerpToSpeed` (Single)
    #[serde(default)]
    pub lerp_to_speed: f32,
    /// DCB field: `lerpBackTime` (Single)
    #[serde(default)]
    pub lerp_back_time: f32,
    /// DCB field: `lerpBackTimeBreak` (Single)
    #[serde(default)]
    pub lerp_back_time_break: f32,
    /// DCB field: `transparencyPostEffectsExclusionRegion` (Single)
    #[serde(default)]
    pub transparency_post_effects_exclusion_region: f32,
    /// DCB field: `circleOfConfusion` (Single)
    #[serde(default)]
    pub circle_of_confusion: f32,
    /// DCB field: `focalRangePadding` (Single)
    #[serde(default)]
    pub focal_range_padding: f32,
    /// DCB field: `multipleTargetFStop` (Single)
    #[serde(default)]
    pub multiple_target_fstop: f32,
    /// DCB field: `manualExposure` (Boolean)
    #[serde(default)]
    pub manual_exposure: bool,
    /// DCB field: `targetExposureValue` (Single)
    #[serde(default)]
    pub target_exposure_value: f32,
    /// DCB field: `exposureCompensation` (Single)
    #[serde(default)]
    pub exposure_compensation: f32,
    /// DCB field: `outOfFocusMaxLuminance` (Single)
    #[serde(default)]
    pub out_of_focus_max_luminance: f32,
    /// DCB field: `applyRendererParams` (Boolean)
    #[serde(default)]
    pub apply_renderer_params: bool,
    /// DCB field: `rendererParams` (Class)
    #[serde(default)]
    pub renderer_params: Option<Handle<DynamicCameraEffectsRendererParams>>,
}

impl Pooled for DynamicCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dy.dynamic_camera_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dy.dynamic_camera_effects }
}

impl<'a> Extract<'a> for DynamicCameraEffects {
    const TYPE_NAME: &'static str = "DynamicCameraEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fov: inst.get_f32("fov").unwrap_or_default(),
            force_fov: inst.get_bool("forceFOV").unwrap_or_default(),
            apply_fovconversation_scale: inst.get_bool("applyFOVConversationScale").unwrap_or_default(),
            f_stop: inst.get_f32("fStop").unwrap_or_default(),
            focal_distance: inst.get_f32("focalDistance").unwrap_or_default(),
            focal_range: inst.get_f32("focalRange").unwrap_or_default(),
            focus_range: inst.get_f32("focusRange").unwrap_or_default(),
            focus_min: inst.get_f32("focusMin").unwrap_or_default(),
            focus_min_scale: inst.get_f32("focusMinScale").unwrap_or_default(),
            blur_amount: inst.get_f32("blurAmount").unwrap_or_default(),
            lerp_to_speed: inst.get_f32("lerpToSpeed").unwrap_or_default(),
            lerp_back_time: inst.get_f32("lerpBackTime").unwrap_or_default(),
            lerp_back_time_break: inst.get_f32("lerpBackTimeBreak").unwrap_or_default(),
            transparency_post_effects_exclusion_region: inst.get_f32("transparencyPostEffectsExclusionRegion").unwrap_or_default(),
            circle_of_confusion: inst.get_f32("circleOfConfusion").unwrap_or_default(),
            focal_range_padding: inst.get_f32("focalRangePadding").unwrap_or_default(),
            multiple_target_fstop: inst.get_f32("multipleTargetFStop").unwrap_or_default(),
            manual_exposure: inst.get_bool("manualExposure").unwrap_or_default(),
            target_exposure_value: inst.get_f32("targetExposureValue").unwrap_or_default(),
            exposure_compensation: inst.get_f32("exposureCompensation").unwrap_or_default(),
            out_of_focus_max_luminance: inst.get_f32("outOfFocusMaxLuminance").unwrap_or_default(),
            apply_renderer_params: inst.get_bool("applyRendererParams").unwrap_or_default(),
            renderer_params: match inst.get("rendererParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DynamicCameraEffectsRendererParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DynamicCameraEffectsRendererParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DynamicCameraEffectsRendererParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCameraEffectsRendererParams {
    /// DCB field: `bloomIntensity` (Single)
    #[serde(default)]
    pub bloom_intensity: f32,
    /// DCB field: `chromaticAberration` (Single)
    #[serde(default)]
    pub chromatic_aberration: f32,
    /// DCB field: `filmGrainSize` (Single)
    #[serde(default)]
    pub film_grain_size: f32,
    /// DCB field: `filmGrainStrength` (Single)
    #[serde(default)]
    pub film_grain_strength: f32,
    /// DCB field: `shutterSpeed` (Single)
    #[serde(default)]
    pub shutter_speed: f32,
    /// DCB field: `vignetting` (Single)
    #[serde(default)]
    pub vignetting: f32,
}

impl Pooled for DynamicCameraEffectsRendererParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dy.dynamic_camera_effects_renderer_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dy.dynamic_camera_effects_renderer_params }
}

impl<'a> Extract<'a> for DynamicCameraEffectsRendererParams {
    const TYPE_NAME: &'static str = "DynamicCameraEffectsRendererParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            bloom_intensity: inst.get_f32("bloomIntensity").unwrap_or_default(),
            chromatic_aberration: inst.get_f32("chromaticAberration").unwrap_or_default(),
            film_grain_size: inst.get_f32("filmGrainSize").unwrap_or_default(),
            film_grain_strength: inst.get_f32("filmGrainStrength").unwrap_or_default(),
            shutter_speed: inst.get_f32("shutterSpeed").unwrap_or_default(),
            vignetting: inst.get_f32("vignetting").unwrap_or_default(),
        }
    }
}

/// DCB type: `DynamicCameraEffectsList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCameraEffectsList {
    /// DCB field: `conversation` (Reference)
    #[serde(default)]
    pub conversation: Option<CigGuid>,
    /// DCB field: `interactionModeFocus` (Reference)
    #[serde(default)]
    pub interaction_mode_focus: Option<CigGuid>,
    /// DCB field: `mobiglas` (Reference)
    #[serde(default)]
    pub mobiglas: Option<CigGuid>,
    /// DCB field: `personalInnerThought` (Reference)
    #[serde(default)]
    pub personal_inner_thought: Option<CigGuid>,
    /// DCB field: `externalInventory` (Reference)
    #[serde(default)]
    pub external_inventory: Option<CigGuid>,
    /// DCB field: `personalInventory` (Reference)
    #[serde(default)]
    pub personal_inventory: Option<CigGuid>,
    /// DCB field: `lootInventory` (Reference)
    #[serde(default)]
    pub loot_inventory: Option<CigGuid>,
    /// DCB field: `onFoot` (Reference)
    #[serde(default)]
    pub on_foot: Option<CigGuid>,
    /// DCB field: `vehicleSeat` (Reference)
    #[serde(default)]
    pub vehicle_seat: Option<CigGuid>,
    /// DCB field: `hacking` (Reference)
    #[serde(default)]
    pub hacking: Option<CigGuid>,
    /// DCB field: `inspectMode` (Reference)
    #[serde(default)]
    pub inspect_mode: Option<CigGuid>,
    /// DCB field: `prototypeMobiGlas` (Reference)
    #[serde(default)]
    pub prototype_mobi_glas: Option<CigGuid>,
    /// DCB field: `simpod` (Reference)
    #[serde(default)]
    pub simpod: Option<CigGuid>,
    /// DCB field: `jumpTravel` (Reference)
    #[serde(default)]
    pub jump_travel: Option<CigGuid>,
}

impl Pooled for DynamicCameraEffectsList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dy.dynamic_camera_effects_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dy.dynamic_camera_effects_list }
}

impl<'a> Extract<'a> for DynamicCameraEffectsList {
    const TYPE_NAME: &'static str = "DynamicCameraEffectsList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            conversation: inst.get("conversation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interaction_mode_focus: inst.get("interactionModeFocus").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mobiglas: inst.get("mobiglas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            personal_inner_thought: inst.get("personalInnerThought").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            external_inventory: inst.get("externalInventory").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            personal_inventory: inst.get("personalInventory").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            loot_inventory: inst.get("lootInventory").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            on_foot: inst.get("onFoot").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            vehicle_seat: inst.get("vehicleSeat").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            hacking: inst.get("hacking").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            inspect_mode: inst.get("inspectMode").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            prototype_mobi_glas: inst.get("prototypeMobiGlas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            simpod: inst.get("simpod").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            jump_travel: inst.get("jumpTravel").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

