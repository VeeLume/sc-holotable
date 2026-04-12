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

/// DCB type: `CameraLensStreak`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensStreak {
    /// DCB field: `Intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// DCB field: `Width` (Single)
    #[serde(default)]
    pub width: f32,
    /// DCB field: `Tint` (Class)
    #[serde(default)]
    pub tint: Option<Handle<RGB>>,
}

impl Pooled for CameraLensStreak {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_lens_streak }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_lens_streak }
}

impl<'a> Extract<'a> for CameraLensStreak {
    const TYPE_NAME: &'static str = "CameraLensStreak";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            intensity: inst.get_f32("Intensity").unwrap_or_default(),
            width: inst.get_f32("Width").unwrap_or_default(),
            tint: match inst.get("Tint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraLensDistortion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensDistortion {
    /// DCB field: `Radial` (Single)
    #[serde(default)]
    pub radial: f32,
    /// DCB field: `Spherical` (Single)
    #[serde(default)]
    pub spherical: f32,
    /// DCB field: `Coma` (Single)
    #[serde(default)]
    pub coma: f32,
    /// DCB field: `Curvature` (Single)
    #[serde(default)]
    pub curvature: f32,
}

impl Pooled for CameraLensDistortion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_lens_distortion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_lens_distortion }
}

impl<'a> Extract<'a> for CameraLensDistortion {
    const TYPE_NAME: &'static str = "CameraLensDistortion";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radial: inst.get_f32("Radial").unwrap_or_default(),
            spherical: inst.get_f32("Spherical").unwrap_or_default(),
            coma: inst.get_f32("Coma").unwrap_or_default(),
            curvature: inst.get_f32("Curvature").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraLensChromaticAberration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensChromaticAberration {
    /// DCB field: `Transverse` (Single)
    #[serde(default)]
    pub transverse: f32,
    /// DCB field: `Axial` (Single)
    #[serde(default)]
    pub axial: f32,
}

impl Pooled for CameraLensChromaticAberration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_lens_chromatic_aberration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_lens_chromatic_aberration }
}

impl<'a> Extract<'a> for CameraLensChromaticAberration {
    const TYPE_NAME: &'static str = "CameraLensChromaticAberration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            transverse: inst.get_f32("Transverse").unwrap_or_default(),
            axial: inst.get_f32("Axial").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraLensGhostInstance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensGhostInstance {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `Position` (Single)
    #[serde(default)]
    pub position: f32,
    /// DCB field: `Intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// DCB field: `Tint` (Class)
    #[serde(default)]
    pub tint: Option<Handle<RGB>>,
}

impl Pooled for CameraLensGhostInstance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_lens_ghost_instance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_lens_ghost_instance }
}

impl<'a> Extract<'a> for CameraLensGhostInstance {
    const TYPE_NAME: &'static str = "CameraLensGhostInstance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            position: inst.get_f32("Position").unwrap_or_default(),
            intensity: inst.get_f32("Intensity").unwrap_or_default(),
            tint: match inst.get("Tint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraLensGhostSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensGhostSet {
    /// DCB field: `SetName` (String)
    #[serde(default)]
    pub set_name: String,
    /// DCB field: `Radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// DCB field: `Distortion` (StrongPointer)
    #[serde(default)]
    pub distortion: Option<Handle<CameraLensDistortion>>,
    /// DCB field: `ChromaticAberration` (StrongPointer)
    #[serde(default)]
    pub chromatic_aberration: Option<Handle<CameraLensChromaticAberration>>,
    /// DCB field: `Instances` (Class (array))
    #[serde(default)]
    pub instances: Vec<Handle<CameraLensGhostInstance>>,
}

impl Pooled for CameraLensGhostSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_lens_ghost_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_lens_ghost_set }
}

