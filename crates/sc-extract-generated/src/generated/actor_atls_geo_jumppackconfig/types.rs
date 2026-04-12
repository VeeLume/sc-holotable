// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-atls_geo_jumppackconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `JumpThrusterPackConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpThrusterPackConfig {
    /// `useJumpThrusters` (Boolean)
    #[serde(default)]
    pub use_jump_thrusters: bool,
    /// `jumpBoosterFuel` (Single)
    #[serde(default)]
    pub jump_booster_fuel: f32,
    /// `jumpBoosterFuelConsumptionSpeed` (Single)
    #[serde(default)]
    pub jump_booster_fuel_consumption_speed: f32,
    /// `jumpBoosterFuelAirRestoreSpeed` (Single)
    #[serde(default)]
    pub jump_booster_fuel_air_restore_speed: f32,
    /// `jumpBoosterFuelGroundRestoreSpeed` (Single)
    #[serde(default)]
    pub jump_booster_fuel_ground_restore_speed: f32,
    /// `jumpThrustersImpulseStrenght` (Single)
    #[serde(default)]
    pub jump_thrusters_impulse_strenght: f32,
    /// `useLandThrusters` (Boolean)
    #[serde(default)]
    pub use_land_thrusters: bool,
    /// `landThrustersImpulseStrenght` (Single)
    #[serde(default)]
    pub land_thrusters_impulse_strenght: f32,
    /// `landThrustersMinThresholdDistance` (Single)
    #[serde(default)]
    pub land_thrusters_min_threshold_distance: f32,
    /// `landThrustersMaxThresholdDistance` (Single)
    #[serde(default)]
    pub land_thrusters_max_threshold_distance: f32,
    /// `landThrustersMinSpeed` (Single)
    #[serde(default)]
    pub land_thrusters_min_speed: f32,
    /// `landThrustersMinTime` (Single)
    #[serde(default)]
    pub land_thrusters_min_time: f32,
}

impl Pooled for JumpThrusterPackConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_atls_geo_jumppackconfig.jump_thruster_pack_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_atls_geo_jumppackconfig.jump_thruster_pack_config }
}

impl<'a> Extract<'a> for JumpThrusterPackConfig {
    const TYPE_NAME: &'static str = "JumpThrusterPackConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            use_jump_thrusters: inst.get_bool("useJumpThrusters").unwrap_or_default(),
            jump_booster_fuel: inst.get_f32("jumpBoosterFuel").unwrap_or_default(),
            jump_booster_fuel_consumption_speed: inst.get_f32("jumpBoosterFuelConsumptionSpeed").unwrap_or_default(),
            jump_booster_fuel_air_restore_speed: inst.get_f32("jumpBoosterFuelAirRestoreSpeed").unwrap_or_default(),
            jump_booster_fuel_ground_restore_speed: inst.get_f32("jumpBoosterFuelGroundRestoreSpeed").unwrap_or_default(),
            jump_thrusters_impulse_strenght: inst.get_f32("jumpThrustersImpulseStrenght").unwrap_or_default(),
            use_land_thrusters: inst.get_bool("useLandThrusters").unwrap_or_default(),
            land_thrusters_impulse_strenght: inst.get_f32("landThrustersImpulseStrenght").unwrap_or_default(),
            land_thrusters_min_threshold_distance: inst.get_f32("landThrustersMinThresholdDistance").unwrap_or_default(),
            land_thrusters_max_threshold_distance: inst.get_f32("landThrustersMaxThresholdDistance").unwrap_or_default(),
            land_thrusters_min_speed: inst.get_f32("landThrustersMinSpeed").unwrap_or_default(),
            land_thrusters_min_time: inst.get_f32("landThrustersMinTime").unwrap_or_default(),
        }
    }
}

