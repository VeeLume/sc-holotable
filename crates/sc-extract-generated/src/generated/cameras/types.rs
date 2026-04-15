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
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `CameraLookBehindConfig`
/// Inherits from: `CameraBaseConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLookBehindConfig {
    /// `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
    /// `distanceConfig` (Class)
    #[serde(default)]
    pub distance_config: Option<Handle<CameraDistanceConfig>>,
    /// `heightOverride` (Single)
    #[serde(default)]
    pub height_override: f32,
    /// `screenOverlayTextureName` (String)
    #[serde(default)]
    pub screen_overlay_texture_name: String,
    /// `introEffectLibrary` (String)
    #[serde(default)]
    pub intro_effect_library: String,
    /// `introEffectName` (String)
    #[serde(default)]
    pub intro_effect_name: String,
    /// `outroEffectLibrary` (String)
    #[serde(default)]
    pub outro_effect_library: String,
    /// `outroEffectName` (String)
    #[serde(default)]
    pub outro_effect_name: String,
}

impl Pooled for CameraLookBehindConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_look_behind_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_look_behind_config }
}

impl<'a> Extract<'a> for CameraLookBehindConfig {
    const TYPE_NAME: &'static str = "CameraLookBehindConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distance_config: match inst.get("distanceConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraDistanceConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            height_override: inst.get_f32("heightOverride").unwrap_or_default(),
            screen_overlay_texture_name: inst.get_str("screenOverlayTextureName").map(String::from).unwrap_or_default(),
            intro_effect_library: inst.get_str("introEffectLibrary").map(String::from).unwrap_or_default(),
            intro_effect_name: inst.get_str("introEffectName").map(String::from).unwrap_or_default(),
            outro_effect_library: inst.get_str("outroEffectLibrary").map(String::from).unwrap_or_default(),
            outro_effect_name: inst.get_str("outroEffectName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraTimeCamConfig`
/// Inherits from: `CameraFreeCamConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTimeCamConfig {
    /// `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
    /// `operatorShake` (Class)
    #[serde(default)]
    pub operator_shake: Option<Handle<CameraShakeConfig>>,
    /// `speedShake` (Class)
    #[serde(default)]
    pub speed_shake: Option<Handle<CameraSpeedShakeConfig>>,
    /// `gforceShake` (Class)
    #[serde(default)]
    pub gforce_shake: Option<Handle<CameraGForceShakeConfig>>,
    /// `raindropsEnabled` (Boolean)
    #[serde(default)]
    pub raindrops_enabled: bool,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `movementSpeed` (Single)
    #[serde(default)]
    pub movement_speed: f32,
    /// `sprintSpeed` (Single)
    #[serde(default)]
    pub sprint_speed: f32,
    /// `freeCamRotationScale` (Single)
    #[serde(default)]
    pub free_cam_rotation_scale: f32,
    /// `DOFFocusDistanceConfig` (Class)
    #[serde(default)]
    pub doffocus_distance_config: Option<Handle<CameraDOFFocusDistanceConfig>>,
    /// `collisionConfig` (Class)
    #[serde(default)]
    pub collision_config: Option<Handle<CameraCollisionConfig>>,
}

impl Pooled for CameraTimeCamConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_time_cam_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_time_cam_config }
}

