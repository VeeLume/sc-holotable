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
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `vfx` feature.
#[derive(Default)]
pub struct VfxIndex {
    pub atmospheric_flight_effects: HashMap<CigGuid, Handle<AtmosphericFlightEffects>>,
    pub damage_map_global_params: HashMap<CigGuid, Handle<DamageMapGlobalParams>>,
    pub dematerialize_animation: HashMap<CigGuid, Handle<DematerializeAnimation>>,
    pub global_gas_cloud_vdbparams: HashMap<CigGuid, Handle<GlobalGasCloudVDBParams>>,
    pub planet_effect_lod: HashMap<CigGuid, Handle<PlanetEffectLOD>>,
    pub quantum_drive_effect_settings: HashMap<CigGuid, Handle<QuantumDriveEffectSettings>>,
    pub screen_effects_library: HashMap<CigGuid, Handle<ScreenEffects_Library>>,
    pub screen_effects_effect: HashMap<CigGuid, Handle<ScreenEffects_Effect>>,
    pub screen_effects_debug: HashMap<CigGuid, Handle<ScreenEffects_Debug>>,
    pub video_comms_shader_params: HashMap<CigGuid, Handle<VideoCommsShaderParams>>,
    pub water_effects_global_params: HashMap<CigGuid, Handle<WaterEffectsGlobalParams>>,
}

impl VfxIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.atmospheric_flight_effects.len();
        total += self.damage_map_global_params.len();
        total += self.dematerialize_animation.len();
        total += self.global_gas_cloud_vdbparams.len();
        total += self.planet_effect_lod.len();
        total += self.quantum_drive_effect_settings.len();
        total += self.screen_effects_library.len();
        total += self.screen_effects_effect.len();
        total += self.screen_effects_debug.len();
        total += self.video_comms_shader_params.len();
        total += self.water_effects_global_params.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
