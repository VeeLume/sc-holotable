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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ActorGForceCameraEffectsData`
pub struct ActorGForceCameraEffectsData {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `effectsOnlyAppliedForward` (Boolean)
    pub effects_only_applied_forward: bool,
    /// `gForceAngleOuter` (Single)
    pub g_force_angle_outer: f32,
    /// `gForceAngleInner` (Single)
    pub g_force_angle_inner: f32,
    /// `gForceMin` (Single)
    pub g_force_min: f32,
    /// `gForceMax` (Single)
    pub g_force_max: f32,
    /// `gForceFOV` (Single)
    pub g_force_fov: f32,
    /// `focusDistance` (Single)
    pub focus_distance: f32,
    /// `afterburnerEffectMinRatio` (Single)
    pub afterburner_effect_min_ratio: f32,
    /// `genericModifiers` (Class)
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
}

impl Pooled for ActorGForceCameraEffectsData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.actor_gforce.actor_gforce_camera_effects_data
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.actor_gforce.actor_gforce_camera_effects_data
    }
}

impl<'a> Extract<'a> for ActorGForceCameraEffectsData {
    const TYPE_NAME: &'static str = "ActorGForceCameraEffectsData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            effects_only_applied_forward: inst
                .get_bool("effectsOnlyAppliedForward")
                .unwrap_or_default(),
            g_force_angle_outer: inst.get_f32("gForceAngleOuter").unwrap_or_default(),
            g_force_angle_inner: inst.get_f32("gForceAngleInner").unwrap_or_default(),
            g_force_min: inst.get_f32("gForceMin").unwrap_or_default(),
            g_force_max: inst.get_f32("gForceMax").unwrap_or_default(),
            g_force_fov: inst.get_f32("gForceFOV").unwrap_or_default(),
            focus_distance: inst.get_f32("focusDistance").unwrap_or_default(),
            afterburner_effect_min_ratio: inst
                .get_f32("afterburnerEffectMinRatio")
                .unwrap_or_default(),
            generic_modifiers: match inst.get("genericModifiers") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CameraEffectsModifiers>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceCameraEffects`
pub struct ActorGForceCameraEffects {
    /// `cameraEffects` (Class)
    pub camera_effects: Option<Handle<ActorGForceCameraEffectsData>>,
}

impl Pooled for ActorGForceCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.actor_gforce.actor_gforce_camera_effects
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.actor_gforce.actor_gforce_camera_effects
    }
}

impl<'a> Extract<'a> for ActorGForceCameraEffects {
    const TYPE_NAME: &'static str = "ActorGForceCameraEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera_effects: match inst.get("cameraEffects") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ActorGForceCameraEffectsData>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
