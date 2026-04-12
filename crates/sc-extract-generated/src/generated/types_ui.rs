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

/// DCB type: `UIHoloVehicle_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIHoloVehicle_Config {
    /// DCB field: `hitEffectTime` (Single)
    #[serde(default)]
    pub hit_effect_time: f32,
    /// DCB field: `highlightEffectTime` (Single)
    #[serde(default)]
    pub highlight_effect_time: f32,
    /// DCB field: `flickerTime` (Single)
    #[serde(default)]
    pub flicker_time: f32,
    /// DCB field: `shieldDistance` (Single)
    #[serde(default)]
    pub shield_distance: f32,
    /// DCB field: `unknownMarkerProxyModel` (String)
    #[serde(default)]
    pub unknown_marker_proxy_model: String,
    /// DCB field: `shieldProxyModel` (String)
    #[serde(default)]
    pub shield_proxy_model: String,
    /// DCB field: `turretViewProxyModel` (String)
    #[serde(default)]
    pub turret_view_proxy_model: String,
    /// DCB field: `turretViewLengthRatio` (Single)
    #[serde(default)]
    pub turret_view_length_ratio: f32,
    /// DCB field: `turretViewWidthRatio` (Single)
    #[serde(default)]
    pub turret_view_width_ratio: f32,
    /// DCB field: `turretViewPitch` (Single)
    #[serde(default)]
    pub turret_view_pitch: f32,
    /// DCB field: `turretViewRoll` (Single)
    #[serde(default)]
    pub turret_view_roll: f32,
    /// DCB field: `turretViewYaw` (Single)
    #[serde(default)]
    pub turret_view_yaw: f32,
    /// DCB field: `turretAimProxyModel` (String)
    #[serde(default)]
    pub turret_aim_proxy_model: String,
    /// DCB field: `turretAimWidthRatio` (Single)
    #[serde(default)]
    pub turret_aim_width_ratio: f32,
    /// DCB field: `cameraFOV` (Single)
    #[serde(default)]
    pub camera_fov: f32,
    /// DCB field: `ownCameraDistanceScaler` (Single)
    #[serde(default)]
    pub own_camera_distance_scaler: f32,
    /// DCB field: `ownAngularOffsetRange` (Single)
    #[serde(default)]
    pub own_angular_offset_range: f32,
    /// DCB field: `ownDefaultViewAngle` (Single)
    #[serde(default)]
    pub own_default_view_angle: f32,
    /// DCB field: `ownTranslationSmoothingTime` (Single)
    #[serde(default)]
    pub own_translation_smoothing_time: f32,
    /// DCB field: `ownTranslationOffsetRadiusRatio` (Single)
    #[serde(default)]
    pub own_translation_offset_radius_ratio: f32,
    /// DCB field: `hitMaterialDarkenFactor` (Single)
    #[serde(default)]
    pub hit_material_darken_factor: f32,
    /// DCB field: `hitIndicatorAnimTime` (Single)
    #[serde(default)]
    pub hit_indicator_anim_time: f32,
    /// DCB field: `targetCameraDistanceScaler` (Single)
    #[serde(default)]
    pub target_camera_distance_scaler: f32,
    /// DCB field: `cameraIntroTime` (Single)
    #[serde(default)]
    pub camera_intro_time: f32,
    /// DCB field: `cameraIntroDistanceScaler` (Single)
    #[serde(default)]
    pub camera_intro_distance_scaler: f32,
    /// DCB field: `cameraIntroInterpolationMode` (EnumChoice)
    #[serde(default)]
    pub camera_intro_interpolation_mode: String,
    /// DCB field: `cameraViewsTransitionTime` (Single)
    #[serde(default)]
    pub camera_views_transition_time: f32,
    /// DCB field: `cameraViewTransitionMode` (EnumChoice)
    #[serde(default)]
    pub camera_view_transition_mode: String,
    /// DCB field: `vehicleMaterial` (String)
    #[serde(default)]
    pub vehicle_material: String,
    /// DCB field: `itemHighlightMaterial` (String)
    #[serde(default)]
    pub item_highlight_material: String,
    /// DCB field: `shieldMaterial` (String)
    #[serde(default)]
    pub shield_material: String,
    /// DCB field: `turretViewStandbyMaterial` (String)
    #[serde(default)]
    pub turret_view_standby_material: String,
    /// DCB field: `turretViewFireMaterial` (String)
    #[serde(default)]
    pub turret_view_fire_material: String,
    /// DCB field: `turretAimMaterial` (String)
    #[serde(default)]
    pub turret_aim_material: String,
    /// DCB field: `vehicleHitMaterial` (String)
    #[serde(default)]
    pub vehicle_hit_material: String,
    /// DCB field: `shieldHitMaterial` (String)
    #[serde(default)]
    pub shield_hit_material: String,
    /// DCB field: `itemTypeWhitelist` (EnumChoice (array))
    #[serde(default)]
    pub item_type_whitelist: Vec<String>,
    /// DCB field: `silhouetteParams` (Class)
    #[serde(default)]
    pub silhouette_params: Option<Handle<SSilhouetteParamsDef>>,
    /// DCB field: `directionArrowGeomName` (String)
    #[serde(default)]
    pub direction_arrow_geom_name: String,
    /// DCB field: `directionArrowTipGeomName` (String)
    #[serde(default)]
    pub direction_arrow_tip_geom_name: String,
    /// DCB field: `directionArrowScale` (Single)
    #[serde(default)]
    pub direction_arrow_scale: f32,
    /// DCB field: `directionArrowForwardOffset` (Single)
    #[serde(default)]
    pub direction_arrow_forward_offset: f32,
    /// DCB field: `directionArrowUpOffset` (Single)
    #[serde(default)]
    pub direction_arrow_up_offset: f32,
    /// DCB field: `directionArrowRightOffset` (Single)
    #[serde(default)]
    pub direction_arrow_right_offset: f32,
}

impl Pooled for UIHoloVehicle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiholo_vehicle_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiholo_vehicle_config }
}

