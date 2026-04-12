// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `cameras`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Camera`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    /// `baseConfig` (StrongPointer)
    #[serde(default)]
    pub base_config: Option<Handle<CameraBaseConfig>>,
}

impl Pooled for Camera {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera }
}

impl<'a> Extract<'a> for Camera {
    const TYPE_NAME: &'static str = "Camera";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_config: match inst.get("baseConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraBaseConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraBaseSettingsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraBaseSettingsConfig {
    /// `attachmentName` (String)
    #[serde(default)]
    pub attachment_name: String,
    /// `defaultCamera` (Boolean)
    #[serde(default)]
    pub default_camera: bool,
    /// `enterExitCamera` (Boolean)
    #[serde(default)]
    pub enter_exit_camera: bool,
    /// `unregisterAfterEnter` (Boolean)
    #[serde(default)]
    pub unregister_after_enter: bool,
    /// `backgroundUpdate` (Boolean)
    #[serde(default)]
    pub background_update: bool,
}

impl Pooled for CameraBaseSettingsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_base_settings_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_base_settings_config }
}

impl<'a> Extract<'a> for CameraBaseSettingsConfig {
    const TYPE_NAME: &'static str = "CameraBaseSettingsConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attachment_name: inst.get_str("attachmentName").map(String::from).unwrap_or_default(),
            default_camera: inst.get_bool("defaultCamera").unwrap_or_default(),
            enter_exit_camera: inst.get_bool("enterExitCamera").unwrap_or_default(),
            unregister_after_enter: inst.get_bool("unregisterAfterEnter").unwrap_or_default(),
            background_update: inst.get_bool("backgroundUpdate").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraBlendConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraBlendConfig {
    /// `allowBlendFrom` (Boolean)
    #[serde(default)]
    pub allow_blend_from: bool,
    /// `allowBlendTo` (Boolean)
    #[serde(default)]
    pub allow_blend_to: bool,
    /// `blendingToTime` (Single)
    #[serde(default)]
    pub blending_to_time: f32,
}

impl Pooled for CameraBlendConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_blend_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_blend_config }
}

impl<'a> Extract<'a> for CameraBlendConfig {
    const TYPE_NAME: &'static str = "CameraBlendConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow_blend_from: inst.get_bool("allowBlendFrom").unwrap_or_default(),
            allow_blend_to: inst.get_bool("allowBlendTo").unwrap_or_default(),
            blending_to_time: inst.get_f32("blendingToTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraFOVConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFOVConfig {
    /// `lensSizeTransitionTime` (Single)
    #[serde(default)]
    pub lens_size_transition_time: f32,
    /// `defaultLensSizePreset` (EnumChoice)
    #[serde(default)]
    pub default_lens_size_preset: String,
    /// `fStopTransitionTime` (Single)
    #[serde(default)]
    pub f_stop_transition_time: f32,
    /// `defaultFStop` (EnumChoice)
    #[serde(default)]
    pub default_fstop: String,
    /// `nearPlane` (Single)
    #[serde(default)]
    pub near_plane: f32,
    /// `farPlane` (Single)
    #[serde(default)]
    pub far_plane: f32,
    /// `focalDistance` (Single)
    #[serde(default)]
    pub focal_distance: f32,
}

impl Pooled for CameraFOVConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_fovconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_fovconfig }
}

impl<'a> Extract<'a> for CameraFOVConfig {
    const TYPE_NAME: &'static str = "CameraFOVConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            lens_size_transition_time: inst.get_f32("lensSizeTransitionTime").unwrap_or_default(),
            default_lens_size_preset: inst.get_str("defaultLensSizePreset").map(String::from).unwrap_or_default(),
            f_stop_transition_time: inst.get_f32("fStopTransitionTime").unwrap_or_default(),
            default_fstop: inst.get_str("defaultFStop").map(String::from).unwrap_or_default(),
            near_plane: inst.get_f32("nearPlane").unwrap_or_default(),
            far_plane: inst.get_f32("farPlane").unwrap_or_default(),
            focal_distance: inst.get_f32("focalDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraBaseConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraBaseConfig {
    /// `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
}

impl Pooled for CameraBaseConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_base_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_base_config }
}

impl<'a> Extract<'a> for CameraBaseConfig {
    const TYPE_NAME: &'static str = "CameraBaseConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraBlendConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraFOVConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraShopItemOffset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraShopItemOffset {
    /// `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
}

impl Pooled for CameraShopItemOffset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_shop_item_offset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_shop_item_offset }
}