impl<'a> Extract<'a> for CameraLensGhostSet {
    const TYPE_NAME: &'static str = "CameraLensGhostSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            set_name: inst.get_str("SetName").map(String::from).unwrap_or_default(),
            radius: inst.get_f32("Radius").unwrap_or_default(),
            distortion: match inst.get("Distortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraLensDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraLensDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chromatic_aberration: match inst.get("ChromaticAberration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraLensChromaticAberration>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraLensChromaticAberration>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            instances: inst.get_array("Instances")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CameraLensGhostInstance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CameraLensGhostInstance>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraLensParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensParams {
    /// DCB field: `BloomIntensity` (Single)
    #[serde(default)]
    pub bloom_intensity: f32,
    /// DCB field: `FlareIntensity` (Single)
    #[serde(default)]
    pub flare_intensity: f32,
    /// DCB field: `Streak` (StrongPointer)
    #[serde(default)]
    pub streak: Option<Handle<CameraLensStreak>>,
    /// DCB field: `GhostSets` (Class (array))
    #[serde(default)]
    pub ghost_sets: Vec<Handle<CameraLensGhostSet>>,
}

impl Pooled for CameraLensParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_lens_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_lens_params }
}

impl<'a> Extract<'a> for CameraLensParams {
    const TYPE_NAME: &'static str = "CameraLensParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bloom_intensity: inst.get_f32("BloomIntensity").unwrap_or_default(),
            flare_intensity: inst.get_f32("FlareIntensity").unwrap_or_default(),
            streak: match inst.get("Streak") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraLensStreak>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraLensStreak>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ghost_sets: inst.get_array("GhostSets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CameraLensGhostSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CameraLensGhostSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Camera`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    /// DCB field: `baseConfig` (StrongPointer)
    #[serde(default)]
    pub base_config: Option<Handle<CameraBaseConfig>>,
}

impl Pooled for Camera {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera }
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
    /// DCB field: `attachmentName` (String)
    #[serde(default)]
    pub attachment_name: String,
    /// DCB field: `defaultCamera` (Boolean)
    #[serde(default)]
    pub default_camera: bool,
    /// DCB field: `enterExitCamera` (Boolean)
    #[serde(default)]
    pub enter_exit_camera: bool,
    /// DCB field: `unregisterAfterEnter` (Boolean)
    #[serde(default)]
    pub unregister_after_enter: bool,
    /// DCB field: `backgroundUpdate` (Boolean)
    #[serde(default)]
    pub background_update: bool,
}

impl Pooled for CameraBaseSettingsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_base_settings_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_base_settings_config }
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
    /// DCB field: `allowBlendFrom` (Boolean)
    #[serde(default)]
    pub allow_blend_from: bool,
    /// DCB field: `allowBlendTo` (Boolean)
    #[serde(default)]
    pub allow_blend_to: bool,
    /// DCB field: `blendingToTime` (Single)
    #[serde(default)]
    pub blending_to_time: f32,
}

impl Pooled for CameraBlendConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_blend_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_blend_config }
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

/// DCB type: `CameraActorVibrationShakeConfig`
///
/// Inherits from: `CameraShakeConfig` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraActorVibrationShakeConfig {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `offsetPosition` (Class)
    #[serde(default)]
    pub offset_position: Option<Handle<Vec3>>,
    /// DCB field: `offsetAngle` (Class)
    #[serde(default)]
    pub offset_angle: Option<Handle<Ang3>>,
    /// DCB field: `timePeriod` (Single)
    #[serde(default)]
    pub time_period: f32,
    /// DCB field: `frequencyNoiseFactor` (Single)
    #[serde(default)]
    pub frequency_noise_factor: f32,
    /// DCB field: `translationNoise` (Single)
    #[serde(default)]
    pub translation_noise: f32,
    /// DCB field: `rotationNoise` (Single)
    #[serde(default)]
    pub rotation_noise: f32,
    /// DCB field: `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// DCB field: `vibrationOutputMapping` (Class)
    #[serde(default)]
    pub vibration_output_mapping: Option<Handle<BezierCurve>>,
    /// DCB field: `processOnlyOnVibrationIncrease` (Boolean)
    #[serde(default)]
    pub process_only_on_vibration_increase: bool,
    /// DCB field: `processOnlyOnVibrationIncreaseDuration` (Single)
    #[serde(default)]
    pub process_only_on_vibration_increase_duration: f32,
    /// DCB field: `processOnlyOnVibrationIncreaseTimeMapping` (Class)
    #[serde(default)]
    pub process_only_on_vibration_increase_time_mapping: Option<Handle<BezierCurve>>,
    /// DCB field: `scaleFirstPerson` (Single)
    #[serde(default)]
    pub scale_first_person: f32,
    /// DCB field: `scaleThirdPerson` (Single)
    #[serde(default)]
    pub scale_third_person: f32,
    /// DCB field: `smoothFactor` (Single)
    #[serde(default)]
    pub smooth_factor: f32,
}