impl<'a> Extract<'a> for UIHoloVehicle_Config {
    const TYPE_NAME: &'static str = "UIHoloVehicle_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hit_effect_time: inst.get_f32("hitEffectTime").unwrap_or_default(),
            highlight_effect_time: inst.get_f32("highlightEffectTime").unwrap_or_default(),
            flicker_time: inst.get_f32("flickerTime").unwrap_or_default(),
            shield_distance: inst.get_f32("shieldDistance").unwrap_or_default(),
            unknown_marker_proxy_model: inst.get_str("unknownMarkerProxyModel").map(String::from).unwrap_or_default(),
            shield_proxy_model: inst.get_str("shieldProxyModel").map(String::from).unwrap_or_default(),
            turret_view_proxy_model: inst.get_str("turretViewProxyModel").map(String::from).unwrap_or_default(),
            turret_view_length_ratio: inst.get_f32("turretViewLengthRatio").unwrap_or_default(),
            turret_view_width_ratio: inst.get_f32("turretViewWidthRatio").unwrap_or_default(),
            turret_view_pitch: inst.get_f32("turretViewPitch").unwrap_or_default(),
            turret_view_roll: inst.get_f32("turretViewRoll").unwrap_or_default(),
            turret_view_yaw: inst.get_f32("turretViewYaw").unwrap_or_default(),
            turret_aim_proxy_model: inst.get_str("turretAimProxyModel").map(String::from).unwrap_or_default(),
            turret_aim_width_ratio: inst.get_f32("turretAimWidthRatio").unwrap_or_default(),
            camera_fov: inst.get_f32("cameraFOV").unwrap_or_default(),
            own_camera_distance_scaler: inst.get_f32("ownCameraDistanceScaler").unwrap_or_default(),
            own_angular_offset_range: inst.get_f32("ownAngularOffsetRange").unwrap_or_default(),
            own_default_view_angle: inst.get_f32("ownDefaultViewAngle").unwrap_or_default(),
            own_translation_smoothing_time: inst.get_f32("ownTranslationSmoothingTime").unwrap_or_default(),
            own_translation_offset_radius_ratio: inst.get_f32("ownTranslationOffsetRadiusRatio").unwrap_or_default(),
            hit_material_darken_factor: inst.get_f32("hitMaterialDarkenFactor").unwrap_or_default(),
            hit_indicator_anim_time: inst.get_f32("hitIndicatorAnimTime").unwrap_or_default(),
            target_camera_distance_scaler: inst.get_f32("targetCameraDistanceScaler").unwrap_or_default(),
            camera_intro_time: inst.get_f32("cameraIntroTime").unwrap_or_default(),
            camera_intro_distance_scaler: inst.get_f32("cameraIntroDistanceScaler").unwrap_or_default(),
            camera_intro_interpolation_mode: inst.get_str("cameraIntroInterpolationMode").map(String::from).unwrap_or_default(),
            camera_views_transition_time: inst.get_f32("cameraViewsTransitionTime").unwrap_or_default(),
            camera_view_transition_mode: inst.get_str("cameraViewTransitionMode").map(String::from).unwrap_or_default(),
            vehicle_material: inst.get_str("vehicleMaterial").map(String::from).unwrap_or_default(),
            item_highlight_material: inst.get_str("itemHighlightMaterial").map(String::from).unwrap_or_default(),
            shield_material: inst.get_str("shieldMaterial").map(String::from).unwrap_or_default(),
            turret_view_standby_material: inst.get_str("turretViewStandbyMaterial").map(String::from).unwrap_or_default(),
            turret_view_fire_material: inst.get_str("turretViewFireMaterial").map(String::from).unwrap_or_default(),
            turret_aim_material: inst.get_str("turretAimMaterial").map(String::from).unwrap_or_default(),
            vehicle_hit_material: inst.get_str("vehicleHitMaterial").map(String::from).unwrap_or_default(),
            shield_hit_material: inst.get_str("shieldHitMaterial").map(String::from).unwrap_or_default(),
            item_type_whitelist: inst.get_array("itemTypeWhitelist")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            silhouette_params: match inst.get("silhouetteParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSilhouetteParamsDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSilhouetteParamsDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            direction_arrow_geom_name: inst.get_str("directionArrowGeomName").map(String::from).unwrap_or_default(),
            direction_arrow_tip_geom_name: inst.get_str("directionArrowTipGeomName").map(String::from).unwrap_or_default(),
            direction_arrow_scale: inst.get_f32("directionArrowScale").unwrap_or_default(),
            direction_arrow_forward_offset: inst.get_f32("directionArrowForwardOffset").unwrap_or_default(),
            direction_arrow_up_offset: inst.get_f32("directionArrowUpOffset").unwrap_or_default(),
            direction_arrow_right_offset: inst.get_f32("directionArrowRightOffset").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// DCB field: `DamageColours` (StrongPointer)
    #[serde(default)]
    pub damage_colours: Option<Handle<Flash_Palette>>,
    /// DCB field: `ColorStates` (StrongPointer)
    #[serde(default)]
    pub color_states: Option<Handle<UIStateColor>>,
    /// DCB field: `InnerThought` (StrongPointer)
    #[serde(default)]
    pub inner_thought: Option<Handle<InnerThought_Config>>,
    /// DCB field: `FPSReticleConfig` (StrongPointer)
    #[serde(default)]
    pub fpsreticle_config: Option<Handle<FPSReticle_Config>>,
    /// DCB field: `EVAReticleConfig` (StrongPointer)
    #[serde(default)]
    pub evareticle_config: Option<Handle<EVAReticle_Config>>,
    /// DCB field: `playerChoiceIMConfig` (Reference)
    #[serde(default)]
    pub player_choice_imconfig: Option<CigGuid>,
    /// DCB field: `visorHUDConfig` (Reference)
    #[serde(default)]
    pub visor_hudconfig: Option<CigGuid>,
    /// DCB field: `playerChoicePITConfig` (Reference)
    #[serde(default)]
    pub player_choice_pitconfig: Option<CigGuid>,
    /// DCB field: `flightHUDUIViewConfig` (Reference)
    #[serde(default)]
    pub flight_huduiview_config: Option<CigGuid>,
}

impl Pooled for UIConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiconfig }
}

impl<'a> Extract<'a> for UIConfig {
    const TYPE_NAME: &'static str = "UIConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_colours: match inst.get("DamageColours") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Flash_Palette>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Flash_Palette>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color_states: match inst.get("ColorStates") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIStateColor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIStateColor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inner_thought: match inst.get("InnerThought") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InnerThought_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InnerThought_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fpsreticle_config: match inst.get("FPSReticleConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FPSReticle_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FPSReticle_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            evareticle_config: match inst.get("EVAReticleConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EVAReticle_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EVAReticle_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_choice_imconfig: inst.get("playerChoiceIMConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            visor_hudconfig: inst.get("visorHUDConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            player_choice_pitconfig: inst.get("playerChoicePITConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            flight_huduiview_config: inst.get("flightHUDUIViewConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `UIStateColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStateColor {
    /// DCB field: `thresholds` (Class (array))
    #[serde(default)]
    pub thresholds: Vec<Handle<UIStateColor_Threshold>>,
}

