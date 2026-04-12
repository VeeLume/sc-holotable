// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-itempreview_config`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ItemPreview_LightIntensities`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_LightIntensities {
    /// `keyLightTopIntensity` (Single)
    #[serde(default)]
    pub key_light_top_intensity: f32,
    /// `fillLightTopIntensity` (Single)
    #[serde(default)]
    pub fill_light_top_intensity: f32,
    /// `rimLightTopIntensity` (Single)
    #[serde(default)]
    pub rim_light_top_intensity: f32,
    /// `keyLightBottomIntensity` (Single)
    #[serde(default)]
    pub key_light_bottom_intensity: f32,
    /// `fillLightBottomIntensity` (Single)
    #[serde(default)]
    pub fill_light_bottom_intensity: f32,
    /// `rimLightBottomIntensity` (Single)
    #[serde(default)]
    pub rim_light_bottom_intensity: f32,
}

impl Pooled for ItemPreview_LightIntensities {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_light_intensities }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_light_intensities }
}

impl<'a> Extract<'a> for ItemPreview_LightIntensities {
    const TYPE_NAME: &'static str = "ItemPreview_LightIntensities";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            key_light_top_intensity: inst.get_f32("keyLightTopIntensity").unwrap_or_default(),
            fill_light_top_intensity: inst.get_f32("fillLightTopIntensity").unwrap_or_default(),
            rim_light_top_intensity: inst.get_f32("rimLightTopIntensity").unwrap_or_default(),
            key_light_bottom_intensity: inst.get_f32("keyLightBottomIntensity").unwrap_or_default(),
            fill_light_bottom_intensity: inst.get_f32("fillLightBottomIntensity").unwrap_or_default(),
            rim_light_bottom_intensity: inst.get_f32("rimLightBottomIntensity").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_LightingSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_LightingSettings {
    /// `lightIntensities` (Class)
    #[serde(default)]
    pub light_intensities: Option<Handle<ItemPreview_LightIntensities>>,
    /// `topAngle` (Single)
    #[serde(default)]
    pub top_angle: f32,
    /// `bottomAngle` (Single)
    #[serde(default)]
    pub bottom_angle: f32,
    /// `leftRightAngle` (Single)
    #[serde(default)]
    pub left_right_angle: f32,
    /// `rimOffsetAngle` (Single)
    #[serde(default)]
    pub rim_offset_angle: f32,
    /// `useEnvProbe` (Boolean)
    #[serde(default)]
    pub use_env_probe: bool,
    /// `envProbeTexture` (String)
    #[serde(default)]
    pub env_probe_texture: String,
    /// `envProbeMultiplier` (Single)
    #[serde(default)]
    pub env_probe_multiplier: f32,
    /// `envProbeRadiusMultiplier` (Single)
    #[serde(default)]
    pub env_probe_radius_multiplier: f32,
}

impl Pooled for ItemPreview_LightingSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_lighting_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_lighting_settings }
}

impl<'a> Extract<'a> for ItemPreview_LightingSettings {
    const TYPE_NAME: &'static str = "ItemPreview_LightingSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_intensities: match inst.get("lightIntensities") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_LightIntensities>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_LightIntensities>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_angle: inst.get_f32("topAngle").unwrap_or_default(),
            bottom_angle: inst.get_f32("bottomAngle").unwrap_or_default(),
            left_right_angle: inst.get_f32("leftRightAngle").unwrap_or_default(),
            rim_offset_angle: inst.get_f32("rimOffsetAngle").unwrap_or_default(),
            use_env_probe: inst.get_bool("useEnvProbe").unwrap_or_default(),
            env_probe_texture: inst.get_str("envProbeTexture").map(String::from).unwrap_or_default(),
            env_probe_multiplier: inst.get_f32("envProbeMultiplier").unwrap_or_default(),
            env_probe_radius_multiplier: inst.get_f32("envProbeRadiusMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_SkinnedLoadoutOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_SkinnedLoadoutOverride {
    /// `skinnedLoadout` (String)
    #[serde(default)]
    pub skinned_loadout: String,
    /// `itemTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_types: Vec<String>,
}

impl Pooled for ItemPreview_SkinnedLoadoutOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_skinned_loadout_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_skinned_loadout_override }
}

impl<'a> Extract<'a> for ItemPreview_SkinnedLoadoutOverride {
    const TYPE_NAME: &'static str = "ItemPreview_SkinnedLoadoutOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            skinned_loadout: inst.get_str("skinnedLoadout").map(String::from).unwrap_or_default(),
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_CameraSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_CameraSettings {
    /// `bones` (EnumChoice (array))
    #[serde(default)]
    pub bones: Vec<String>,
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `distanceScaler` (Single)
    #[serde(default)]
    pub distance_scaler: f32,
    /// `fieldOfView` (Single)
    #[serde(default)]
    pub field_of_view: f32,
    /// `pitch` (Single)
    #[serde(default)]
    pub pitch: f32,
}

impl Pooled for ItemPreview_CameraSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_camera_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_camera_settings }
}