impl Pooled for CameraActorVibrationShakeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_actor_vibration_shake_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_actor_vibration_shake_config }
}

impl<'a> Extract<'a> for CameraActorVibrationShakeConfig {
    const TYPE_NAME: &'static str = "CameraActorVibrationShakeConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            offset_position: match inst.get("offsetPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_angle: match inst.get("offsetAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time_period: inst.get_f32("timePeriod").unwrap_or_default(),
            frequency_noise_factor: inst.get_f32("frequencyNoiseFactor").unwrap_or_default(),
            translation_noise: inst.get_f32("translationNoise").unwrap_or_default(),
            rotation_noise: inst.get_f32("rotationNoise").unwrap_or_default(),
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            vibration_output_mapping: match inst.get("vibrationOutputMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            process_only_on_vibration_increase: inst.get_bool("processOnlyOnVibrationIncrease").unwrap_or_default(),
            process_only_on_vibration_increase_duration: inst.get_f32("processOnlyOnVibrationIncreaseDuration").unwrap_or_default(),
            process_only_on_vibration_increase_time_mapping: match inst.get("processOnlyOnVibrationIncreaseTimeMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scale_first_person: inst.get_f32("scaleFirstPerson").unwrap_or_default(),
            scale_third_person: inst.get_f32("scaleThirdPerson").unwrap_or_default(),
            smooth_factor: inst.get_f32("smoothFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraFOVConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFOVConfig {
    /// DCB field: `lensSizeTransitionTime` (Single)
    #[serde(default)]
    pub lens_size_transition_time: f32,
    /// DCB field: `defaultLensSizePreset` (EnumChoice)
    #[serde(default)]
    pub default_lens_size_preset: String,
    /// DCB field: `fStopTransitionTime` (Single)
    #[serde(default)]
    pub f_stop_transition_time: f32,
    /// DCB field: `defaultFStop` (EnumChoice)
    #[serde(default)]
    pub default_fstop: String,
    /// DCB field: `nearPlane` (Single)
    #[serde(default)]
    pub near_plane: f32,
    /// DCB field: `farPlane` (Single)
    #[serde(default)]
    pub far_plane: f32,
    /// DCB field: `focalDistance` (Single)
    #[serde(default)]
    pub focal_distance: f32,
}

impl Pooled for CameraFOVConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_fovconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_fovconfig }
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
    /// DCB field: `baseSettings` (Class)
    #[serde(default)]
    pub base_settings: Option<Handle<CameraBaseSettingsConfig>>,
    /// DCB field: `blendConfig` (Class)
    #[serde(default)]
    pub blend_config: Option<Handle<CameraBlendConfig>>,
    /// DCB field: `FOVConfig` (Class)
    #[serde(default)]
    pub fovconfig: Option<Handle<CameraFOVConfig>>,
}

impl Pooled for CameraBaseConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_base_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_base_config }
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
    /// DCB field: `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
}

