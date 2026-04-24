// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `entities-vfx` feature.
#[derive(Default)]
pub struct EntitiesVfxPools {
    pub daylight_particle_group_component_params: Vec<Option<DaylightParticleGroupComponentParams>>,
    pub placed_surface_effects_emitter: Vec<Option<PlacedSurfaceEffects_Emitter>>,
    pub surface_raindrops_target_component_params:
        Vec<Option<SurfaceRaindropsTargetComponentParams>>,
}
