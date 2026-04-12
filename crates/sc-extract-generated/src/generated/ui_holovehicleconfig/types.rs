// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-holovehicleconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SSilhouetteParamsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSilhouetteParamsDef {
    /// `Enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// `ColourSource` (EnumChoice)
    #[serde(default)]
    pub colour_source: String,
    /// `TintColour` (Class)
    #[serde(default)]
    pub tint_colour: Option<Handle<RGB>>,
    /// `TintStrength` (Single)
    #[serde(default)]
    pub tint_strength: f32,
    /// `Brightness` (Single)
    #[serde(default)]
    pub brightness: f32,
    /// `EdgeWidth` (Single)
    #[serde(default)]
    pub edge_width: f32,
    /// `EdgeIntensity` (Single)
    #[serde(default)]
    pub edge_intensity: f32,
    /// `FillIntensity` (Single)
    #[serde(default)]
    pub fill_intensity: f32,
    /// `BlurRadius` (Single)
    #[serde(default)]
    pub blur_radius: f32,
    /// `EdgeGradient` (Single)
    #[serde(default)]
    pub edge_gradient: f32,
}

impl Pooled for SSilhouetteParamsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_holovehicleconfig.ssilhouette_params_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_holovehicleconfig.ssilhouette_params_def }
}

impl<'a> Extract<'a> for SSilhouetteParamsDef {
    const TYPE_NAME: &'static str = "SSilhouetteParamsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("Enable").unwrap_or_default(),
            colour_source: inst.get_str("ColourSource").map(String::from).unwrap_or_default(),
            tint_colour: match inst.get("TintColour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tint_strength: inst.get_f32("TintStrength").unwrap_or_default(),
            brightness: inst.get_f32("Brightness").unwrap_or_default(),
            edge_width: inst.get_f32("EdgeWidth").unwrap_or_default(),
            edge_intensity: inst.get_f32("EdgeIntensity").unwrap_or_default(),
            fill_intensity: inst.get_f32("FillIntensity").unwrap_or_default(),
            blur_radius: inst.get_f32("BlurRadius").unwrap_or_default(),
            edge_gradient: inst.get_f32("EdgeGradient").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIHoloVehicle_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIHoloVehicle_Config {
    /// `hitEffectTime` (Single)
    #[serde(default)]
    pub hit_effect_time: f32,
    /// `highlightEffectTime` (Single)
    #[serde(default)]
    pub highlight_effect_time: f32,
    /// `flickerTime` (Single)
    #[serde(default)]
    pub flicker_time: f32,
    /// `shieldDistance` (Single)
    #[serde(default)]
    pub shield_distance: f32,
    /// `unknownMarkerProxyModel` (String)
    #[serde(default)]
    pub unknown_marker_proxy_model: String,
    /// `shieldProxyModel` (String)
    #[serde(default)]
    pub shield_proxy_model: String,
    /// `turretViewProxyModel` (String)
    #[serde(default)]
    pub turret_view_proxy_model: String,
    /// `turretViewLengthRatio` (Single)
    #[serde(default)]
    pub turret_view_length_ratio: f32,
    /// `turretViewWidthRatio` (Single)
    #[serde(default)]
    pub turret_view_width_ratio: f32,
    /// `turretViewPitch` (Single)
    #[serde(default)]
    pub turret_view_pitch: f32,
    /// `turretViewRoll` (Single)
    #[serde(default)]
    pub turret_view_roll: f32,
    /// `turretViewYaw` (Single)
    #[serde(default)]
    pub turret_view_yaw: f32,
    /// `turretAimProxyModel` (String)
    #[serde(default)]
    pub turret_aim_proxy_model: String,
    /// `turretAimWidthRatio` (Single)
    #[serde(default)]
    pub turret_aim_width_ratio: f32,
    /// `cameraFOV` (Single)
    #[serde(default)]
    pub camera_fov: f32,
    /// `ownCameraDistanceScaler` (Single)
    #[serde(default)]
    pub own_camera_distance_scaler: f32,
    /// `ownAngularOffsetRange` (Single)
    #[serde(default)]
    pub own_angular_offset_range: f32,
    /// `ownDefaultViewAngle` (Single)
    #[serde(default)]
    pub own_default_view_angle: f32,
    /// `ownTranslationSmoothingTime` (Single)
    #[serde(default)]
    pub own_translation_smoothing_time: f32,
    /// `ownTranslationOffsetRadiusRatio` (Single)
    #[serde(default)]
    pub own_translation_offset_radius_ratio: f32,
    /// `hitMaterialDarkenFactor` (Single)
    #[serde(default)]
    pub hit_material_darken_factor: f32,
    /// `hitIndicatorAnimTime` (Single)
    #[serde(default)]
    pub hit_indicator_anim_time: f32,
    /// `targetCameraDistanceScaler` (Single)
    #[serde(default)]
    pub target_camera_distance_scaler: f32,
    /// `cameraIntroTime` (Single)
    #[serde(default)]
    pub camera_intro_time: f32,
    /// `cameraIntroDistanceScaler` (Single)
    #[serde(default)]
    pub camera_intro_distance_scaler: f32,
    /// `cameraIntroInterpolationMode` (EnumChoice)
    #[serde(default)]
    pub camera_intro_interpolation_mode: String,
    /// `cameraViewsTransitionTime` (Single)
    #[serde(default)]
    pub camera_views_transition_time: f32,
    /// `cameraViewTransitionMode` (EnumChoice)
    #[serde(default)]
    pub camera_view_transition_mode: String,
    /// `vehicleMaterial` (String)
    #[serde(default)]
    pub vehicle_material: String,
    /// `itemHighlightMaterial` (String)
    #[serde(default)]
    pub item_highlight_material: String,
    /// `shieldMaterial` (String)
    #[serde(default)]
    pub shield_material: String,
    /// `turretViewStandbyMaterial` (String)
    #[serde(default)]
    pub turret_view_standby_material: String,
    /// `turretViewFireMaterial` (String)
    #[serde(default)]
    pub turret_view_fire_material: String,
    /// `turretAimMaterial` (String)
    #[serde(default)]
    pub turret_aim_material: String,
    /// `vehicleHitMaterial` (String)
    #[serde(default)]
    pub vehicle_hit_material: String,
    /// `shieldHitMaterial` (String)
    #[serde(default)]
    pub shield_hit_material: String,
    /// `itemTypeWhitelist` (EnumChoice (array))
    #[serde(default)]
    pub item_type_whitelist: Vec<String>,
    /// `silhouetteParams` (Class)
    #[serde(default)]
    pub silhouette_params: Option<Handle<SSilhouetteParamsDef>>,
    /// `directionArrowGeomName` (String)
    #[serde(default)]
    pub direction_arrow_geom_name: String,
    /// `directionArrowTipGeomName` (String)
    #[serde(default)]
    pub direction_arrow_tip_geom_name: String,
    /// `directionArrowScale` (Single)
    #[serde(default)]
    pub direction_arrow_scale: f32,
    /// `directionArrowForwardOffset` (Single)
    #[serde(default)]
    pub direction_arrow_forward_offset: f32,
    /// `directionArrowUpOffset` (Single)
    #[serde(default)]
    pub direction_arrow_up_offset: f32,
    /// `directionArrowRightOffset` (Single)
    #[serde(default)]
    pub direction_arrow_right_offset: f32,
}

impl Pooled for UIHoloVehicle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_holovehicleconfig.uiholo_vehicle_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_holovehicleconfig.uiholo_vehicle_config }
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