impl Pooled for CameraShopItemOffset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_shop_item_offset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_shop_item_offset }
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
    /// DCB field: `initialPositionOffset` (Class)
    #[serde(default)]
    pub initial_position_offset: Option<Handle<Vec3>>,
    /// DCB field: `itemOffsets` (Class (array))
    #[serde(default)]
    pub item_offsets: Vec<Handle<CameraShopItemOffset>>,
    /// DCB field: `minVerticalRotationAngle` (Single)
    #[serde(default)]
    pub min_vertical_rotation_angle: f32,
    /// DCB field: `maxVerticalRotationAngle` (Single)
    #[serde(default)]
    pub max_vertical_rotation_angle: f32,
    /// DCB field: `minHorizontalRotationAngle` (Single)
    #[serde(default)]
    pub min_horizontal_rotation_angle: f32,
    /// DCB field: `maxHorizontalRotationAngle` (Single)
    #[serde(default)]
    pub max_horizontal_rotation_angle: f32,
    /// DCB field: `inTranslationSpeed` (Single)
    #[serde(default)]
    pub in_translation_speed: f32,
    /// DCB field: `outTranslationSpeed` (Single)
    #[serde(default)]
    pub out_translation_speed: f32,
    /// DCB field: `inRotationSpeed` (Single)
    #[serde(default)]
    pub in_rotation_speed: f32,
    /// DCB field: `outRotationSpeed` (Single)
    #[serde(default)]
    pub out_rotation_speed: f32,
    /// DCB field: `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// DCB field: `zoomMin` (Single)
    #[serde(default)]
    pub zoom_min: f32,
    /// DCB field: `zoomMax` (Single)
    #[serde(default)]
    pub zoom_max: f32,
    /// DCB field: `zoomSpeed` (Single)
    #[serde(default)]
    pub zoom_speed: f32,
    /// DCB field: `twirlSpeed` (Single)
    #[serde(default)]
    pub twirl_speed: f32,
    /// DCB field: `timeToActivateTwirl` (Single)
    #[serde(default)]
    pub time_to_activate_twirl: f32,
}

impl Pooled for CameraShopConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_shop_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_shop_config }
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

/// DCB type: `CameraFOVChangeData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFOVChangeData {
    /// DCB field: `fovLerpSpeed` (Single)
    #[serde(default)]
    pub fov_lerp_speed: f32,
    /// DCB field: `resetFOVLerpSpeed` (Single)
    #[serde(default)]
    pub reset_fovlerp_speed: f32,
}

impl Pooled for CameraFOVChangeData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_fovchange_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_fovchange_data }
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

/// DCB type: `CargoManifest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoManifest {
    /// DCB field: `descriptionTags` (Class)
    #[serde(default)]
    pub description_tags: Option<Handle<TagList>>,
    /// DCB field: `cargoFillCapacity` (StrongPointer)
    #[serde(default)]
    pub cargo_fill_capacity: Option<Handle<BaseCargoFillCapacityValue>>,
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for CargoManifest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.cargo_manifest }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.cargo_manifest }
}

impl<'a> Extract<'a> for CargoManifest {
    const TYPE_NAME: &'static str = "CargoManifest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description_tags: match inst.get("descriptionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cargo_fill_capacity: match inst.get("cargoFillCapacity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseCargoFillCapacityValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseCargoFillCapacityValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CarryConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryConfig {
    /// DCB field: `overArmThrow` (Class)
    #[serde(default)]
    pub over_arm_throw: Option<Handle<ThrowParams>>,
    /// DCB field: `underArmThrow` (Class)
    #[serde(default)]
    pub under_arm_throw: Option<Handle<ThrowParams>>,
    /// DCB field: `twoHandedThrow` (Class)
    #[serde(default)]
    pub two_handed_throw: Option<Handle<ThrowParams>>,
    /// DCB field: `carry` (Class)
    #[serde(default)]
    pub carry: Option<Handle<ItemCarryParams>>,
    /// DCB field: `reloadRummageDuration` (Single)
    #[serde(default)]
    pub reload_rummage_duration: f32,
    /// DCB field: `EVAHandReachPlaceDistance` (Single)
    #[serde(default)]
    pub evahand_reach_place_distance: f32,
    /// DCB field: `tagSwitches` (Class (array))
    #[serde(default)]
    pub tag_switches: Vec<Handle<SActorCarryConfigTagSwitch>>,
    /// DCB field: `dropHeldItemHandOffset` (Class)
    #[serde(default)]
    pub drop_held_item_hand_offset: Option<Handle<Vec3>>,
    /// DCB field: `dropEquippedItemHandOffset` (Class)
    #[serde(default)]
    pub drop_equipped_item_hand_offset: Option<Handle<Vec3>>,
}

impl Pooled for CarryConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.carry_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.carry_config }
}