impl Pooled for UIStateColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uistate_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uistate_color }
}

impl<'a> Extract<'a> for UIStateColor {
    const TYPE_NAME: &'static str = "UIStateColor";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            thresholds: inst.get_array("thresholds")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIStateColor_Threshold>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIStateColor_Threshold>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UIStateColor_Threshold`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStateColor_Threshold {
    /// DCB field: `minThresholdValue` (Single)
    #[serde(default)]
    pub min_threshold_value: f32,
    /// DCB field: `stateColor` (EnumChoice)
    #[serde(default)]
    pub state_color: String,
}

impl Pooled for UIStateColor_Threshold {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uistate_color_threshold }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uistate_color_threshold }
}

impl<'a> Extract<'a> for UIStateColor_Threshold {
    const TYPE_NAME: &'static str = "UIStateColor_Threshold";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_threshold_value: inst.get_f32("minThresholdValue").unwrap_or_default(),
            state_color: inst.get_str("stateColor").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `UIModeVisibilitySettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIModeVisibilitySettings {
    /// DCB field: `itemPortTrackers` (Boolean)
    #[serde(default)]
    pub item_port_trackers: bool,
    /// DCB field: `grenadeTrackers` (Boolean)
    #[serde(default)]
    pub grenade_trackers: bool,
    /// DCB field: `missionObjectiveTrackers` (Boolean)
    #[serde(default)]
    pub mission_objective_trackers: bool,
    /// DCB field: `unattendedVehicleTrackers` (Boolean)
    #[serde(default)]
    pub unattended_vehicle_trackers: bool,
    /// DCB field: `radarObjectTrackers` (Boolean)
    #[serde(default)]
    pub radar_object_trackers: bool,
}

impl Pooled for UIModeVisibilitySettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uimode_visibility_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uimode_visibility_settings }
}

impl<'a> Extract<'a> for UIModeVisibilitySettings {
    const TYPE_NAME: &'static str = "UIModeVisibilitySettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_port_trackers: inst.get_bool("itemPortTrackers").unwrap_or_default(),
            grenade_trackers: inst.get_bool("grenadeTrackers").unwrap_or_default(),
            mission_objective_trackers: inst.get_bool("missionObjectiveTrackers").unwrap_or_default(),
            unattended_vehicle_trackers: inst.get_bool("unattendedVehicleTrackers").unwrap_or_default(),
            radar_object_trackers: inst.get_bool("radarObjectTrackers").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIStateDisplay_Threshold`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStateDisplay_Threshold {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `timelineLabel` (String)
    #[serde(default)]
    pub timeline_label: String,
    /// DCB field: `minThresholdValue` (Single)
    #[serde(default)]
    pub min_threshold_value: f32,
    /// DCB field: `stateColor` (EnumChoice)
    #[serde(default)]
    pub state_color: String,
}

impl Pooled for UIStateDisplay_Threshold {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uistate_display_threshold }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uistate_display_threshold }
}

impl<'a> Extract<'a> for UIStateDisplay_Threshold {
    const TYPE_NAME: &'static str = "UIStateDisplay_Threshold";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            timeline_label: inst.get_str("timelineLabel").map(String::from).unwrap_or_default(),
            min_threshold_value: inst.get_f32("minThresholdValue").unwrap_or_default(),
            state_color: inst.get_str("stateColor").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `UIStateDisplay`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStateDisplay {
    /// DCB field: `thresholds` (Class (array))
    #[serde(default)]
    pub thresholds: Vec<Handle<UIStateDisplay_Threshold>>,
}

impl Pooled for UIStateDisplay {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uistate_display }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uistate_display }
}

impl<'a> Extract<'a> for UIStateDisplay {
    const TYPE_NAME: &'static str = "UIStateDisplay";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            thresholds: inst.get_array("thresholds")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIStateDisplay_Threshold>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIStateDisplay_Threshold>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UIElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    /// DCB field: `sourceFile` (String)
    #[serde(default)]
    pub source_file: String,
    /// DCB field: `soundsRecord` (Reference)
    #[serde(default)]
    pub sounds_record: Option<CigGuid>,
    /// DCB field: `fontFile` (String)
    #[serde(default)]
    pub font_file: String,
    /// DCB field: `alignMode` (EnumChoice)
    #[serde(default)]
    pub align_mode: String,
    /// DCB field: `alignScale` (Boolean)
    #[serde(default)]
    pub align_scale: bool,
    /// DCB field: `alignMax` (Boolean)
    #[serde(default)]
    pub align_max: bool,
    /// DCB field: `layer` (Int32)
    #[serde(default)]
    pub layer: i32,
    /// DCB field: `skipFirstFrameInit` (Boolean)
    #[serde(default)]
    pub skip_first_frame_init: bool,
    /// DCB field: `keyEvents` (Boolean)
    #[serde(default)]
    pub key_events: bool,
    /// DCB field: `eventsExclusive` (Boolean)
    #[serde(default)]
    pub events_exclusive: bool,
    /// DCB field: `noDepthTest` (Boolean)
    #[serde(default)]
    pub no_depth_test: bool,
    /// DCB field: `forceNoUnload` (Boolean)
    #[serde(default)]
    pub force_no_unload: bool,
    /// DCB field: `noPostProcessing` (Boolean)
    #[serde(default)]
    pub no_post_processing: bool,
}

impl Pooled for UIElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uielement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uielement }
}

