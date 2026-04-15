// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `vfx` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VfxIndex {
    #[serde(default)]
    pub atmospheric_flight_effects: HashMap<CigGuid, Handle<AtmosphericFlightEffects>>,
    #[serde(default)]
    pub damage_map_global_params: HashMap<CigGuid, Handle<DamageMapGlobalParams>>,
    #[serde(default)]
    pub dematerialize_animation: HashMap<CigGuid, Handle<DematerializeAnimation>>,
    #[serde(default)]
    pub global_gas_cloud_vdbparams: HashMap<CigGuid, Handle<GlobalGasCloudVDBParams>>,
    #[serde(default)]
    pub planet_effect_lod: HashMap<CigGuid, Handle<PlanetEffectLOD>>,
    #[serde(default)]
    pub quantum_drive_effect_settings: HashMap<CigGuid, Handle<QuantumDriveEffectSettings>>,
    #[serde(default)]
    pub screen_effects_library: HashMap<CigGuid, Handle<ScreenEffects_Library>>,
    #[serde(default)]
    pub screen_effects_effect: HashMap<CigGuid, Handle<ScreenEffects_Effect>>,
    #[serde(default)]
    pub screen_effects_debug: HashMap<CigGuid, Handle<ScreenEffects_Debug>>,
    #[serde(default)]
    pub video_comms_shader_params: HashMap<CigGuid, Handle<VideoCommsShaderParams>>,
    #[serde(default)]
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

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
