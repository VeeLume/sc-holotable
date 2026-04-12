// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `mining`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `HitConsistencyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitConsistencyParams {
    /// `hitHistoryWindow` (Single)
    #[serde(default)]
    pub hit_history_window: f32,
    /// `standardDeviationMultiplier` (Single)
    #[serde(default)]
    pub standard_deviation_multiplier: f32,
    /// `timeExponent` (Single)
    #[serde(default)]
    pub time_exponent: f32,
    /// `minDeviation` (Single)
    #[serde(default)]
    pub min_deviation: f32,
    /// `extractionMagnitude` (Single)
    #[serde(default)]
    pub extraction_magnitude: f32,
    /// `maxEffectOnInstability` (Single)
    #[serde(default)]
    pub max_effect_on_instability: f32,
}

impl Pooled for HitConsistencyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.hit_consistency_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.hit_consistency_params }
}

impl<'a> Extract<'a> for HitConsistencyParams {
    const TYPE_NAME: &'static str = "HitConsistencyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hit_history_window: inst.get_f32("hitHistoryWindow").unwrap_or_default(),
            standard_deviation_multiplier: inst.get_f32("standardDeviationMultiplier").unwrap_or_default(),
            time_exponent: inst.get_f32("timeExponent").unwrap_or_default(),
            min_deviation: inst.get_f32("minDeviation").unwrap_or_default(),
            extraction_magnitude: inst.get_f32("extractionMagnitude").unwrap_or_default(),
            max_effect_on_instability: inst.get_f32("maxEffectOnInstability").unwrap_or_default(),
        }
    }
}

/// DCB type: `MineableExplosionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineableExplosionParams {
    /// `defaultExplosionParams` (Class)
    #[serde(default)]
    pub default_explosion_params: Option<Handle<ExplosionParams>>,
    /// `dangerPoolFactor` (Single)
    #[serde(default)]
    pub danger_pool_factor: f32,
    /// `defaultVolume` (Single)
    #[serde(default)]
    pub default_volume: f32,
}

impl Pooled for MineableExplosionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mineable_explosion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mineable_explosion_params }
}

impl<'a> Extract<'a> for MineableExplosionParams {
    const TYPE_NAME: &'static str = "MineableExplosionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_explosion_params: match inst.get("defaultExplosionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            danger_pool_factor: inst.get_f32("dangerPoolFactor").unwrap_or_default(),
            default_volume: inst.get_f32("defaultVolume").unwrap_or_default(),
        }
    }
}

/// DCB type: `MineableInstabilityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineableInstabilityParams {
    /// `instabilityWavePeriod` (Single)
    #[serde(default)]
    pub instability_wave_period: f32,
    /// `instabilityWaveVariance` (Single)
    #[serde(default)]
    pub instability_wave_variance: f32,
    /// `instabilityCurveFactor` (Single)
    #[serde(default)]
    pub instability_curve_factor: f32,
}

impl Pooled for MineableInstabilityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mineable_instability_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mineable_instability_params }
}

