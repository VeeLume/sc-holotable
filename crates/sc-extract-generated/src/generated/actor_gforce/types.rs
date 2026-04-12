// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-gforce`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ActorGForceHeadBobFakeVelocityGData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceHeadBobFakeVelocityGData {
    /// `maxLinearSpeed` (Single)
    #[serde(default)]
    pub max_linear_speed: f32,
    /// `mappedMaxFakeGs` (Single)
    #[serde(default)]
    pub mapped_max_fake_gs: f32,
    /// `enabledForSpaceships` (Boolean)
    #[serde(default)]
    pub enabled_for_spaceships: bool,
    /// `enabledForGravlev` (Boolean)
    #[serde(default)]
    pub enabled_for_gravlev: bool,
    /// `enabledForGroundVehicles` (Boolean)
    #[serde(default)]
    pub enabled_for_ground_vehicles: bool,
    /// `enabledForBoats` (Boolean)
    #[serde(default)]
    pub enabled_for_boats: bool,
    /// `mapping` (Class)
    #[serde(default)]
    pub mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for ActorGForceHeadBobFakeVelocityGData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_gforce.actor_gforce_head_bob_fake_velocity_gdata }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_gforce.actor_gforce_head_bob_fake_velocity_gdata }
}

impl<'a> Extract<'a> for ActorGForceHeadBobFakeVelocityGData {
    const TYPE_NAME: &'static str = "ActorGForceHeadBobFakeVelocityGData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_linear_speed: inst.get_f32("maxLinearSpeed").unwrap_or_default(),
            mapped_max_fake_gs: inst.get_f32("mappedMaxFakeGs").unwrap_or_default(),
            enabled_for_spaceships: inst.get_bool("enabledForSpaceships").unwrap_or_default(),
            enabled_for_gravlev: inst.get_bool("enabledForGravlev").unwrap_or_default(),
            enabled_for_ground_vehicles: inst.get_bool("enabledForGroundVehicles").unwrap_or_default(),
            enabled_for_boats: inst.get_bool("enabledForBoats").unwrap_or_default(),
            mapping: match inst.get("mapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceHeadBobData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceHeadBobData {
    /// `maxGforce` (Class)
    #[serde(default)]
    pub max_gforce: Option<Handle<Vec3>>,
    /// `undampedFrequencyTranslation` (Class)
    #[serde(default)]
    pub undamped_frequency_translation: Option<Handle<Vec3>>,
    /// `dampingRatioTranslation` (Class)
    #[serde(default)]
    pub damping_ratio_translation: Option<Handle<Vec3>>,
    /// `maxHeadTranslation` (Class)
    #[serde(default)]
    pub max_head_translation: Option<Handle<Vec3>>,
    /// `translationSmoothing` (Single)
    #[serde(default)]
    pub translation_smoothing: f32,
    /// `undampedFrequencyRotation` (Class)
    #[serde(default)]
    pub undamped_frequency_rotation: Option<Handle<Vec3>>,
    /// `dampingRatioRotation` (Class)
    #[serde(default)]
    pub damping_ratio_rotation: Option<Handle<Vec3>>,
    /// `maxHeadRotation` (Class)
    #[serde(default)]
    pub max_head_rotation: Option<Handle<Ang3>>,
    /// `maxHeadRotationMagLaunch` (Class)
    #[serde(default)]
    pub max_head_rotation_mag_launch: Option<Handle<Ang3>>,
    /// `rotationSmoothing` (Single)
    #[serde(default)]
    pub rotation_smoothing: f32,
    /// `headTranslationLimitsPositive` (Class)
    #[serde(default)]
    pub head_translation_limits_positive: Option<Handle<Vec3>>,
    /// `headTranslationLimitsNegative` (Class)
    #[serde(default)]
    pub head_translation_limits_negative: Option<Handle<Vec3>>,
    /// `rotationalAccelerationInputModifier` (Single)
    #[serde(default)]
    pub rotational_acceleration_input_modifier: f32,
    /// `jumpDriveFlightModifierTranslation` (Single)
    #[serde(default)]
    pub jump_drive_flight_modifier_translation: f32,
    /// `jumpDriveFlightModifierRotation` (Single)
    #[serde(default)]
    pub jump_drive_flight_modifier_rotation: f32,
    /// `fakeVelocityGs` (Class)
    #[serde(default)]
    pub fake_velocity_gs: Option<Handle<ActorGForceHeadBobFakeVelocityGData>>,
    /// `quantumAccelerationMaxGs` (Single)
    #[serde(default)]
    pub quantum_acceleration_max_gs: f32,
}

impl Pooled for ActorGForceHeadBobData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_gforce.actor_gforce_head_bob_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_gforce.actor_gforce_head_bob_data }
}