impl<'a> Extract<'a> for UIElement {
    const TYPE_NAME: &'static str = "UIElement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            source_file: inst.get_str("sourceFile").map(String::from).unwrap_or_default(),
            sounds_record: inst.get("soundsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            font_file: inst.get_str("fontFile").map(String::from).unwrap_or_default(),
            align_mode: inst.get_str("alignMode").map(String::from).unwrap_or_default(),
            align_scale: inst.get_bool("alignScale").unwrap_or_default(),
            align_max: inst.get_bool("alignMax").unwrap_or_default(),
            layer: inst.get_i32("layer").unwrap_or_default(),
            skip_first_frame_init: inst.get_bool("skipFirstFrameInit").unwrap_or_default(),
            key_events: inst.get_bool("keyEvents").unwrap_or_default(),
            events_exclusive: inst.get_bool("eventsExclusive").unwrap_or_default(),
            no_depth_test: inst.get_bool("noDepthTest").unwrap_or_default(),
            force_no_unload: inst.get_bool("forceNoUnload").unwrap_or_default(),
            no_post_processing: inst.get_bool("noPostProcessing").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIElementSoundsRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElementSoundsRecord {
    /// DCB field: `SoundDBs` (Class (array))
    #[serde(default)]
    pub sound_dbs: Vec<Handle<UIElementSoundEntry>>,
}

impl Pooled for UIElementSoundsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uielement_sounds_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uielement_sounds_record }
}

impl<'a> Extract<'a> for UIElementSoundsRecord {
    const TYPE_NAME: &'static str = "UIElementSoundsRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sound_dbs: inst.get_array("SoundDBs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIElementSoundEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIElementSoundEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UIElementSoundEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElementSoundEntry {
    /// DCB field: `SoundID` (String)
    #[serde(default)]
    pub sound_id: String,
    /// DCB field: `SoundPath` (String)
    #[serde(default)]
    pub sound_path: String,
}

impl Pooled for UIElementSoundEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uielement_sound_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uielement_sound_entry }
}

impl<'a> Extract<'a> for UIElementSoundEntry {
    const TYPE_NAME: &'static str = "UIElementSoundEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sound_id: inst.get_str("SoundID").map(String::from).unwrap_or_default(),
            sound_path: inst.get_str("SoundPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `UIAudioEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAudioEvent {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `audioTrigger` (Class)
    #[serde(default)]
    pub audio_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for UIAudioEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiaudio_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiaudio_event }
}

impl<'a> Extract<'a> for UIAudioEvent {
    const TYPE_NAME: &'static str = "UIAudioEvent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_trigger: match inst.get("audioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UIAudioParameter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAudioParameter {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `rtpc` (Class)
    #[serde(default)]
    pub rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for UIAudioParameter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiaudio_parameter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiaudio_parameter }
}

impl<'a> Extract<'a> for UIAudioParameter {
    const TYPE_NAME: &'static str = "UIAudioParameter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rtpc: match inst.get("rtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UIDialogueEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDialogueEvent {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `dialogueContext` (Reference)
    #[serde(default)]
    pub dialogue_context: Option<CigGuid>,
}

impl Pooled for UIDialogueEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uidialogue_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uidialogue_event }
}

impl<'a> Extract<'a> for UIDialogueEvent {
    const TYPE_NAME: &'static str = "UIDialogueEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dialogue_context: inst.get("dialogueContext").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `UIAudioDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAudioDefinition {
    /// DCB field: `events` (Class (array))
    #[serde(default)]
    pub events: Vec<Handle<UIAudioEvent>>,
    /// DCB field: `parameters` (Class (array))
    #[serde(default)]
    pub parameters: Vec<Handle<UIAudioParameter>>,
    /// DCB field: `dialogueEvents` (Class (array))
    #[serde(default)]
    pub dialogue_events: Vec<Handle<UIDialogueEvent>>,
}

impl Pooled for UIAudioDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiaudio_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiaudio_definition }
}

impl<'a> Extract<'a> for UIAudioDefinition {
    const TYPE_NAME: &'static str = "UIAudioDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            events: inst.get_array("events")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIAudioEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIAudioEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIAudioParameter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIAudioParameter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            dialogue_events: inst.get_array("dialogueEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIDialogueEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIDialogueEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplay3DParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplay3DParams {
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// DCB field: `renderRadius` (Double)
    #[serde(default)]
    pub render_radius: f64,
    /// DCB field: `renderNodeMaterial` (Class)
    #[serde(default)]
    pub render_node_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `inputParams` (Class)
    #[serde(default)]
    pub input_params: Option<Handle<UI3DDisplayInputParams>>,
    /// DCB field: `holographicSettings` (StrongPointer)
    #[serde(default)]
    pub holographic_settings: Option<Handle<UIWorldDisplayHolographicSettings>>,
    /// DCB field: `rotationModeSettings` (Class)
    #[serde(default)]
    pub rotation_mode_settings: Option<Handle<UIWorldDisplayRotationModeParams>>,
    /// DCB field: `centerToSelf` (Boolean)
    #[serde(default)]
    pub center_to_self: bool,
    /// DCB field: `focusChangeDuration` (Single)
    #[serde(default)]
    pub focus_change_duration: f32,
    /// DCB field: `extraZoomScale` (Single)
    #[serde(default)]
    pub extra_zoom_scale: f32,
    /// DCB field: `childlessExtraZoomScale` (Single)
    #[serde(default)]
    pub childless_extra_zoom_scale: f32,
    /// DCB field: `youAreHereZoomDiameter` (Single)
    #[serde(default)]
    pub you_are_here_zoom_diameter: f32,
    /// DCB field: `maxZoomScalingPerUpdate` (Single)
    #[serde(default)]
    pub max_zoom_scaling_per_update: f32,
    /// DCB field: `autoRotationSettings` (StrongPointer)
    #[serde(default)]
    pub auto_rotation_settings: Option<Handle<UIWorldDisplayAutoRotationParams>>,
    /// DCB field: `soundSettings` (StrongPointer)
    #[serde(default)]
    pub sound_settings: Option<Handle<UIWorldDisplaySoundParams>>,
    /// DCB field: `loadoutDummyCameraOffset` (Single)
    #[serde(default)]
    pub loadout_dummy_camera_offset: f32,
}

impl Pooled for UIWorldDisplay3DParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display3_dparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display3_dparams }
}