impl<'a> Extract<'a> for MineableInstabilityParams {
    const TYPE_NAME: &'static str = "MineableInstabilityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            instability_wave_period: inst.get_f32("instabilityWavePeriod").unwrap_or_default(),
            instability_wave_variance: inst.get_f32("instabilityWaveVariance").unwrap_or_default(),
            instability_curve_factor: inst.get_f32("instabilityCurveFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `MiningGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningGlobalParams {
    /// `powerCapacityPerMass` (Single)
    #[serde(default)]
    pub power_capacity_per_mass: f32,
    /// `decayPerMass` (Single)
    #[serde(default)]
    pub decay_per_mass: f32,
    /// `optimalWindowSize` (Single)
    #[serde(default)]
    pub optimal_window_size: f32,
    /// `optimalWindowFactor` (Single)
    #[serde(default)]
    pub optimal_window_factor: f32,
    /// `resistanceCurveFactor` (Single)
    #[serde(default)]
    pub resistance_curve_factor: f32,
    /// `optimalWindowThinnessCurveFactor` (Single)
    #[serde(default)]
    pub optimal_window_thinness_curve_factor: f32,
    /// `optimalWindowMaxSize` (Single)
    #[serde(default)]
    pub optimal_window_max_size: f32,
    /// `controlledBreakingFillRate` (Single)
    #[serde(default)]
    pub controlled_breaking_fill_rate: f32,
    /// `controlledBreakingFillRateDanger` (Single)
    #[serde(default)]
    pub controlled_breaking_fill_rate_danger: f32,
    /// `controlledBreakingDecayRate` (Single)
    #[serde(default)]
    pub controlled_breaking_decay_rate: f32,
    /// `dangerBreakingFillRate` (Single)
    #[serde(default)]
    pub danger_breaking_fill_rate: f32,
    /// `dangerBreakingFillRateExponent` (Single)
    #[serde(default)]
    pub danger_breaking_fill_rate_exponent: f32,
    /// `dangerBreakingDecayRate` (Single)
    #[serde(default)]
    pub danger_breaking_decay_rate: f32,
    /// `absorbableVolumeThreshold` (Single)
    #[serde(default)]
    pub absorbable_volume_threshold: f32,
    /// `mineableInstabilityParams` (Class)
    #[serde(default)]
    pub mineable_instability_params: Option<Handle<MineableInstabilityParams>>,
    /// `mineableExplosionParams` (Class)
    #[serde(default)]
    pub mineable_explosion_params: Option<Handle<MineableExplosionParams>>,
    /// `childRockInvulnerabilityTime` (Single)
    #[serde(default)]
    pub child_rock_invulnerability_time: f32,
    /// `cSCUPerVolume` (Single)
    #[serde(default)]
    pub c_scuper_volume: f32,
    /// `defaultMass` (Single)
    #[serde(default)]
    pub default_mass: f32,
    /// `fractureParticleEffect` (Class)
    #[serde(default)]
    pub fracture_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `explosionParticleEffect` (Class)
    #[serde(default)]
    pub explosion_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `centerRockDestroyParticleEffect` (Class)
    #[serde(default)]
    pub center_rock_destroy_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `fullyExtractedRockParticleEffect` (Class)
    #[serde(default)]
    pub fully_extracted_rock_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `hitConsistencyParams` (Class)
    #[serde(default)]
    pub hit_consistency_params: Option<Handle<HitConsistencyParams>>,
    /// `modifierPersistenceTime` (Single)
    #[serde(default)]
    pub modifier_persistence_time: f32,
    /// `childRockLifeTimer` (Single)
    #[serde(default)]
    pub child_rock_life_timer: f32,
    /// `childRockZeroGDamping` (Single)
    #[serde(default)]
    pub child_rock_zero_gdamping: f32,
    /// `terrainFactorStaticThreshold` (Single)
    #[serde(default)]
    pub terrain_factor_static_threshold: f32,
    /// `showExplosionFXForSurplusChild` (Boolean)
    #[serde(default)]
    pub show_explosion_fxfor_surplus_child: bool,
    /// `childRockInactivityLifetime` (Single)
    #[serde(default)]
    pub child_rock_inactivity_lifetime: f32,
    /// `gadgetDetachThreshold` (Single)
    #[serde(default)]
    pub gadget_detach_threshold: f32,
    /// `gadgetDestroyThreshold` (Single)
    #[serde(default)]
    pub gadget_destroy_threshold: f32,
    /// `dangerToGadgetDamage` (Single)
    #[serde(default)]
    pub danger_to_gadget_damage: f32,
    /// `wasteResourceType` (Reference)
    #[serde(default)]
    pub waste_resource_type: Option<CigGuid>,
}

impl Pooled for MiningGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mining_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mining_global_params }
}

