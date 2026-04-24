// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-vfx`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `DaylightParticleGroupComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct DaylightParticleGroupComponentParams {
    /// `activationBehavior` (EnumChoice)
    pub activation_behavior: DaylightParticleGroupActivation,
}

impl Pooled for DaylightParticleGroupComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_vfx.daylight_particle_group_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_vfx.daylight_particle_group_component_params }
}

impl<'a> Extract<'a> for DaylightParticleGroupComponentParams {
    const TYPE_NAME: &'static str = "DaylightParticleGroupComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activation_behavior: DaylightParticleGroupActivation::from_dcb_str(inst.get_str("activationBehavior").unwrap_or("")),
        }
    }
}

/// DCB type: `PlacedSurfaceEffects_Emitter`
pub struct PlacedSurfaceEffects_Emitter {
    /// `name` (String)
    pub name: String,
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `particleEffect` (Class)
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `emitterPosition` (Class)
    pub emitter_position: Option<Handle<Vec3>>,
    /// `linkedToSdf` (Boolean)
    pub linked_to_sdf: bool,
    /// `fadeOutDuration` (Single)
    pub fade_out_duration: f32,
}

impl Pooled for PlacedSurfaceEffects_Emitter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_vfx.placed_surface_effects_emitter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_vfx.placed_surface_effects_emitter }
}

impl<'a> Extract<'a> for PlacedSurfaceEffects_Emitter {
    const TYPE_NAME: &'static str = "PlacedSurfaceEffects_Emitter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            emitter_position: match inst.get("emitterPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            linked_to_sdf: inst.get_bool("linkedToSdf").unwrap_or_default(),
            fade_out_duration: inst.get_f32("fadeOutDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `SurfaceRaindropsTargetComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SurfaceRaindropsTargetComponentParams {
}

impl Pooled for SurfaceRaindropsTargetComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_vfx.surface_raindrops_target_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_vfx.surface_raindrops_target_component_params }
}

impl<'a> Extract<'a> for SurfaceRaindropsTargetComponentParams {
    const TYPE_NAME: &'static str = "SurfaceRaindropsTargetComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