impl<'a> Extract<'a> for UIWorldDisplay3DParams {
    const TYPE_NAME: &'static str = "UIWorldDisplay3DParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            render_radius: inst.get_f64("renderRadius").unwrap_or_default(),
            render_node_material: match inst.get("renderNodeMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            input_params: match inst.get("inputParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UI3DDisplayInputParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UI3DDisplayInputParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            holographic_settings: match inst.get("holographicSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayHolographicSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayHolographicSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_mode_settings: match inst.get("rotationModeSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayRotationModeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayRotationModeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            center_to_self: inst.get_bool("centerToSelf").unwrap_or_default(),
            focus_change_duration: inst.get_f32("focusChangeDuration").unwrap_or_default(),
            extra_zoom_scale: inst.get_f32("extraZoomScale").unwrap_or_default(),
            childless_extra_zoom_scale: inst.get_f32("childlessExtraZoomScale").unwrap_or_default(),
            you_are_here_zoom_diameter: inst.get_f32("youAreHereZoomDiameter").unwrap_or_default(),
            max_zoom_scaling_per_update: inst.get_f32("maxZoomScalingPerUpdate").unwrap_or_default(),
            auto_rotation_settings: match inst.get("autoRotationSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayAutoRotationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayAutoRotationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sound_settings: match inst.get("soundSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplaySoundParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplaySoundParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loadout_dummy_camera_offset: inst.get_f32("loadoutDummyCameraOffset").unwrap_or_default(),
        }
    }
}

/// DCB type: `UI3DDisplayInputParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UI3DDisplayInputParams {
    /// DCB field: `allowInputOutOfBounds` (Boolean)
    #[serde(default)]
    pub allow_input_out_of_bounds: bool,
    /// DCB field: `disablePimWhenOverScreen` (Boolean)
    #[serde(default)]
    pub disable_pim_when_over_screen: bool,
    /// DCB field: `allowPitch` (Boolean)
    #[serde(default)]
    pub allow_pitch: bool,
    /// DCB field: `allowYaw` (Boolean)
    #[serde(default)]
    pub allow_yaw: bool,
    /// DCB field: `allowPan` (Boolean)
    #[serde(default)]
    pub allow_pan: bool,
    /// DCB field: `allowZoom` (Boolean)
    #[serde(default)]
    pub allow_zoom: bool,
    /// DCB field: `invertPitch` (Boolean)
    #[serde(default)]
    pub invert_pitch: bool,
    /// DCB field: `inputs` (Class)
    #[serde(default)]
    pub inputs: Option<Handle<UI3DDisplayInput>>,
    /// DCB field: `allowAnalogZoom` (Boolean)
    #[serde(default)]
    pub allow_analog_zoom: bool,
    /// DCB field: `analogZoomSensitivity` (Single)
    #[serde(default)]
    pub analog_zoom_sensitivity: f32,
    /// DCB field: `allowDigitalZoom` (Boolean)
    #[serde(default)]
    pub allow_digital_zoom: bool,
    /// DCB field: `digitalZoomSensitivity` (Single)
    #[serde(default)]
    pub digital_zoom_sensitivity: f32,
    /// DCB field: `selectionScale` (Single)
    #[serde(default)]
    pub selection_scale: f32,
}

impl Pooled for UI3DDisplayInputParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.ui3_ddisplay_input_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.ui3_ddisplay_input_params }
}

impl<'a> Extract<'a> for UI3DDisplayInputParams {
    const TYPE_NAME: &'static str = "UI3DDisplayInputParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            allow_input_out_of_bounds: inst.get_bool("allowInputOutOfBounds").unwrap_or_default(),
            disable_pim_when_over_screen: inst.get_bool("disablePimWhenOverScreen").unwrap_or_default(),
            allow_pitch: inst.get_bool("allowPitch").unwrap_or_default(),
            allow_yaw: inst.get_bool("allowYaw").unwrap_or_default(),
            allow_pan: inst.get_bool("allowPan").unwrap_or_default(),
            allow_zoom: inst.get_bool("allowZoom").unwrap_or_default(),
            invert_pitch: inst.get_bool("invertPitch").unwrap_or_default(),
            inputs: match inst.get("inputs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UI3DDisplayInput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UI3DDisplayInput>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_analog_zoom: inst.get_bool("allowAnalogZoom").unwrap_or_default(),
            analog_zoom_sensitivity: inst.get_f32("analogZoomSensitivity").unwrap_or_default(),
            allow_digital_zoom: inst.get_bool("allowDigitalZoom").unwrap_or_default(),
            digital_zoom_sensitivity: inst.get_f32("digitalZoomSensitivity").unwrap_or_default(),
            selection_scale: inst.get_f32("selectionScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `UI3DDisplayInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UI3DDisplayInput {
    /// DCB field: `sensitivity` (Single)
    #[serde(default)]
    pub sensitivity: f32,
    /// DCB field: `hasInertia` (Boolean)
    #[serde(default)]
    pub has_inertia: bool,
    /// DCB field: `deceleration` (Single)
    #[serde(default)]
    pub deceleration: f32,
    /// DCB field: `speedCap` (Single)
    #[serde(default)]
    pub speed_cap: f32,
}

impl Pooled for UI3DDisplayInput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.ui3_ddisplay_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.ui3_ddisplay_input }
}

impl<'a> Extract<'a> for UI3DDisplayInput {
    const TYPE_NAME: &'static str = "UI3DDisplayInput";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sensitivity: inst.get_f32("sensitivity").unwrap_or_default(),
            has_inertia: inst.get_bool("hasInertia").unwrap_or_default(),
            deceleration: inst.get_f32("deceleration").unwrap_or_default(),
            speed_cap: inst.get_f32("speedCap").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayHolographicSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayHolographicSettings {
    /// DCB field: `spawnHoloVolume` (Boolean)
    #[serde(default)]
    pub spawn_holo_volume: bool,
    /// DCB field: `volumeShapeType` (EnumChoice)
    #[serde(default)]
    pub volume_shape_type: String,
    /// DCB field: `cubeSizeMultiplier` (Class)
    #[serde(default)]
    pub cube_size_multiplier: Option<Handle<Vec3>>,
    /// DCB field: `holographicVolumeMaterial` (Class)
    #[serde(default)]
    pub holographic_volume_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `defaultGlow` (Single)
    #[serde(default)]
    pub default_glow: f32,
    /// DCB field: `fadeRatio` (Single)
    #[serde(default)]
    pub fade_ratio: f32,
    /// DCB field: `transparentListDepthBias` (Single)
    #[serde(default)]
    pub transparent_list_depth_bias: f32,
    /// DCB field: `offsetFromAnchor` (Class)
    #[serde(default)]
    pub offset_from_anchor: Option<Handle<QuatT>>,
}

impl Pooled for UIWorldDisplayHolographicSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_holographic_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_holographic_settings }
}

