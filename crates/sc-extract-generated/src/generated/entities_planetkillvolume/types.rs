// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-planetkillvolume`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `PlanetKillVolumeComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct PlanetKillVolumeComponentParams {
    /// `planetRadius` (Single)
    pub planet_radius: f32,
    /// `killRadius` (Single)
    pub kill_radius: f32,
    /// `shakeRadius` (Single)
    pub shake_radius: f32,
    /// `warningRadius` (Single)
    pub warning_radius: f32,
    /// `shadowPenumbraWidth` (Single)
    pub shadow_penumbra_width: f32,
    /// `shadowMaxRange` (Single)
    pub shadow_max_range: f32,
}

impl Pooled for PlanetKillVolumeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_planetkillvolume
            .planet_kill_volume_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_planetkillvolume
            .planet_kill_volume_component_params
    }
}

impl<'a> Extract<'a> for PlanetKillVolumeComponentParams {
    const TYPE_NAME: &'static str = "PlanetKillVolumeComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            planet_radius: inst.get_f32("planetRadius").unwrap_or_default(),
            kill_radius: inst.get_f32("killRadius").unwrap_or_default(),
            shake_radius: inst.get_f32("shakeRadius").unwrap_or_default(),
            warning_radius: inst.get_f32("warningRadius").unwrap_or_default(),
            shadow_penumbra_width: inst.get_f32("shadowPenumbraWidth").unwrap_or_default(),
            shadow_max_range: inst.get_f32("shadowMaxRange").unwrap_or_default(),
        }
    }
}
