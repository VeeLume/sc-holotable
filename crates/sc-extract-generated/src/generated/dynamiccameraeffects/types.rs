// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `dynamiccameraeffects`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `DynamicCameraEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCameraEffects {
    /// `fov` (Single)
    #[serde(default)]
    pub fov: f32,
    /// `forceFOV` (Boolean)
    #[serde(default)]
    pub force_fov: bool,
    /// `applyFOVConversationScale` (Boolean)
    #[serde(default)]
    pub apply_fovconversation_scale: bool,
    /// `fStop` (Single)
    #[serde(default)]
    pub f_stop: f32,
    /// `focalDistance` (Single)
    #[serde(default)]
    pub focal_distance: f32,
    /// `focalRange` (Single)
    #[serde(default)]
    pub focal_range: f32,
    /// `focusRange` (Single)
    #[serde(default)]
    pub focus_range: f32,
    /// `focusMin` (Single)
    #[serde(default)]
    pub focus_min: f32,
    /// `focusMinScale` (Single)
    #[serde(default)]
    pub focus_min_scale: f32,
    /// `blurAmount` (Single)
    #[serde(default)]
    pub blur_amount: f32,
    /// `lerpToSpeed` (Single)
    #[serde(default)]
    pub lerp_to_speed: f32,
    /// `lerpBackTime` (Single)
    #[serde(default)]
    pub lerp_back_time: f32,
    /// `lerpBackTimeBreak` (Single)
    #[serde(default)]
    pub lerp_back_time_break: f32,
    /// `transparencyPostEffectsExclusionRegion` (Single)
    #[serde(default)]
    pub transparency_post_effects_exclusion_region: f32,
    /// `circleOfConfusion` (Single)
    #[serde(default)]
    pub circle_of_confusion: f32,
    /// `focalRangePadding` (Single)
    #[serde(default)]
    pub focal_range_padding: f32,
    /// `multipleTargetFStop` (Single)
    #[serde(default)]
    pub multiple_target_fstop: f32,
    /// `manualExposure` (Boolean)
    #[serde(default)]
    pub manual_exposure: bool,
    /// `targetExposureValue` (Single)
    #[serde(default)]
    pub target_exposure_value: f32,
    /// `exposureCompensation` (Single)
    #[serde(default)]
    pub exposure_compensation: f32,
    /// `outOfFocusMaxLuminance` (Single)
    #[serde(default)]
    pub out_of_focus_max_luminance: f32,
    /// `applyRendererParams` (Boolean)
    #[serde(default)]
    pub apply_renderer_params: bool,
    /// `rendererParams` (Class)
    #[serde(default)]
    pub renderer_params: Option<Handle<DynamicCameraEffectsRendererParams>>,
}

impl Pooled for DynamicCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiccameraeffects.dynamic_camera_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiccameraeffects.dynamic_camera_effects }
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
    /// `bloomIntensity` (Single)
    #[serde(default)]
    pub bloom_intensity: f32,
    /// `chromaticAberration` (Single)
    #[serde(default)]
    pub chromatic_aberration: f32,
    /// `filmGrainSize` (Single)
    #[serde(default)]
    pub film_grain_size: f32,
    /// `filmGrainStrength` (Single)
    #[serde(default)]
    pub film_grain_strength: f32,
    /// `shutterSpeed` (Single)
    #[serde(default)]
    pub shutter_speed: f32,
    /// `vignetting` (Single)
    #[serde(default)]
    pub vignetting: f32,
}

impl Pooled for DynamicCameraEffectsRendererParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiccameraeffects.dynamic_camera_effects_renderer_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiccameraeffects.dynamic_camera_effects_renderer_params }
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
    /// `conversation` (Reference)
    #[serde(default)]
    pub conversation: Option<CigGuid>,
    /// `interactionModeFocus` (Reference)
    #[serde(default)]
    pub interaction_mode_focus: Option<CigGuid>,
    /// `mobiglas` (Reference)
    #[serde(default)]
    pub mobiglas: Option<CigGuid>,
    /// `personalInnerThought` (Reference)
    #[serde(default)]
    pub personal_inner_thought: Option<CigGuid>,
    /// `externalInventory` (Reference)
    #[serde(default)]
    pub external_inventory: Option<CigGuid>,
    /// `personalInventory` (Reference)
    #[serde(default)]
    pub personal_inventory: Option<CigGuid>,
    /// `lootInventory` (Reference)
    #[serde(default)]
    pub loot_inventory: Option<CigGuid>,
    /// `onFoot` (Reference)
    #[serde(default)]
    pub on_foot: Option<CigGuid>,
    /// `vehicleSeat` (Reference)
    #[serde(default)]
    pub vehicle_seat: Option<CigGuid>,
    /// `hacking` (Reference)
    #[serde(default)]
    pub hacking: Option<CigGuid>,
    /// `inspectMode` (Reference)
    #[serde(default)]
    pub inspect_mode: Option<CigGuid>,
    /// `prototypeMobiGlas` (Reference)
    #[serde(default)]
    pub prototype_mobi_glas: Option<CigGuid>,
    /// `simpod` (Reference)
    #[serde(default)]
    pub simpod: Option<CigGuid>,
    /// `jumpTravel` (Reference)
    #[serde(default)]
    pub jump_travel: Option<CigGuid>,
}

impl Pooled for DynamicCameraEffectsList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiccameraeffects.dynamic_camera_effects_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiccameraeffects.dynamic_camera_effects_list }
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