impl<'a> Extract<'a> for CameraTimeCamConfig {
    const TYPE_NAME: &'static str = "CameraTimeCamConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            operator_shake: match inst.get("operatorShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            speed_shake: match inst.get("speedShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraSpeedShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            gforce_shake: match inst.get("gforceShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraGForceShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            raindrops_enabled: inst.get_bool("raindropsEnabled").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            movement_speed: inst.get_f32("movementSpeed").unwrap_or_default(),
            sprint_speed: inst.get_f32("sprintSpeed").unwrap_or_default(),
            free_cam_rotation_scale: inst.get_f32("freeCamRotationScale").unwrap_or_default(),
            doffocus_distance_config: match inst.get("DOFFocusDistanceConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraDOFFocusDistanceConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            collision_config: match inst.get("collisionConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraCollisionConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `OrbitEntityCinematicEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitEntityCinematicEvent {
    /// `triggerTime` (Single)
    #[serde(default)]
    pub trigger_time: f32,
    /// `loadViewIndex` (Int32)
    #[serde(default)]
    pub load_view_index: i32,
}

impl Pooled for OrbitEntityCinematicEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.orbit_entity_cinematic_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.orbit_entity_cinematic_event }
}

impl<'a> Extract<'a> for OrbitEntityCinematicEvent {
    const TYPE_NAME: &'static str = "OrbitEntityCinematicEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger_time: inst.get_f32("triggerTime").unwrap_or_default(),
            load_view_index: inst.get_i32("loadViewIndex").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraOrbitEntityCinematicConfig`
/// Inherits from: `CameraOrbitEntityConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraOrbitEntityCinematicConfig {
    /// `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
    /// `operatorShake` (Class)
    #[serde(default)]
    pub operator_shake: Option<Handle<CameraShakeConfig>>,
    /// `speedShake` (Class)
    #[serde(default)]
    pub speed_shake: Option<Handle<CameraSpeedShakeConfig>>,
    /// `gforceShake` (Class)
    #[serde(default)]
    pub gforce_shake: Option<Handle<CameraGForceShakeConfig>>,
    /// `raindropsEnabled` (Boolean)
    #[serde(default)]
    pub raindrops_enabled: bool,
    /// `rotationConfig` (Class)
    #[serde(default)]
    pub rotation_config: Option<Handle<CameraRotationConfig>>,
    /// `distanceConfig` (Class)
    #[serde(default)]
    pub distance_config: Option<Handle<CameraDistanceConfig>>,
    /// `targetOffsetConfig` (Class)
    #[serde(default)]
    pub target_offset_config: Option<Handle<CameraTargetOffsetConfig>>,
    /// `springConfig` (Class)
    #[serde(default)]
    pub spring_config: Option<Handle<CameraSpringConfig>>,
    /// `collisionConfig` (Class)
    #[serde(default)]
    pub collision_config: Option<Handle<CameraCollisionConfig>>,
    /// `defaultViews` (Class (array))
    #[serde(default)]
    pub default_views: Vec<Handle<SCameraViewStateOrbit>>,
    /// `alternativeViewsConfig` (Class)
    #[serde(default)]
    pub alternative_views_config: Option<Handle<CameraAlternativeViewsConfig>>,
    /// `actorOffsetConfig` (Class (array))
    #[serde(default)]
    pub actor_offset_config: Vec<Handle<ActorCameraOffsetFiltered>>,
    /// `cinematicEvents` (Class (array))
    #[serde(default)]
    pub cinematic_events: Vec<Handle<OrbitEntityCinematicEvent>>,
    /// `expiryViewIndex` (Int32)
    #[serde(default)]
    pub expiry_view_index: i32,
    /// `expiryAutomaticDollySpeed` (Single)
    #[serde(default)]
    pub expiry_automatic_dolly_speed: f32,
}

impl Pooled for CameraOrbitEntityCinematicConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_orbit_entity_cinematic_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_orbit_entity_cinematic_config }
}