impl<'a> Extract<'a> for CarryConfig {
    const TYPE_NAME: &'static str = "CarryConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            over_arm_throw: match inst.get("overArmThrow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ThrowParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ThrowParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            under_arm_throw: match inst.get("underArmThrow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ThrowParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ThrowParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            two_handed_throw: match inst.get("twoHandedThrow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ThrowParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ThrowParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            carry: match inst.get("carry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemCarryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemCarryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reload_rummage_duration: inst.get_f32("reloadRummageDuration").unwrap_or_default(),
            evahand_reach_place_distance: inst.get_f32("EVAHandReachPlaceDistance").unwrap_or_default(),
            tag_switches: inst.get_array("tagSwitches")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorCarryConfigTagSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorCarryConfigTagSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            drop_held_item_hand_offset: match inst.get("dropHeldItemHandOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drop_equipped_item_hand_offset: match inst.get("dropEquippedItemHandOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraEffectsModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraEffectsModifiers {
    /// DCB field: `smoothing` (Single)
    #[serde(default)]
    pub smoothing: f32,
    /// DCB field: `chromaticAbberation` (Single)
    #[serde(default)]
    pub chromatic_abberation: f32,
    /// DCB field: `fStopMin` (Single)
    #[serde(default)]
    pub f_stop_min: f32,
    /// DCB field: `fStopMax` (Single)
    #[serde(default)]
    pub f_stop_max: f32,
    /// DCB field: `fovMapping` (Class)
    #[serde(default)]
    pub fov_mapping: Option<Handle<BezierCurve>>,
    /// DCB field: `suggestedFOVMode` (EnumChoice)
    #[serde(default)]
    pub suggested_fovmode: String,
    /// DCB field: `cameraEffectsMapping` (Class)
    #[serde(default)]
    pub camera_effects_mapping: Option<Handle<BezierCurve>>,
    /// DCB field: `opticsLensPreset` (Reference)
    #[serde(default)]
    pub optics_lens_preset: Option<CigGuid>,
}

impl Pooled for CameraEffectsModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_effects_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_effects_modifiers }
}

impl<'a> Extract<'a> for CameraEffectsModifiers {
    const TYPE_NAME: &'static str = "CameraEffectsModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            smoothing: inst.get_f32("smoothing").unwrap_or_default(),
            chromatic_abberation: inst.get_f32("chromaticAbberation").unwrap_or_default(),
            f_stop_min: inst.get_f32("fStopMin").unwrap_or_default(),
            f_stop_max: inst.get_f32("fStopMax").unwrap_or_default(),
            fov_mapping: match inst.get("fovMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            suggested_fovmode: inst.get_str("suggestedFOVMode").map(String::from).unwrap_or_default(),
            camera_effects_mapping: match inst.get("cameraEffectsMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            optics_lens_preset: inst.get("opticsLensPreset").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `CarryableInteractionsMetadataDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryableInteractionsMetadataDef {
    /// DCB field: `ignoreDefaultActionWhenBlocked` (Boolean)
    #[serde(default)]
    pub ignore_default_action_when_blocked: bool,
}

impl Pooled for CarryableInteractionsMetadataDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.carryable_interactions_metadata_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.carryable_interactions_metadata_def }
}

impl<'a> Extract<'a> for CarryableInteractionsMetadataDef {
    const TYPE_NAME: &'static str = "CarryableInteractionsMetadataDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ignore_default_action_when_blocked: inst.get_bool("ignoreDefaultActionWhenBlocked").unwrap_or_default(),
        }
    }
}

/// DCB type: `CarryableInteractionsMetadataConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryableInteractionsMetadataConfigDef {
    /// DCB field: `metadata` (StrongPointer)
    #[serde(default)]
    pub metadata: Option<Handle<CarryableInteractionsMetadataDef>>,
}

impl Pooled for CarryableInteractionsMetadataConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.carryable_interactions_metadata_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.carryable_interactions_metadata_config_def }
}