impl<'a> Extract<'a> for MiningGlobalParams {
    const TYPE_NAME: &'static str = "MiningGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            power_capacity_per_mass: inst.get_f32("powerCapacityPerMass").unwrap_or_default(),
            decay_per_mass: inst.get_f32("decayPerMass").unwrap_or_default(),
            optimal_window_size: inst.get_f32("optimalWindowSize").unwrap_or_default(),
            optimal_window_factor: inst.get_f32("optimalWindowFactor").unwrap_or_default(),
            resistance_curve_factor: inst.get_f32("resistanceCurveFactor").unwrap_or_default(),
            optimal_window_thinness_curve_factor: inst.get_f32("optimalWindowThinnessCurveFactor").unwrap_or_default(),
            optimal_window_max_size: inst.get_f32("optimalWindowMaxSize").unwrap_or_default(),
            controlled_breaking_fill_rate: inst.get_f32("controlledBreakingFillRate").unwrap_or_default(),
            controlled_breaking_fill_rate_danger: inst.get_f32("controlledBreakingFillRateDanger").unwrap_or_default(),
            controlled_breaking_decay_rate: inst.get_f32("controlledBreakingDecayRate").unwrap_or_default(),
            danger_breaking_fill_rate: inst.get_f32("dangerBreakingFillRate").unwrap_or_default(),
            danger_breaking_fill_rate_exponent: inst.get_f32("dangerBreakingFillRateExponent").unwrap_or_default(),
            danger_breaking_decay_rate: inst.get_f32("dangerBreakingDecayRate").unwrap_or_default(),
            absorbable_volume_threshold: inst.get_f32("absorbableVolumeThreshold").unwrap_or_default(),
            mineable_instability_params: match inst.get("mineableInstabilityParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MineableInstabilityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MineableInstabilityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_explosion_params: match inst.get("mineableExplosionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MineableExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MineableExplosionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            child_rock_invulnerability_time: inst.get_f32("childRockInvulnerabilityTime").unwrap_or_default(),
            c_scuper_volume: inst.get_f32("cSCUPerVolume").unwrap_or_default(),
            default_mass: inst.get_f32("defaultMass").unwrap_or_default(),
            fracture_particle_effect: match inst.get("fractureParticleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            explosion_particle_effect: match inst.get("explosionParticleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            center_rock_destroy_particle_effect: match inst.get("centerRockDestroyParticleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fully_extracted_rock_particle_effect: match inst.get("fullyExtractedRockParticleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hit_consistency_params: match inst.get("hitConsistencyParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HitConsistencyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HitConsistencyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifier_persistence_time: inst.get_f32("modifierPersistenceTime").unwrap_or_default(),
            child_rock_life_timer: inst.get_f32("childRockLifeTimer").unwrap_or_default(),
            child_rock_zero_gdamping: inst.get_f32("childRockZeroGDamping").unwrap_or_default(),
            terrain_factor_static_threshold: inst.get_f32("terrainFactorStaticThreshold").unwrap_or_default(),
            show_explosion_fxfor_surplus_child: inst.get_bool("showExplosionFXForSurplusChild").unwrap_or_default(),
            child_rock_inactivity_lifetime: inst.get_f32("childRockInactivityLifetime").unwrap_or_default(),
            gadget_detach_threshold: inst.get_f32("gadgetDetachThreshold").unwrap_or_default(),
            gadget_destroy_threshold: inst.get_f32("gadgetDestroyThreshold").unwrap_or_default(),
            danger_to_gadget_damage: inst.get_f32("dangerToGadgetDamage").unwrap_or_default(),
            waste_resource_type: inst.get("wasteResourceType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MineableElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineableElement {
    /// `resourceType` (Reference)
    #[serde(default)]
    pub resource_type: Option<CigGuid>,
    /// `elementInstability` (Single)
    #[serde(default)]
    pub element_instability: f32,
    /// `elementResistance` (Single)
    #[serde(default)]
    pub element_resistance: f32,
    /// `elementOptimalWindowMidpoint` (Single)
    #[serde(default)]
    pub element_optimal_window_midpoint: f32,
    /// `elementOptimalWindowMidpointRandomness` (Single)
    #[serde(default)]
    pub element_optimal_window_midpoint_randomness: f32,
    /// `elementOptimalWindowThinness` (Single)
    #[serde(default)]
    pub element_optimal_window_thinness: f32,
    /// `elementExplosionMultiplier` (Single)
    #[serde(default)]
    pub element_explosion_multiplier: f32,
    /// `elementClusterFactor` (Single)
    #[serde(default)]
    pub element_cluster_factor: f32,
}

impl Pooled for MineableElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mineable_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mineable_element }
}

impl<'a> Extract<'a> for MineableElement {
    const TYPE_NAME: &'static str = "MineableElement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource_type: inst.get("resourceType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            element_instability: inst.get_f32("elementInstability").unwrap_or_default(),
            element_resistance: inst.get_f32("elementResistance").unwrap_or_default(),
            element_optimal_window_midpoint: inst.get_f32("elementOptimalWindowMidpoint").unwrap_or_default(),
            element_optimal_window_midpoint_randomness: inst.get_f32("elementOptimalWindowMidpointRandomness").unwrap_or_default(),
            element_optimal_window_thinness: inst.get_f32("elementOptimalWindowThinness").unwrap_or_default(),
            element_explosion_multiplier: inst.get_f32("elementExplosionMultiplier").unwrap_or_default(),
            element_cluster_factor: inst.get_f32("elementClusterFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `MineableCompositionPart`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineableCompositionPart {
    /// `mineableElement` (Reference)
    #[serde(default)]
    pub mineable_element: Option<CigGuid>,
    /// `minPercentage` (Single)
    #[serde(default)]
    pub min_percentage: f32,
    /// `maxPercentage` (Single)
    #[serde(default)]
    pub max_percentage: f32,
    /// `probability` (Single)
    #[serde(default)]
    pub probability: f32,
    /// `curveExponent` (Single)
    #[serde(default)]
    pub curve_exponent: f32,
    /// `qualityScale` (Single)
    #[serde(default)]
    pub quality_scale: f32,
}

impl Pooled for MineableCompositionPart {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mineable_composition_part }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mineable_composition_part }
}

impl<'a> Extract<'a> for MineableCompositionPart {
    const TYPE_NAME: &'static str = "MineableCompositionPart";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mineable_element: inst.get("mineableElement").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            min_percentage: inst.get_f32("minPercentage").unwrap_or_default(),
            max_percentage: inst.get_f32("maxPercentage").unwrap_or_default(),
            probability: inst.get_f32("probability").unwrap_or_default(),
            curve_exponent: inst.get_f32("curveExponent").unwrap_or_default(),
            quality_scale: inst.get_f32("qualityScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `MineableComposition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineableComposition {
    /// `depositName` (Locale)
    #[serde(default)]
    pub deposit_name: String,
    /// `minimumDistinctElements` (Int32)
    #[serde(default)]
    pub minimum_distinct_elements: i32,
    /// `compositionArray` (Class (array))
    #[serde(default)]
    pub composition_array: Vec<Handle<MineableCompositionPart>>,
}

impl Pooled for MineableComposition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mineable_composition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mineable_composition }
}

impl<'a> Extract<'a> for MineableComposition {
    const TYPE_NAME: &'static str = "MineableComposition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            deposit_name: inst.get_str("depositName").map(String::from).unwrap_or_default(),
            minimum_distinct_elements: inst.get_i32("minimumDistinctElements").unwrap_or_default(),
            composition_array: inst.get_array("compositionArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MineableCompositionPart>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MineableCompositionPart>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MiningLaserGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningLaserGlobalParams {
    /// `blockThrottleChangeWhenNotFiring` (Boolean)
    #[serde(default)]
    pub block_throttle_change_when_not_firing: bool,
    /// `throttleResetOnStopFire` (Boolean)
    #[serde(default)]
    pub throttle_reset_on_stop_fire: bool,
    /// `throttleChangePerAction` (Single)
    #[serde(default)]
    pub throttle_change_per_action: f32,
    /// `throttleAccPeriod` (Single)
    #[serde(default)]
    pub throttle_acc_period: f32,
    /// `throttleAccFactor` (Single)
    #[serde(default)]
    pub throttle_acc_factor: f32,
    /// `throttleHoldAccFactor` (Single)
    #[serde(default)]
    pub throttle_hold_acc_factor: f32,
    /// `throttleRTPC` (Class)
    #[serde(default)]
    pub throttle_rtpc: Option<Handle<AudioRtpc>>,
    /// `mineableGlowStrengthRTPC` (Class)
    #[serde(default)]
    pub mineable_glow_strength_rtpc: Option<Handle<AudioRtpc>>,
    /// `mineableOptimalBreakZoneRTPC` (Class)
    #[serde(default)]
    pub mineable_optimal_break_zone_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for MiningLaserGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mining_laser_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mining_laser_global_params }
}

impl<'a> Extract<'a> for MiningLaserGlobalParams {
    const TYPE_NAME: &'static str = "MiningLaserGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            block_throttle_change_when_not_firing: inst.get_bool("blockThrottleChangeWhenNotFiring").unwrap_or_default(),
            throttle_reset_on_stop_fire: inst.get_bool("throttleResetOnStopFire").unwrap_or_default(),
            throttle_change_per_action: inst.get_f32("throttleChangePerAction").unwrap_or_default(),
            throttle_acc_period: inst.get_f32("throttleAccPeriod").unwrap_or_default(),
            throttle_acc_factor: inst.get_f32("throttleAccFactor").unwrap_or_default(),
            throttle_hold_acc_factor: inst.get_f32("throttleHoldAccFactor").unwrap_or_default(),
            throttle_rtpc: match inst.get("throttleRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_glow_strength_rtpc: match inst.get("mineableGlowStrengthRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_optimal_break_zone_rtpc: match inst.get("mineableOptimalBreakZoneRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MiningCameraShakeConfig`
/// Inherits from: `CameraShakeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningCameraShakeConfig {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `offsetPosition` (Class)
    #[serde(default)]
    pub offset_position: Option<Handle<Vec3>>,
    /// `offsetAngle` (Class)
    #[serde(default)]
    pub offset_angle: Option<Handle<Ang3>>,
    /// `timePeriod` (Single)
    #[serde(default)]
    pub time_period: f32,
    /// `frequencyNoiseFactor` (Single)
    #[serde(default)]
    pub frequency_noise_factor: f32,
    /// `translationNoise` (Single)
    #[serde(default)]
    pub translation_noise: f32,
    /// `rotationNoise` (Single)
    #[serde(default)]
    pub rotation_noise: f32,
    /// `maxShakeWhenUnderOptimalWindow` (Single)
    #[serde(default)]
    pub max_shake_when_under_optimal_window: f32,
    /// `shakeInOptimalWindow` (Single)
    #[serde(default)]
    pub shake_in_optimal_window: f32,
    /// `minShakeInDangerWindow` (Single)
    #[serde(default)]
    pub min_shake_in_danger_window: f32,
    /// `shakeChangeLerpSpeed` (Single)
    #[serde(default)]
    pub shake_change_lerp_speed: f32,
}

impl Pooled for MiningCameraShakeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mining_camera_shake_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mining_camera_shake_config }
}

impl<'a> Extract<'a> for MiningCameraShakeConfig {
    const TYPE_NAME: &'static str = "MiningCameraShakeConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            offset_position: match inst.get("offsetPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_angle: match inst.get("offsetAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time_period: inst.get_f32("timePeriod").unwrap_or_default(),
            frequency_noise_factor: inst.get_f32("frequencyNoiseFactor").unwrap_or_default(),
            translation_noise: inst.get_f32("translationNoise").unwrap_or_default(),
            rotation_noise: inst.get_f32("rotationNoise").unwrap_or_default(),
            max_shake_when_under_optimal_window: inst.get_f32("maxShakeWhenUnderOptimalWindow").unwrap_or_default(),
            shake_in_optimal_window: inst.get_f32("shakeInOptimalWindow").unwrap_or_default(),
            min_shake_in_danger_window: inst.get_f32("minShakeInDangerWindow").unwrap_or_default(),
            shake_change_lerp_speed: inst.get_f32("shakeChangeLerpSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `MiningControllerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningControllerGlobalParams {
    /// `highlightColor` (Class)
    #[serde(default)]
    pub highlight_color: Option<Handle<RGBA>>,
    /// `highlightColorAbsorbable` (Class)
    #[serde(default)]
    pub highlight_color_absorbable: Option<Handle<RGBA>>,
    /// `highlightColorDistant` (Class)
    #[serde(default)]
    pub highlight_color_distant: Option<Handle<RGBA>>,
    /// `highlightColorDistantScanned` (Class)
    #[serde(default)]
    pub highlight_color_distant_scanned: Option<Handle<RGBA>>,
    /// `highlightOccludedAlpha` (Single)
    #[serde(default)]
    pub highlight_occluded_alpha: f32,
    /// `highlightOutlineWidth` (Single)
    #[serde(default)]
    pub highlight_outline_width: f32,
    /// `highlightDistantMineablesRange` (Single)
    #[serde(default)]
    pub highlight_distant_mineables_range: f32,
    /// `cameraShakeConfig` (Class)
    #[serde(default)]
    pub camera_shake_config: Option<Handle<MiningCameraShakeConfig>>,
    /// `showChildRockRadarIcon` (Boolean)
    #[serde(default)]
    pub show_child_rock_radar_icon: bool,
    /// `scalePowerGraphMin` (Boolean)
    #[serde(default)]
    pub scale_power_graph_min: bool,
    /// `noProgressHintTime` (Single)
    #[serde(default)]
    pub no_progress_hint_time: f32,
    /// `noProgressHintPower` (Single)
    #[serde(default)]
    pub no_progress_hint_power: f32,
    /// `fractureDoneFeedbackDuration` (Single)
    #[serde(default)]
    pub fracture_done_feedback_duration: f32,
    /// `maxScanRaycastDistance` (Single)
    #[serde(default)]
    pub max_scan_raycast_distance: f32,
}

impl Pooled for MiningControllerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.mining.mining_controller_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.mining.mining_controller_global_params }
}