impl<'a> Extract<'a> for CameraOrbitEntityCinematicConfig {
    const TYPE_NAME: &'static str = "CameraOrbitEntityCinematicConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            operator_shake: match inst.get("operatorShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            speed_shake: match inst.get("speedShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraSpeedShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            gforce_shake: match inst.get("gforceShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraGForceShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            raindrops_enabled: inst.get_bool("raindropsEnabled").unwrap_or_default(),
            rotation_config: match inst.get("rotationConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraRotationConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distance_config: match inst.get("distanceConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraDistanceConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            target_offset_config: match inst.get("targetOffsetConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraTargetOffsetConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spring_config: match inst.get("springConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraSpringConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            collision_config: match inst.get("collisionConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraCollisionConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_views: inst.get_array("defaultViews")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCameraViewStateOrbit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCameraViewStateOrbit>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            alternative_views_config: match inst.get("alternativeViewsConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraAlternativeViewsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            actor_offset_config: inst.get_array("actorOffsetConfig")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorCameraOffsetFiltered>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ActorCameraOffsetFiltered>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cinematic_events: inst.get_array("cinematicEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<OrbitEntityCinematicEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<OrbitEntityCinematicEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            expiry_view_index: inst.get_i32("expiryViewIndex").unwrap_or_default(),
            expiry_automatic_dolly_speed: inst.get_f32("expiryAutomaticDollySpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraOrbitFPSDeathCamConfig`
/// Inherits from: `CameraOrbitEntityConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraOrbitFPSDeathCamConfig {
    /// `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
    /// `operatorShake` (Class)
    #[serde(default)]
    pub operator_shake: Option<Handle<CameraShakeConfig>>,
    /// `speedShake` (Class)
    #[serde(default)]
    pub speed_shake: Option<Handle<CameraSpeedShakeConfig>>,
    /// `gforceShake` (Class)
    #[serde(default)]
    pub gforce_shake: Option<Handle<CameraGForceShakeConfig>>,
    /// `raindropsEnabled` (Boolean)
    #[serde(default)]
    pub raindrops_enabled: bool,
    /// `rotationConfig` (Class)
    #[serde(default)]
    pub rotation_config: Option<Handle<CameraRotationConfig>>,
    /// `distanceConfig` (Class)
    #[serde(default)]
    pub distance_config: Option<Handle<CameraDistanceConfig>>,
    /// `targetOffsetConfig` (Class)
    #[serde(default)]
    pub target_offset_config: Option<Handle<CameraTargetOffsetConfig>>,
    /// `springConfig` (Class)
    #[serde(default)]
    pub spring_config: Option<Handle<CameraSpringConfig>>,
    /// `collisionConfig` (Class)
    #[serde(default)]
    pub collision_config: Option<Handle<CameraCollisionConfig>>,
    /// `defaultViews` (Class (array))
    #[serde(default)]
    pub default_views: Vec<Handle<SCameraViewStateOrbit>>,
    /// `alternativeViewsConfig` (Class)
    #[serde(default)]
    pub alternative_views_config: Option<Handle<CameraAlternativeViewsConfig>>,
    /// `actorOffsetConfig` (Class (array))
    #[serde(default)]
    pub actor_offset_config: Vec<Handle<ActorCameraOffsetFiltered>>,
}

impl Pooled for CameraOrbitFPSDeathCamConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_orbit_fpsdeath_cam_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_orbit_fpsdeath_cam_config }
}

impl<'a> Extract<'a> for CameraOrbitFPSDeathCamConfig {
    const TYPE_NAME: &'static str = "CameraOrbitFPSDeathCamConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            operator_shake: match inst.get("operatorShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            speed_shake: match inst.get("speedShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraSpeedShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            gforce_shake: match inst.get("gforceShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraGForceShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            raindrops_enabled: inst.get_bool("raindropsEnabled").unwrap_or_default(),
            rotation_config: match inst.get("rotationConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraRotationConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distance_config: match inst.get("distanceConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraDistanceConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            target_offset_config: match inst.get("targetOffsetConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraTargetOffsetConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spring_config: match inst.get("springConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraSpringConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            collision_config: match inst.get("collisionConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraCollisionConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_views: inst.get_array("defaultViews")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCameraViewStateOrbit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCameraViewStateOrbit>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            alternative_views_config: match inst.get("alternativeViewsConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraAlternativeViewsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            actor_offset_config: inst.get_array("actorOffsetConfig")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorCameraOffsetFiltered>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ActorCameraOffsetFiltered>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraDockingConfig`
/// Inherits from: `CameraBaseConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraDockingConfig {
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

impl Pooled for CameraDockingConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_docking_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_docking_config }
}

impl<'a> Extract<'a> for CameraDockingConfig {
    const TYPE_NAME: &'static str = "CameraDockingConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
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
    pub item_type: EItemType,
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
            item_type: EItemType::from_dcb_str(inst.get_str("itemType").unwrap_or("")),
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
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
                _ => None,
            },
            item_offsets: inst.get_array("itemOffsets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CameraShopItemOffset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<CameraShopItemOffset>(b.db.instance(r.struct_index, r.instance_index), true)),
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

/// DCB type: `CameraStaticConfig`
/// Inherits from: `CameraBaseConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraStaticConfig {
    /// `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
    /// `targetOffsetConfig` (Class)
    #[serde(default)]
    pub target_offset_config: Option<Handle<CameraTargetOffsetConfig>>,
    /// `focusTargetAttachmentName` (String)
    #[serde(default)]
    pub focus_target_attachment_name: String,
    /// `targetAttachmentPositionOffset` (Class)
    #[serde(default)]
    pub target_attachment_position_offset: Option<Handle<Vec3>>,
    /// `targetAttachmentLerpFactor` (Single)
    #[serde(default)]
    pub target_attachment_lerp_factor: f32,
}

impl Pooled for CameraStaticConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_static_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_static_config }
}

impl<'a> Extract<'a> for CameraStaticConfig {
    const TYPE_NAME: &'static str = "CameraStaticConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            target_offset_config: match inst.get("targetOffsetConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraTargetOffsetConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            focus_target_attachment_name: inst.get_str("focusTargetAttachmentName").map(String::from).unwrap_or_default(),
            target_attachment_position_offset: match inst.get("targetAttachmentPositionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            target_attachment_lerp_factor: inst.get_f32("targetAttachmentLerpFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraView2ShipsFrameParams`
/// Inherits from: `CameraThirdPersonBaseConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraView2ShipsFrameParams {
    /// `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
    /// `operatorShake` (Class)
    #[serde(default)]
    pub operator_shake: Option<Handle<CameraShakeConfig>>,
    /// `speedShake` (Class)
    #[serde(default)]
    pub speed_shake: Option<Handle<CameraSpeedShakeConfig>>,
    /// `gforceShake` (Class)
    #[serde(default)]
    pub gforce_shake: Option<Handle<CameraGForceShakeConfig>>,
    /// `raindropsEnabled` (Boolean)
    #[serde(default)]
    pub raindrops_enabled: bool,
}

impl Pooled for CameraView2ShipsFrameParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_view2_ships_frame_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_view2_ships_frame_params }
}

impl<'a> Extract<'a> for CameraView2ShipsFrameParams {
    const TYPE_NAME: &'static str = "CameraView2ShipsFrameParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            operator_shake: match inst.get("operatorShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            speed_shake: match inst.get("speedShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraSpeedShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            gforce_shake: match inst.get("gforceShake") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraGForceShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            raindrops_enabled: inst.get_bool("raindropsEnabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraTrackviewConfig`
/// Inherits from: `CameraBaseConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTrackviewConfig {
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

impl Pooled for CameraTrackviewConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cameras.camera_trackview_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cameras.camera_trackview_config }
}

impl<'a> Extract<'a> for CameraTrackviewConfig {
    const TYPE_NAME: &'static str = "CameraTrackviewConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_settings: match inst.get("baseSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBaseSettingsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blend_config: match inst.get("blendConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraBlendConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fovconfig: match inst.get("FOVConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraFOVConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
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