impl<'a> Extract<'a> for CarryableInteractionsMetadataConfigDef {
    const TYPE_NAME: &'static str = "CarryableInteractionsMetadataConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            metadata: match inst.get("metadata") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CarryableInteractionsMetadataDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CarryableInteractionsMetadataDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CargoLoadingNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoLoadingNotificationParams {
    /// DCB field: `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// DCB field: `screenTimer` (Single)
    #[serde(default)]
    pub screen_timer: f32,
    /// DCB field: `hurryScreenTimer` (Single)
    #[serde(default)]
    pub hurry_screen_timer: f32,
    /// DCB field: `disabled` (Boolean)
    #[serde(default)]
    pub disabled: bool,
    /// DCB field: `dockNotificationParamsOverride` (Reference)
    #[serde(default)]
    pub dock_notification_params_override: Option<CigGuid>,
}

impl Pooled for CargoLoadingNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.cargo_loading_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.cargo_loading_notification_params }
}

impl<'a> Extract<'a> for CargoLoadingNotificationParams {
    const TYPE_NAME: &'static str = "CargoLoadingNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            screen_timer: inst.get_f32("screenTimer").unwrap_or_default(),
            hurry_screen_timer: inst.get_f32("hurryScreenTimer").unwrap_or_default(),
            disabled: inst.get_bool("disabled").unwrap_or_default(),
            dock_notification_params_override: inst.get("dockNotificationParamsOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `CargoGridOccupantFace`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoGridOccupantFace {
    /// DCB field: `faceUpwardAllowed` (Boolean)
    #[serde(default)]
    pub face_upward_allowed: bool,
    /// DCB field: `stackingSupport` (EnumChoice)
    #[serde(default)]
    pub stacking_support: String,
}

impl Pooled for CargoGridOccupantFace {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.cargo_grid_occupant_face }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.cargo_grid_occupant_face }
}

impl<'a> Extract<'a> for CargoGridOccupantFace {
    const TYPE_NAME: &'static str = "CargoGridOccupantFace";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            face_upward_allowed: inst.get_bool("faceUpwardAllowed").unwrap_or_default(),
            stacking_support: inst.get_str("stackingSupport").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoGridOccupantProperties`
///
/// Inherits from: `EntityClassStaticDataParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoGridOccupantProperties {
    /// DCB field: `Top` (Class)
    #[serde(default)]
    pub top: Option<Handle<CargoGridOccupantFace>>,
    /// DCB field: `Bottom` (Class)
    #[serde(default)]
    pub bottom: Option<Handle<CargoGridOccupantFace>>,
    /// DCB field: `Front` (Class)
    #[serde(default)]
    pub front: Option<Handle<CargoGridOccupantFace>>,
    /// DCB field: `Back` (Class)
    #[serde(default)]
    pub back: Option<Handle<CargoGridOccupantFace>>,
    /// DCB field: `Right` (Class)
    #[serde(default)]
    pub right: Option<Handle<CargoGridOccupantFace>>,
    /// DCB field: `Left` (Class)
    #[serde(default)]
    pub left: Option<Handle<CargoGridOccupantFace>>,
}

impl Pooled for CargoGridOccupantProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.cargo_grid_occupant_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.cargo_grid_occupant_properties }
}

impl<'a> Extract<'a> for CargoGridOccupantProperties {
    const TYPE_NAME: &'static str = "CargoGridOccupantProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            top: match inst.get("Top") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom: match inst.get("Bottom") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            front: match inst.get("Front") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            back: match inst.get("Back") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            right: match inst.get("Right") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            left: match inst.get("Left") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CapacitorAssignmentInputOutputDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacitorAssignmentInputOutputDef {
    /// DCB field: `inputOutputMapping` (Class)
    #[serde(default)]
    pub input_output_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for CapacitorAssignmentInputOutputDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.capacitor_assignment_input_output_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.capacitor_assignment_input_output_def }
}

impl<'a> Extract<'a> for CapacitorAssignmentInputOutputDef {
    const TYPE_NAME: &'static str = "CapacitorAssignmentInputOutputDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_output_mapping: match inst.get("inputOutputMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraTransitionInterpolationCurveRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTransitionInterpolationCurveRecord {
    /// DCB field: `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for CameraTransitionInterpolationCurveRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ca.camera_transition_interpolation_curve_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ca.camera_transition_interpolation_curve_record }
}

impl<'a> Extract<'a> for CameraTransitionInterpolationCurveRecord {
    const TYPE_NAME: &'static str = "CameraTransitionInterpolationCurveRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

