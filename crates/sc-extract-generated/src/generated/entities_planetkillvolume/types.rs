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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `PlanetKillVolumeComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetKillVolumeComponentParams {
    /// `planetRadius` (Single)
    #[serde(default)]
    pub planet_radius: f32,
    /// `killRadius` (Single)
    #[serde(default)]
    pub kill_radius: f32,
    /// `shakeRadius` (Single)
    #[serde(default)]
    pub shake_radius: f32,
    /// `warningRadius` (Single)
    #[serde(default)]
    pub warning_radius: f32,
    /// `shadowPenumbraWidth` (Single)
    #[serde(default)]
    pub shadow_penumbra_width: f32,
    /// `shadowMaxRange` (Single)
    #[serde(default)]
    pub shadow_max_range: f32,
}

impl Pooled for PlanetKillVolumeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_planetkillvolume.planet_kill_volume_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_planetkillvolume.planet_kill_volume_component_params }
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