impl<'a> Extract<'a> for ActorGForceHeadBobData {
    const TYPE_NAME: &'static str = "ActorGForceHeadBobData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_gforce: match inst.get("maxGforce") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            undamped_frequency_translation: match inst.get("undampedFrequencyTranslation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damping_ratio_translation: match inst.get("dampingRatioTranslation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_head_translation: match inst.get("maxHeadTranslation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            translation_smoothing: inst.get_f32("translationSmoothing").unwrap_or_default(),
            undamped_frequency_rotation: match inst.get("undampedFrequencyRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damping_ratio_rotation: match inst.get("dampingRatioRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_head_rotation: match inst.get("maxHeadRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_head_rotation_mag_launch: match inst.get("maxHeadRotationMagLaunch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_smoothing: inst.get_f32("rotationSmoothing").unwrap_or_default(),
            head_translation_limits_positive: match inst.get("headTranslationLimitsPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_translation_limits_negative: match inst.get("headTranslationLimitsNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotational_acceleration_input_modifier: inst.get_f32("rotationalAccelerationInputModifier").unwrap_or_default(),
            jump_drive_flight_modifier_translation: inst.get_f32("jumpDriveFlightModifierTranslation").unwrap_or_default(),
            jump_drive_flight_modifier_rotation: inst.get_f32("jumpDriveFlightModifierRotation").unwrap_or_default(),
            fake_velocity_gs: match inst.get("fakeVelocityGs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceHeadBobFakeVelocityGData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceHeadBobFakeVelocityGData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            quantum_acceleration_max_gs: inst.get_f32("quantumAccelerationMaxGs").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorGForceHeadBob`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceHeadBob {
    /// `headBobData` (Class)
    #[serde(default)]
    pub head_bob_data: Option<Handle<ActorGForceHeadBobData>>,
    /// `headBobDataLegacy` (Class)
    #[serde(default)]
    pub head_bob_data_legacy: Option<Handle<ActorGForceHeadBobData>>,
}

impl Pooled for ActorGForceHeadBob {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_gforce.actor_gforce_head_bob }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_gforce.actor_gforce_head_bob }
}

impl<'a> Extract<'a> for ActorGForceHeadBob {
    const TYPE_NAME: &'static str = "ActorGForceHeadBob";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            head_bob_data: match inst.get("headBobData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceHeadBobData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceHeadBobData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_bob_data_legacy: match inst.get("headBobDataLegacy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceHeadBobData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceHeadBobData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceCameraEffectsData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceCameraEffectsData {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `effectsOnlyAppliedForward` (Boolean)
    #[serde(default)]
    pub effects_only_applied_forward: bool,
    /// `gForceAngleOuter` (Single)
    #[serde(default)]
    pub g_force_angle_outer: f32,
    /// `gForceAngleInner` (Single)
    #[serde(default)]
    pub g_force_angle_inner: f32,
    /// `gForceMin` (Single)
    #[serde(default)]
    pub g_force_min: f32,
    /// `gForceMax` (Single)
    #[serde(default)]
    pub g_force_max: f32,
    /// `gForceFOV` (Single)
    #[serde(default)]
    pub g_force_fov: f32,
    /// `focusDistance` (Single)
    #[serde(default)]
    pub focus_distance: f32,
    /// `afterburnerEffectMinRatio` (Single)
    #[serde(default)]
    pub afterburner_effect_min_ratio: f32,
    /// `genericModifiers` (Class)
    #[serde(default)]
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
}

impl Pooled for ActorGForceCameraEffectsData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_gforce.actor_gforce_camera_effects_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_gforce.actor_gforce_camera_effects_data }
}

impl<'a> Extract<'a> for ActorGForceCameraEffectsData {
    const TYPE_NAME: &'static str = "ActorGForceCameraEffectsData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            effects_only_applied_forward: inst.get_bool("effectsOnlyAppliedForward").unwrap_or_default(),
            g_force_angle_outer: inst.get_f32("gForceAngleOuter").unwrap_or_default(),
            g_force_angle_inner: inst.get_f32("gForceAngleInner").unwrap_or_default(),
            g_force_min: inst.get_f32("gForceMin").unwrap_or_default(),
            g_force_max: inst.get_f32("gForceMax").unwrap_or_default(),
            g_force_fov: inst.get_f32("gForceFOV").unwrap_or_default(),
            focus_distance: inst.get_f32("focusDistance").unwrap_or_default(),
            afterburner_effect_min_ratio: inst.get_f32("afterburnerEffectMinRatio").unwrap_or_default(),
            generic_modifiers: match inst.get("genericModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraEffectsModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraEffectsModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceCameraEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceCameraEffects {
    /// `cameraEffects` (Class)
    #[serde(default)]
    pub camera_effects: Option<Handle<ActorGForceCameraEffectsData>>,
}

impl Pooled for ActorGForceCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_gforce.actor_gforce_camera_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_gforce.actor_gforce_camera_effects }
}

impl<'a> Extract<'a> for ActorGForceCameraEffects {
    const TYPE_NAME: &'static str = "ActorGForceCameraEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera_effects: match inst.get("cameraEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceCameraEffectsData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceCameraEffectsData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

