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

/// Pool storage for the `entities-scitem-weapons` feature.
#[derive(Default)]
pub struct EntitiesScitemWeaponsPools {
    pub ssensor_mine_proximity_trigger: Vec<Option<SSensorMineProximityTrigger>>,
    pub sgalactapedia_unlockable_component_params:
        Vec<Option<SGalactapediaUnlockableComponentParams>>,
    pub schange_holo_entity_state_modifier: Vec<Option<SChangeHoloEntityStateModifier>>,
    pub sweapon_aibeam_params: Vec<Option<SWeaponAIBeamParams>>,
    pub sweapon_condition_targeting_fire: Vec<Option<SWeaponConditionTargetingFire>>,
    pub sweapon_action_fire_thrust_params: Vec<Option<SWeaponActionFireThrustParams>>,
    pub scharge_drain_prime_params: Vec<Option<SChargeDrainPrimeParams>>,
    pub scharge_drain_range_params: Vec<Option<SChargeDrainRangeParams>>,
    pub sweapon_action_fire_charge_drain_params: Vec<Option<SWeaponActionFireChargeDrainParams>>,
    pub sextinguisher_impact_params: Vec<Option<SExtinguisherImpactParams>>,
    pub stemperature_read_out_params: Vec<Option<STemperatureReadOutParams>>,
    pub svector_field_params: Vec<Option<SVectorFieldParams>>,
    pub sextinguisher_vector_field_params: Vec<Option<SExtinguisherVectorFieldParams>>,
    pub sweapon_action_fire_extinguisher_params: Vec<Option<SWeaponActionFireExtinguisherParams>>,
}