impl<'a> Extract<'a> for ItemPreview_CameraSettings {
    const TYPE_NAME: &'static str = "ItemPreview_CameraSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bones: inst.get_array("bones")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_scaler: inst.get_f32("distanceScaler").unwrap_or_default(),
            field_of_view: inst.get_f32("fieldOfView").unwrap_or_default(),
            pitch: inst.get_f32("pitch").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_CameraSettingsOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_CameraSettingsOverride {
    /// `settings` (Class)
    #[serde(default)]
    pub settings: Option<Handle<ItemPreview_CameraSettings>>,
    /// `itemTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_types: Vec<String>,
}

impl Pooled for ItemPreview_CameraSettingsOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_camera_settings_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_camera_settings_override }
}

impl<'a> Extract<'a> for ItemPreview_CameraSettingsOverride {
    const TYPE_NAME: &'static str = "ItemPreview_CameraSettingsOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            settings: match inst.get("settings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_TurntableSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_TurntableSettings {
    /// `initialRotation` (Class)
    #[serde(default)]
    pub initial_rotation: Option<Handle<Ang3>>,
    /// `rotationChange` (Class)
    #[serde(default)]
    pub rotation_change: Option<Handle<Vec3>>,
}

impl Pooled for ItemPreview_TurntableSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_turntable_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_turntable_settings }
}

impl<'a> Extract<'a> for ItemPreview_TurntableSettings {
    const TYPE_NAME: &'static str = "ItemPreview_TurntableSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            initial_rotation: match inst.get("initialRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_change: match inst.get("rotationChange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemPreview_TurntableOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_TurntableOverride {
    /// `settings` (Class)
    #[serde(default)]
    pub settings: Option<Handle<ItemPreview_TurntableSettings>>,
    /// `itemTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_types: Vec<String>,
}

impl Pooled for ItemPreview_TurntableOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_turntable_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_turntable_override }
}

impl<'a> Extract<'a> for ItemPreview_TurntableOverride {
    const TYPE_NAME: &'static str = "ItemPreview_TurntableOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            settings: match inst.get("settings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_Config {
    /// `dummyPoseAnim` (String)
    #[serde(default)]
    pub dummy_pose_anim: String,
    /// `dummyBaseClass` (Reference)
    #[serde(default)]
    pub dummy_base_class: Option<CigGuid>,
    /// `skinnedLoadoutDefault` (String)
    #[serde(default)]
    pub skinned_loadout_default: String,
    /// `cameraSettingsDefault` (Class)
    #[serde(default)]
    pub camera_settings_default: Option<Handle<ItemPreview_CameraSettings>>,
    /// `turntableSettingsDefault` (Class)
    #[serde(default)]
    pub turntable_settings_default: Option<Handle<ItemPreview_TurntableSettings>>,
    /// `skinnedLoadoutOverrides` (Class (array))
    #[serde(default)]
    pub skinned_loadout_overrides: Vec<Handle<ItemPreview_SkinnedLoadoutOverride>>,
    /// `cameraSettingsOverrides` (Class (array))
    #[serde(default)]
    pub camera_settings_overrides: Vec<Handle<ItemPreview_CameraSettingsOverride>>,
    /// `turntableSettingsOverrides` (Class (array))
    #[serde(default)]
    pub turntable_settings_overrides: Vec<Handle<ItemPreview_TurntableOverride>>,
    /// `lightingSettings` (Class)
    #[serde(default)]
    pub lighting_settings: Option<Handle<ItemPreview_LightingSettings>>,
    /// `fadeDelay` (Single)
    #[serde(default)]
    pub fade_delay: f32,
    /// `fadeTime` (Single)
    #[serde(default)]
    pub fade_time: f32,
}

impl Pooled for ItemPreview_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itempreview_config.item_preview_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itempreview_config.item_preview_config }
}

impl<'a> Extract<'a> for ItemPreview_Config {
    const TYPE_NAME: &'static str = "ItemPreview_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            dummy_pose_anim: inst.get_str("dummyPoseAnim").map(String::from).unwrap_or_default(),
            dummy_base_class: inst.get("dummyBaseClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            skinned_loadout_default: inst.get_str("skinnedLoadoutDefault").map(String::from).unwrap_or_default(),
            camera_settings_default: match inst.get("cameraSettingsDefault") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turntable_settings_default: match inst.get("turntableSettingsDefault") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            skinned_loadout_overrides: inst.get_array("skinnedLoadoutOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPreview_SkinnedLoadoutOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPreview_SkinnedLoadoutOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            camera_settings_overrides: inst.get_array("cameraSettingsOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPreview_CameraSettingsOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPreview_CameraSettingsOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            turntable_settings_overrides: inst.get_array("turntableSettingsOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPreview_TurntableOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPreview_TurntableOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            lighting_settings: match inst.get("lightingSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_LightingSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_LightingSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fade_delay: inst.get_f32("fadeDelay").unwrap_or_default(),
            fade_time: inst.get_f32("fadeTime").unwrap_or_default(),
        }
    }
}