/// DCB type: `ConstantDOFPosWeights`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFPosWeights {
    /// `gridDistance` (Int32)
    #[serde(default)]
    pub grid_distance: i32,
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for ConstantDOFPosWeights {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiccameraeffects.constant_dofpos_weights }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiccameraeffects.constant_dofpos_weights }
}

impl<'a> Extract<'a> for ConstantDOFPosWeights {
    const TYPE_NAME: &'static str = "ConstantDOFPosWeights";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grid_distance: inst.get_i32("gridDistance").unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFWeights`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFWeights {
    /// `maxPositionWeight` (Single)
    #[serde(default)]
    pub max_position_weight: f32,
    /// `positionWeights` (Class (array))
    #[serde(default)]
    pub position_weights: Vec<Handle<ConstantDOFPosWeights>>,
    /// `NPC` (Single)
    #[serde(default)]
    pub npc: f32,
    /// `localPlayer` (Single)
    #[serde(default)]
    pub local_player: f32,
    /// `entity` (Single)
    #[serde(default)]
    pub entity: f32,
    /// `actorLookingAtPlayer` (Single)
    #[serde(default)]
    pub actor_looking_at_player: f32,
    /// `door` (Single)
    #[serde(default)]
    pub door: f32,
}

impl Pooled for ConstantDOFWeights {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiccameraeffects.constant_dofweights }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiccameraeffects.constant_dofweights }
}

impl<'a> Extract<'a> for ConstantDOFWeights {
    const TYPE_NAME: &'static str = "ConstantDOFWeights";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_position_weight: inst.get_f32("maxPositionWeight").unwrap_or_default(),
            position_weights: inst.get_array("positionWeights")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ConstantDOFPosWeights>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ConstantDOFPosWeights>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            npc: inst.get_f32("NPC").unwrap_or_default(),
            local_player: inst.get_f32("localPlayer").unwrap_or_default(),
            entity: inst.get_f32("entity").unwrap_or_default(),
            actor_looking_at_player: inst.get_f32("actorLookingAtPlayer").unwrap_or_default(),
            door: inst.get_f32("door").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFGrid`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFGrid {
    /// `verticalGridSize` (Int32)
    #[serde(default)]
    pub vertical_grid_size: i32,
    /// `horizontalGridSize` (Int32)
    #[serde(default)]
    pub horizontal_grid_size: i32,
    /// `verticalSpacing` (Single)
    #[serde(default)]
    pub vertical_spacing: f32,
    /// `horizontalSpacing` (Single)
    #[serde(default)]
    pub horizontal_spacing: f32,
}

impl Pooled for ConstantDOFGrid {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiccameraeffects.constant_dofgrid }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiccameraeffects.constant_dofgrid }
}

impl<'a> Extract<'a> for ConstantDOFGrid {
    const TYPE_NAME: &'static str = "ConstantDOFGrid";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            vertical_grid_size: inst.get_i32("verticalGridSize").unwrap_or_default(),
            horizontal_grid_size: inst.get_i32("horizontalGridSize").unwrap_or_default(),
            vertical_spacing: inst.get_f32("verticalSpacing").unwrap_or_default(),
            horizontal_spacing: inst.get_f32("horizontalSpacing").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFGlobalData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFGlobalData {
    /// `movementThresholdToIgnorePlayer` (Single)
    #[serde(default)]
    pub movement_threshold_to_ignore_player: f32,
    /// `distanceToIgnorePlayer` (Single)
    #[serde(default)]
    pub distance_to_ignore_player: f32,
    /// `rotationThresholdToDisable` (Single)
    #[serde(default)]
    pub rotation_threshold_to_disable: f32,
    /// `pierceability` (Int32)
    #[serde(default)]
    pub pierceability: i32,
    /// `nonEntityDistanceScale` (Single)
    #[serde(default)]
    pub non_entity_distance_scale: f32,
    /// `LODBiasOnTarget` (Int32)
    #[serde(default)]
    pub lodbias_on_target: i32,
    /// `maxRangeScale` (Single)
    #[serde(default)]
    pub max_range_scale: f32,
    /// `circleOfConfusion` (Single)
    #[serde(default)]
    pub circle_of_confusion: f32,
    /// `gridParams` (Class)
    #[serde(default)]
    pub grid_params: Option<Handle<ConstantDOFGrid>>,
    /// `weights` (Class)
    #[serde(default)]
    pub weights: Option<Handle<ConstantDOFWeights>>,
}

impl Pooled for ConstantDOFGlobalData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dynamiccameraeffects.constant_dofglobal_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dynamiccameraeffects.constant_dofglobal_data }
}

impl<'a> Extract<'a> for ConstantDOFGlobalData {
    const TYPE_NAME: &'static str = "ConstantDOFGlobalData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movement_threshold_to_ignore_player: inst.get_f32("movementThresholdToIgnorePlayer").unwrap_or_default(),
            distance_to_ignore_player: inst.get_f32("distanceToIgnorePlayer").unwrap_or_default(),
            rotation_threshold_to_disable: inst.get_f32("rotationThresholdToDisable").unwrap_or_default(),
            pierceability: inst.get_i32("pierceability").unwrap_or_default(),
            non_entity_distance_scale: inst.get_f32("nonEntityDistanceScale").unwrap_or_default(),
            lodbias_on_target: inst.get_i32("LODBiasOnTarget").unwrap_or_default(),
            max_range_scale: inst.get_f32("maxRangeScale").unwrap_or_default(),
            circle_of_confusion: inst.get_f32("circleOfConfusion").unwrap_or_default(),
            grid_params: match inst.get("gridParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ConstantDOFGrid>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConstantDOFGrid>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weights: match inst.get("weights") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ConstantDOFWeights>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConstantDOFWeights>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

