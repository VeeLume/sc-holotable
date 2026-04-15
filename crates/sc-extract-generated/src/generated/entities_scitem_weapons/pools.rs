// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-scitem-weapons` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemWeaponsPools {
    #[serde(default)]
    pub ssensor_mine_proximity_trigger: Vec<Option<SSensorMineProximityTrigger>>,
    #[serde(default)]
    pub sgalactapedia_unlockable_component_params: Vec<Option<SGalactapediaUnlockableComponentParams>>,
    #[serde(default)]
    pub schange_holo_entity_state_modifier: Vec<Option<SChangeHoloEntityStateModifier>>,
    #[serde(default)]
    pub sweapon_aibeam_params: Vec<Option<SWeaponAIBeamParams>>,
    #[serde(default)]
    pub sweapon_condition_targeting_fire: Vec<Option<SWeaponConditionTargetingFire>>,
    #[serde(default)]
    pub sweapon_action_fire_thrust_params: Vec<Option<SWeaponActionFireThrustParams>>,
    #[serde(default)]
    pub scharge_drain_prime_params: Vec<Option<SChargeDrainPrimeParams>>,
    #[serde(default)]
    pub scharge_drain_range_params: Vec<Option<SChargeDrainRangeParams>>,
    #[serde(default)]
    pub sweapon_action_fire_charge_drain_params: Vec<Option<SWeaponActionFireChargeDrainParams>>,
    #[serde(default)]
    pub sextinguisher_impact_params: Vec<Option<SExtinguisherImpactParams>>,
    #[serde(default)]
    pub stemperature_read_out_params: Vec<Option<STemperatureReadOutParams>>,
    #[serde(default)]
    pub svector_field_params: Vec<Option<SVectorFieldParams>>,
    #[serde(default)]
    pub sextinguisher_vector_field_params: Vec<Option<SExtinguisherVectorFieldParams>>,
    #[serde(default)]
    pub sweapon_action_fire_extinguisher_params: Vec<Option<SWeaponActionFireExtinguisherParams>>,
}