impl<'a> Extract<'a> for UIWorldDisplayHolographicSettings {
    const TYPE_NAME: &'static str = "UIWorldDisplayHolographicSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spawn_holo_volume: inst.get_bool("spawnHoloVolume").unwrap_or_default(),
            volume_shape_type: inst.get_str("volumeShapeType").map(String::from).unwrap_or_default(),
            cube_size_multiplier: match inst.get("cubeSizeMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            holographic_volume_material: match inst.get("holographicVolumeMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_glow: inst.get_f32("defaultGlow").unwrap_or_default(),
            fade_ratio: inst.get_f32("fadeRatio").unwrap_or_default(),
            transparent_list_depth_bias: inst.get_f32("transparentListDepthBias").unwrap_or_default(),
            offset_from_anchor: match inst.get("offsetFromAnchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UIWorldDisplayRotationModeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayRotationModeParams {
    /// DCB field: `defaultPlaneAlignment` (EnumChoice)
    #[serde(default)]
    pub default_plane_alignment: String,
    /// DCB field: `defaultPitchRotation` (Single)
    #[serde(default)]
    pub default_pitch_rotation: f32,
    /// DCB field: `defaultFollowRotationMode` (EnumChoice)
    #[serde(default)]
    pub default_follow_rotation_mode: String,
    /// DCB field: `defaultUseInputRotation` (Boolean)
    #[serde(default)]
    pub default_use_input_rotation: bool,
    /// DCB field: `transitionDuration` (Single)
    #[serde(default)]
    pub transition_duration: f32,
    /// DCB field: `transitionInterpolationMode` (EnumChoice)
    #[serde(default)]
    pub transition_interpolation_mode: String,
}

impl Pooled for UIWorldDisplayRotationModeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_rotation_mode_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_rotation_mode_params }
}

impl<'a> Extract<'a> for UIWorldDisplayRotationModeParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayRotationModeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_plane_alignment: inst.get_str("defaultPlaneAlignment").map(String::from).unwrap_or_default(),
            default_pitch_rotation: inst.get_f32("defaultPitchRotation").unwrap_or_default(),
            default_follow_rotation_mode: inst.get_str("defaultFollowRotationMode").map(String::from).unwrap_or_default(),
            default_use_input_rotation: inst.get_bool("defaultUseInputRotation").unwrap_or_default(),
            transition_duration: inst.get_f32("transitionDuration").unwrap_or_default(),
            transition_interpolation_mode: inst.get_str("transitionInterpolationMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayAutoRotationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayAutoRotationParams {
    /// DCB field: `autoRotationRate` (Single)
    #[serde(default)]
    pub auto_rotation_rate: f32,
    /// DCB field: `autoRotateDelay` (Single)
    #[serde(default)]
    pub auto_rotate_delay: f32,
    /// DCB field: `autoRotateRampUpDuration` (Single)
    #[serde(default)]
    pub auto_rotate_ramp_up_duration: f32,
    /// DCB field: `autoRotateRampDownDuration` (Single)
    #[serde(default)]
    pub auto_rotate_ramp_down_duration: f32,
}

impl Pooled for UIWorldDisplayAutoRotationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_auto_rotation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_auto_rotation_params }
}

impl<'a> Extract<'a> for UIWorldDisplayAutoRotationParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayAutoRotationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            auto_rotation_rate: inst.get_f32("autoRotationRate").unwrap_or_default(),
            auto_rotate_delay: inst.get_f32("autoRotateDelay").unwrap_or_default(),
            auto_rotate_ramp_up_duration: inst.get_f32("autoRotateRampUpDuration").unwrap_or_default(),
            auto_rotate_ramp_down_duration: inst.get_f32("autoRotateRampDownDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplaySoundParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplaySoundParams {
    /// DCB field: `inputSounds` (Class)
    #[serde(default)]
    pub input_sounds: Option<Handle<UIWorldDisplayInputSoundParams>>,
    /// DCB field: `highlightChangeEventTag` (Reference)
    #[serde(default)]
    pub highlight_change_event_tag: Option<CigGuid>,
    /// DCB field: `selectionChangeEventTag` (Reference)
    #[serde(default)]
    pub selection_change_event_tag: Option<CigGuid>,
}

impl Pooled for UIWorldDisplaySoundParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_sound_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_sound_params }
}

impl<'a> Extract<'a> for UIWorldDisplaySoundParams {
    const TYPE_NAME: &'static str = "UIWorldDisplaySoundParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_sounds: match inst.get("inputSounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayInputSoundParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayInputSoundParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_change_event_tag: inst.get("highlightChangeEventTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            selection_change_event_tag: inst.get("selectionChangeEventTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `UIWorldDisplayInputSoundParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayInputSoundParams {
    /// DCB field: `eventTag` (Reference)
    #[serde(default)]
    pub event_tag: Option<CigGuid>,
    /// DCB field: `rtpcParamX` (Class)
    #[serde(default)]
    pub rtpc_param_x: Option<Handle<UIWorldDisplayInputSoundRtpcParam>>,
    /// DCB field: `rtpcParamY` (Class)
    #[serde(default)]
    pub rtpc_param_y: Option<Handle<UIWorldDisplayInputSoundRtpcParam>>,
}

impl Pooled for UIWorldDisplayInputSoundParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_input_sound_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_input_sound_params }
}

impl<'a> Extract<'a> for UIWorldDisplayInputSoundParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayInputSoundParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            event_tag: inst.get("eventTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rtpc_param_x: match inst.get("rtpcParamX") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayInputSoundRtpcParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayInputSoundRtpcParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_param_y: match inst.get("rtpcParamY") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayInputSoundRtpcParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayInputSoundRtpcParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UIWorldDisplayInputSoundRtpcParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayInputSoundRtpcParam {
    /// DCB field: `id` (Class)
    #[serde(default)]
    pub id: Option<Handle<AudioRtpc>>,
    /// DCB field: `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
}

impl Pooled for UIWorldDisplayInputSoundRtpcParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_input_sound_rtpc_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_input_sound_rtpc_param }
}

impl<'a> Extract<'a> for UIWorldDisplayInputSoundRtpcParam {
    const TYPE_NAME: &'static str = "UIWorldDisplayInputSoundRtpcParam";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: match inst.get("id") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            threshold: inst.get_f32("threshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIDataBankDisplay3DSpaceDustParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDataBankDisplay3DSpaceDustParams {
    /// DCB field: `sizeMultiplier` (Single)
    #[serde(default)]
    pub size_multiplier: f32,
    /// DCB field: `minimumSize` (Single)
    #[serde(default)]
    pub minimum_size: f32,
    /// DCB field: `maximumSize` (Single)
    #[serde(default)]
    pub maximum_size: f32,
}