impl<'a> Extract<'a> for CameraShopItemOffset {
    const TYPE_NAME: &'static str = "CameraShopItemOffset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraShopConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraShopConfig {
    /// `initialPositionOffset` (Class)
    #[serde(default)]
    pub initial_position_offset: Option<Handle<Vec3>>,
    /// `itemOffsets` (Class (array))
    #[serde(default)]
    pub item_offsets: Vec<Handle<CameraShopItemOffset>>,
    /// `minVerticalRotationAngle` (Single)
    #[serde(default)]
    pub min_vertical_rotation_angle: f32,
    /// `maxVerticalRotationAngle` (Single)
    #[serde(default)]
    pub max_vertical_rotation_angle: f32,
    /// `minHorizontalRotationAngle` (Single)
    #[serde(default)]
    pub min_horizontal_rotation_angle: f32,
    /// `maxHorizontalRotationAngle` (Single)
    #[serde(default)]
    pub max_horizontal_rotation_angle: f32,
    /// `inTranslationSpeed` (Single)
    #[serde(default)]
    pub in_translation_speed: f32,
    /// `outTranslationSpeed` (Single)
    #[serde(default)]
    pub out_translation_speed: f32,
    /// `inRotationSpeed` (Single)
    #[serde(default)]
    pub in_rotation_speed: f32,
    /// `outRotationSpeed` (Single)
    #[serde(default)]
    pub out_rotation_speed: f32,
    /// `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// `zoomMin` (Single)
    #[serde(default)]
    pub zoom_min: f32,
    /// `zoomMax` (Single)
    #[serde(default)]
    pub zoom_max: f32,
    /// `zoomSpeed` (Single)
    #[serde(default)]
    pub zoom_speed: f32,
    /// `twirlSpeed` (Single)
    #[serde(default)]
    pub twirl_speed: f32,
    /// `timeToActivateTwirl` (Single)
    #[serde(default)]
    pub time_to_activate_twirl: f32,
}

impl Pooled for CameraShopConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_shop_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_shop_config }
}

impl<'a> Extract<'a> for CameraShopConfig {
    const TYPE_NAME: &'static str = "CameraShopConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            initial_position_offset: match inst.get("initialPositionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_offsets: inst.get_array("itemOffsets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CameraShopItemOffset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CameraShopItemOffset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            min_vertical_rotation_angle: inst.get_f32("minVerticalRotationAngle").unwrap_or_default(),
            max_vertical_rotation_angle: inst.get_f32("maxVerticalRotationAngle").unwrap_or_default(),
            min_horizontal_rotation_angle: inst.get_f32("minHorizontalRotationAngle").unwrap_or_default(),
            max_horizontal_rotation_angle: inst.get_f32("maxHorizontalRotationAngle").unwrap_or_default(),
            in_translation_speed: inst.get_f32("inTranslationSpeed").unwrap_or_default(),
            out_translation_speed: inst.get_f32("outTranslationSpeed").unwrap_or_default(),
            in_rotation_speed: inst.get_f32("inRotationSpeed").unwrap_or_default(),
            out_rotation_speed: inst.get_f32("outRotationSpeed").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            zoom_min: inst.get_f32("zoomMin").unwrap_or_default(),
            zoom_max: inst.get_f32("zoomMax").unwrap_or_default(),
            zoom_speed: inst.get_f32("zoomSpeed").unwrap_or_default(),
            twirl_speed: inst.get_f32("twirlSpeed").unwrap_or_default(),
            time_to_activate_twirl: inst.get_f32("timeToActivateTwirl").unwrap_or_default(),
        }
    }
}

/// DCB type: `SuggestedFOVSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFOVSetup {
    /// `suggestedFOV` (Single)
    #[serde(default)]
    pub suggested_fov: f32,
    /// `mode` (EnumChoice)
    #[serde(default)]
    pub mode: String,
}

impl Pooled for SuggestedFOVSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.suggested_fovsetup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.suggested_fovsetup }
}

impl<'a> Extract<'a> for SuggestedFOVSetup {
    const TYPE_NAME: &'static str = "SuggestedFOVSetup";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            suggested_fov: inst.get_f32("suggestedFOV").unwrap_or_default(),
            mode: inst.get_str("mode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CinematicCameraControllerSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicCameraControllerSetup {
    /// `actionHoldTime` (Single)
    #[serde(default)]
    pub action_hold_time: f32,
    /// `expiryLingerTime` (Single)
    #[serde(default)]
    pub expiry_linger_time: f32,
}

impl Pooled for CinematicCameraControllerSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.cinematic_camera_controller_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.cinematic_camera_controller_setup }
}

impl<'a> Extract<'a> for CinematicCameraControllerSetup {
    const TYPE_NAME: &'static str = "CinematicCameraControllerSetup";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            action_hold_time: inst.get_f32("actionHoldTime").unwrap_or_default(),
            expiry_linger_time: inst.get_f32("expiryLingerTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraFOVChangeData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFOVChangeData {
    /// `fovLerpSpeed` (Single)
    #[serde(default)]
    pub fov_lerp_speed: f32,
    /// `resetFOVLerpSpeed` (Single)
    #[serde(default)]
    pub reset_fovlerp_speed: f32,
}

impl Pooled for CameraFOVChangeData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_fovchange_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_fovchange_data }
}

impl<'a> Extract<'a> for CameraFOVChangeData {
    const TYPE_NAME: &'static str = "CameraFOVChangeData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fov_lerp_speed: inst.get_f32("fovLerpSpeed").unwrap_or_default(),
            reset_fovlerp_speed: inst.get_f32("resetFOVLerpSpeed").unwrap_or_default(),
        }
    }
}