impl<'a> Extract<'a> for MiningControllerGlobalParams {
    const TYPE_NAME: &'static str = "MiningControllerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            highlight_color: match inst.get("highlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_color_absorbable: match inst.get("highlightColorAbsorbable") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_color_distant: match inst.get("highlightColorDistant") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_color_distant_scanned: match inst.get("highlightColorDistantScanned") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_occluded_alpha: inst.get_f32("highlightOccludedAlpha").unwrap_or_default(),
            highlight_outline_width: inst.get_f32("highlightOutlineWidth").unwrap_or_default(),
            highlight_distant_mineables_range: inst.get_f32("highlightDistantMineablesRange").unwrap_or_default(),
            camera_shake_config: match inst.get("cameraShakeConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MiningCameraShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MiningCameraShakeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            show_child_rock_radar_icon: inst.get_bool("showChildRockRadarIcon").unwrap_or_default(),
            scale_power_graph_min: inst.get_bool("scalePowerGraphMin").unwrap_or_default(),
            no_progress_hint_time: inst.get_f32("noProgressHintTime").unwrap_or_default(),
            no_progress_hint_power: inst.get_f32("noProgressHintPower").unwrap_or_default(),
            fracture_done_feedback_duration: inst.get_f32("fractureDoneFeedbackDuration").unwrap_or_default(),
            max_scan_raycast_distance: inst.get_f32("maxScanRaycastDistance").unwrap_or_default(),
        }
    }
}