impl Pooled for UIDataBankDisplay3DSpaceDustParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uidata_bank_display3_dspace_dust_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uidata_bank_display3_dspace_dust_params }
}

impl<'a> Extract<'a> for UIDataBankDisplay3DSpaceDustParams {
    const TYPE_NAME: &'static str = "UIDataBankDisplay3DSpaceDustParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            size_multiplier: inst.get_f32("sizeMultiplier").unwrap_or_default(),
            minimum_size: inst.get_f32("minimumSize").unwrap_or_default(),
            maximum_size: inst.get_f32("maximumSize").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIDataBankDisplay3DInterpolateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDataBankDisplay3DInterpolateParams {
    /// DCB field: `openDuration` (Single)
    #[serde(default)]
    pub open_duration: f32,
    /// DCB field: `openCurve` (Class)
    #[serde(default)]
    pub open_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `staggeredDelay` (Single)
    #[serde(default)]
    pub staggered_delay: f32,
}

impl Pooled for UIDataBankDisplay3DInterpolateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uidata_bank_display3_dinterpolate_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uidata_bank_display3_dinterpolate_params }
}

impl<'a> Extract<'a> for UIDataBankDisplay3DInterpolateParams {
    const TYPE_NAME: &'static str = "UIDataBankDisplay3DInterpolateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            open_duration: inst.get_f32("openDuration").unwrap_or_default(),
            open_curve: match inst.get("openCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            staggered_delay: inst.get_f32("staggeredDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIDataBankDisplay3DParams`
///
/// Inherits from: `UIWorldDisplay3DParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDataBankDisplay3DParams {
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// DCB field: `renderRadius` (Double)
    #[serde(default)]
    pub render_radius: f64,
    /// DCB field: `renderNodeMaterial` (Class)
    #[serde(default)]
    pub render_node_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `inputParams` (Class)
    #[serde(default)]
    pub input_params: Option<Handle<UI3DDisplayInputParams>>,
    /// DCB field: `holographicSettings` (StrongPointer)
    #[serde(default)]
    pub holographic_settings: Option<Handle<UIWorldDisplayHolographicSettings>>,
    /// DCB field: `rotationModeSettings` (Class)
    #[serde(default)]
    pub rotation_mode_settings: Option<Handle<UIWorldDisplayRotationModeParams>>,
    /// DCB field: `centerToSelf` (Boolean)
    #[serde(default)]
    pub center_to_self: bool,
    /// DCB field: `focusChangeDuration` (Single)
    #[serde(default)]
    pub focus_change_duration: f32,
    /// DCB field: `extraZoomScale` (Single)
    #[serde(default)]
    pub extra_zoom_scale: f32,
    /// DCB field: `childlessExtraZoomScale` (Single)
    #[serde(default)]
    pub childless_extra_zoom_scale: f32,
    /// DCB field: `youAreHereZoomDiameter` (Single)
    #[serde(default)]
    pub you_are_here_zoom_diameter: f32,
    /// DCB field: `maxZoomScalingPerUpdate` (Single)
    #[serde(default)]
    pub max_zoom_scaling_per_update: f32,
    /// DCB field: `autoRotationSettings` (StrongPointer)
    #[serde(default)]
    pub auto_rotation_settings: Option<Handle<UIWorldDisplayAutoRotationParams>>,
    /// DCB field: `soundSettings` (StrongPointer)
    #[serde(default)]
    pub sound_settings: Option<Handle<UIWorldDisplaySoundParams>>,
    /// DCB field: `loadoutDummyCameraOffset` (Single)
    #[serde(default)]
    pub loadout_dummy_camera_offset: f32,
    /// DCB field: `showSpaceDust` (Boolean)
    #[serde(default)]
    pub show_space_dust: bool,
    /// DCB field: `collapseDistance` (Double)
    #[serde(default)]
    pub collapse_distance: f64,
    /// DCB field: `labelScale` (Single)
    #[serde(default)]
    pub label_scale: f32,
    /// DCB field: `labelOffsetMultiplier` (Single)
    #[serde(default)]
    pub label_offset_multiplier: f32,
    /// DCB field: `overlaySize` (UInt32)
    #[serde(default)]
    pub overlay_size: u32,
    /// DCB field: `maxRelativeHideSize` (Single)
    #[serde(default)]
    pub max_relative_hide_size: f32,
    /// DCB field: `minimumDisplaySizeMultiplier` (Single)
    #[serde(default)]
    pub minimum_display_size_multiplier: f32,
    /// DCB field: `interpolateSettings` (Class)
    #[serde(default)]
    pub interpolate_settings: Option<Handle<UIDataBankDisplay3DInterpolateParams>>,
    /// DCB field: `overlayGrayedOutColor` (Class)
    #[serde(default)]
    pub overlay_grayed_out_color: Option<Handle<SRGBA8>>,
    /// DCB field: `overlayDefaultColor` (Class)
    #[serde(default)]
    pub overlay_default_color: Option<Handle<SRGBA8>>,
    /// DCB field: `overlayHighlightedColor` (Class)
    #[serde(default)]
    pub overlay_highlighted_color: Option<Handle<SRGBA8>>,
    /// DCB field: `overlaySelectedColor` (Class)
    #[serde(default)]
    pub overlay_selected_color: Option<Handle<SRGBA8>>,
    /// DCB field: `overlaySelectedHighlightedColor` (Class)
    #[serde(default)]
    pub overlay_selected_highlighted_color: Option<Handle<SRGBA8>>,
    /// DCB field: `orbitLineWidth` (Single)
    #[serde(default)]
    pub orbit_line_width: f32,
    /// DCB field: `orbitLineDefaultColor` (Class)
    #[serde(default)]
    pub orbit_line_default_color: Option<Handle<SRGBA8>>,
    /// DCB field: `orbitLineHighlightedColor` (Class)
    #[serde(default)]
    pub orbit_line_highlighted_color: Option<Handle<SRGBA8>>,
    /// DCB field: `orbitLineUVStart` (Class)
    #[serde(default)]
    pub orbit_line_uvstart: Option<Handle<Vec2>>,
    /// DCB field: `orbitLineUVSize` (Class)
    #[serde(default)]
    pub orbit_line_uvsize: Option<Handle<Vec2>>,
    /// DCB field: `spaceDustSettings` (Class)
    #[serde(default)]
    pub space_dust_settings: Option<Handle<UIDataBankDisplay3DSpaceDustParams>>,
    /// DCB field: `backdropGeometry` (Class)
    #[serde(default)]
    pub backdrop_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `backdropMaterial` (Class)
    #[serde(default)]
    pub backdrop_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `backdropScale` (Single)
    #[serde(default)]
    pub backdrop_scale: f32,
}

impl Pooled for UIDataBankDisplay3DParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uidata_bank_display3_dparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uidata_bank_display3_dparams }
}

impl<'a> Extract<'a> for UIDataBankDisplay3DParams {
    const TYPE_NAME: &'static str = "UIDataBankDisplay3DParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            render_radius: inst.get_f64("renderRadius").unwrap_or_default(),
            render_node_material: match inst.get("renderNodeMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            input_params: match inst.get("inputParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UI3DDisplayInputParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UI3DDisplayInputParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            holographic_settings: match inst.get("holographicSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayHolographicSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayHolographicSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_mode_settings: match inst.get("rotationModeSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayRotationModeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayRotationModeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            center_to_self: inst.get_bool("centerToSelf").unwrap_or_default(),
            focus_change_duration: inst.get_f32("focusChangeDuration").unwrap_or_default(),
            extra_zoom_scale: inst.get_f32("extraZoomScale").unwrap_or_default(),
            childless_extra_zoom_scale: inst.get_f32("childlessExtraZoomScale").unwrap_or_default(),
            you_are_here_zoom_diameter: inst.get_f32("youAreHereZoomDiameter").unwrap_or_default(),
            max_zoom_scaling_per_update: inst.get_f32("maxZoomScalingPerUpdate").unwrap_or_default(),
            auto_rotation_settings: match inst.get("autoRotationSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayAutoRotationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayAutoRotationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sound_settings: match inst.get("soundSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplaySoundParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplaySoundParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loadout_dummy_camera_offset: inst.get_f32("loadoutDummyCameraOffset").unwrap_or_default(),
            show_space_dust: inst.get_bool("showSpaceDust").unwrap_or_default(),
            collapse_distance: inst.get_f64("collapseDistance").unwrap_or_default(),
            label_scale: inst.get_f32("labelScale").unwrap_or_default(),
            label_offset_multiplier: inst.get_f32("labelOffsetMultiplier").unwrap_or_default(),
            overlay_size: inst.get_u32("overlaySize").unwrap_or_default(),
            max_relative_hide_size: inst.get_f32("maxRelativeHideSize").unwrap_or_default(),
            minimum_display_size_multiplier: inst.get_f32("minimumDisplaySizeMultiplier").unwrap_or_default(),
            interpolate_settings: match inst.get("interpolateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIDataBankDisplay3DInterpolateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIDataBankDisplay3DInterpolateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_grayed_out_color: match inst.get("overlayGrayedOutColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_default_color: match inst.get("overlayDefaultColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_highlighted_color: match inst.get("overlayHighlightedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_selected_color: match inst.get("overlaySelectedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_selected_highlighted_color: match inst.get("overlaySelectedHighlightedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_width: inst.get_f32("orbitLineWidth").unwrap_or_default(),
            orbit_line_default_color: match inst.get("orbitLineDefaultColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_highlighted_color: match inst.get("orbitLineHighlightedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_uvstart: match inst.get("orbitLineUVStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_uvsize: match inst.get("orbitLineUVSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            space_dust_settings: match inst.get("spaceDustSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIDataBankDisplay3DSpaceDustParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIDataBankDisplay3DSpaceDustParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            backdrop_geometry: match inst.get("backdropGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            backdrop_material: match inst.get("backdropMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            backdrop_scale: inst.get_f32("backdropScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayPathStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayPathStateParams {
    /// DCB field: `renderNodeMaterial` (Class)
    #[serde(default)]
    pub render_node_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGBA>>,
    /// DCB field: `scrollUvSpeed` (Single)
    #[serde(default)]
    pub scroll_uv_speed: f32,
}

impl Pooled for UIWorldDisplayPathStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_path_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_path_state_params }
}

impl<'a> Extract<'a> for UIWorldDisplayPathStateParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayPathStateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            render_node_material: match inst.get("renderNodeMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scroll_uv_speed: inst.get_f32("scrollUvSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayPathLineParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayPathLineParams {
    /// DCB field: `lineWidth` (Single)
    #[serde(default)]
    pub line_width: f32,
    /// DCB field: `uvStart` (Class)
    #[serde(default)]
    pub uv_start: Option<Handle<Vec2>>,
    /// DCB field: `uvSize` (Class)
    #[serde(default)]
    pub uv_size: Option<Handle<Vec2>>,
    /// DCB field: `cutOffExtraLengthDivision` (Int32)
    #[serde(default)]
    pub cut_off_extra_length_division: i32,
}

impl Pooled for UIWorldDisplayPathLineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_path_line_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_path_line_params }
}

impl<'a> Extract<'a> for UIWorldDisplayPathLineParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayPathLineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            line_width: inst.get_f32("lineWidth").unwrap_or_default(),
            uv_start: match inst.get("uvStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            uv_size: match inst.get("uvSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cut_off_extra_length_division: inst.get_i32("cutOffExtraLengthDivision").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayPathParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayPathParams {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `validStateSettings` (Class)
    #[serde(default)]
    pub valid_state_settings: Option<Handle<UIWorldDisplayPathStateParams>>,
    /// DCB field: `invalidStateSettings` (Class)
    #[serde(default)]
    pub invalid_state_settings: Option<Handle<UIWorldDisplayPathStateParams>>,
    /// DCB field: `pathLineSettings` (Class)
    #[serde(default)]
    pub path_line_settings: Option<Handle<UIWorldDisplayPathLineParams>>,
}

impl Pooled for UIWorldDisplayPathParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ui.uiworld_display_path_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ui.uiworld_display_path_params }
}

impl<'a> Extract<'a> for UIWorldDisplayPathParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayPathParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            valid_state_settings: match inst.get("validStateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invalid_state_settings: match inst.get("invalidStateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            path_line_settings: match inst.get("pathLineSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathLineParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathLineParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

