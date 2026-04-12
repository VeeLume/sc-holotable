// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `MistedBreathParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MistedBreathParams {
    /// DCB field: `apparentTemperatureTrigger` (Single)
    #[serde(default)]
    pub apparent_temperature_trigger: f32,
    /// DCB field: `minStrengthAtTemperature` (Single)
    #[serde(default)]
    pub min_strength_at_temperature: f32,
    /// DCB field: `maxStrengthAtTemperature` (Single)
    #[serde(default)]
    pub max_strength_at_temperature: f32,
    /// DCB field: `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
}

impl Pooled for MistedBreathParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.misted_breath_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.misted_breath_params }
}

impl<'a> Extract<'a> for MistedBreathParams {
    const TYPE_NAME: &'static str = "MistedBreathParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            apparent_temperature_trigger: inst.get_f32("apparentTemperatureTrigger").unwrap_or_default(),
            min_strength_at_temperature: inst.get_f32("minStrengthAtTemperature").unwrap_or_default(),
            max_strength_at_temperature: inst.get_f32("maxStrengthAtTemperature").unwrap_or_default(),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MineableExplosionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineableExplosionParams {
    /// DCB field: `defaultExplosionParams` (Class)
    #[serde(default)]
    pub default_explosion_params: Option<Handle<ExplosionParams>>,
    /// DCB field: `dangerPoolFactor` (Single)
    #[serde(default)]
    pub danger_pool_factor: f32,
    /// DCB field: `defaultVolume` (Single)
    #[serde(default)]
    pub default_volume: f32,
}

impl Pooled for MineableExplosionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mineable_explosion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mineable_explosion_params }
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
    /// DCB field: `instabilityWavePeriod` (Single)
    #[serde(default)]
    pub instability_wave_period: f32,
    /// DCB field: `instabilityWaveVariance` (Single)
    #[serde(default)]
    pub instability_wave_variance: f32,
    /// DCB field: `instabilityCurveFactor` (Single)
    #[serde(default)]
    pub instability_curve_factor: f32,
}

impl Pooled for MineableInstabilityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mineable_instability_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mineable_instability_params }
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
    /// DCB field: `powerCapacityPerMass` (Single)
    #[serde(default)]
    pub power_capacity_per_mass: f32,
    /// DCB field: `decayPerMass` (Single)
    #[serde(default)]
    pub decay_per_mass: f32,
    /// DCB field: `optimalWindowSize` (Single)
    #[serde(default)]
    pub optimal_window_size: f32,
    /// DCB field: `optimalWindowFactor` (Single)
    #[serde(default)]
    pub optimal_window_factor: f32,
    /// DCB field: `resistanceCurveFactor` (Single)
    #[serde(default)]
    pub resistance_curve_factor: f32,
    /// DCB field: `optimalWindowThinnessCurveFactor` (Single)
    #[serde(default)]
    pub optimal_window_thinness_curve_factor: f32,
    /// DCB field: `optimalWindowMaxSize` (Single)
    #[serde(default)]
    pub optimal_window_max_size: f32,
    /// DCB field: `controlledBreakingFillRate` (Single)
    #[serde(default)]
    pub controlled_breaking_fill_rate: f32,
    /// DCB field: `controlledBreakingFillRateDanger` (Single)
    #[serde(default)]
    pub controlled_breaking_fill_rate_danger: f32,
    /// DCB field: `controlledBreakingDecayRate` (Single)
    #[serde(default)]
    pub controlled_breaking_decay_rate: f32,
    /// DCB field: `dangerBreakingFillRate` (Single)
    #[serde(default)]
    pub danger_breaking_fill_rate: f32,
    /// DCB field: `dangerBreakingFillRateExponent` (Single)
    #[serde(default)]
    pub danger_breaking_fill_rate_exponent: f32,
    /// DCB field: `dangerBreakingDecayRate` (Single)
    #[serde(default)]
    pub danger_breaking_decay_rate: f32,
    /// DCB field: `absorbableVolumeThreshold` (Single)
    #[serde(default)]
    pub absorbable_volume_threshold: f32,
    /// DCB field: `mineableInstabilityParams` (Class)
    #[serde(default)]
    pub mineable_instability_params: Option<Handle<MineableInstabilityParams>>,
    /// DCB field: `mineableExplosionParams` (Class)
    #[serde(default)]
    pub mineable_explosion_params: Option<Handle<MineableExplosionParams>>,
    /// DCB field: `childRockInvulnerabilityTime` (Single)
    #[serde(default)]
    pub child_rock_invulnerability_time: f32,
    /// DCB field: `cSCUPerVolume` (Single)
    #[serde(default)]
    pub c_scuper_volume: f32,
    /// DCB field: `defaultMass` (Single)
    #[serde(default)]
    pub default_mass: f32,
    /// DCB field: `fractureParticleEffect` (Class)
    #[serde(default)]
    pub fracture_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `explosionParticleEffect` (Class)
    #[serde(default)]
    pub explosion_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `centerRockDestroyParticleEffect` (Class)
    #[serde(default)]
    pub center_rock_destroy_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `fullyExtractedRockParticleEffect` (Class)
    #[serde(default)]
    pub fully_extracted_rock_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `hitConsistencyParams` (Class)
    #[serde(default)]
    pub hit_consistency_params: Option<Handle<HitConsistencyParams>>,
    /// DCB field: `modifierPersistenceTime` (Single)
    #[serde(default)]
    pub modifier_persistence_time: f32,
    /// DCB field: `childRockLifeTimer` (Single)
    #[serde(default)]
    pub child_rock_life_timer: f32,
    /// DCB field: `childRockZeroGDamping` (Single)
    #[serde(default)]
    pub child_rock_zero_gdamping: f32,
    /// DCB field: `terrainFactorStaticThreshold` (Single)
    #[serde(default)]
    pub terrain_factor_static_threshold: f32,
    /// DCB field: `showExplosionFXForSurplusChild` (Boolean)
    #[serde(default)]
    pub show_explosion_fxfor_surplus_child: bool,
    /// DCB field: `childRockInactivityLifetime` (Single)
    #[serde(default)]
    pub child_rock_inactivity_lifetime: f32,
    /// DCB field: `gadgetDetachThreshold` (Single)
    #[serde(default)]
    pub gadget_detach_threshold: f32,
    /// DCB field: `gadgetDestroyThreshold` (Single)
    #[serde(default)]
    pub gadget_destroy_threshold: f32,
    /// DCB field: `dangerToGadgetDamage` (Single)
    #[serde(default)]
    pub danger_to_gadget_damage: f32,
    /// DCB field: `wasteResourceType` (Reference)
    #[serde(default)]
    pub waste_resource_type: Option<CigGuid>,
}

impl Pooled for MiningGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mining_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mining_global_params }
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

/// DCB type: `MiningAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningAudioParams {
    /// DCB field: `mineablePowerLevelRtpc` (Class)
    #[serde(default)]
    pub mineable_power_level_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineableDangerBreakingRtpc` (Class)
    #[serde(default)]
    pub mineable_danger_breaking_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineableOptimalBreakingRtpc` (Class)
    #[serde(default)]
    pub mineable_optimal_breaking_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineableMassRtpc` (Class)
    #[serde(default)]
    pub mineable_mass_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineablePowerIncreasingRtpc` (Class)
    #[serde(default)]
    pub mineable_power_increasing_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineableCrackGlowStrengthRtpc` (Class)
    #[serde(default)]
    pub mineable_crack_glow_strength_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineableBreakZoneIndicatorRtpc` (Class)
    #[serde(default)]
    pub mineable_break_zone_indicator_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineablePowerIncreasingFallOff` (Single)
    #[serde(default)]
    pub mineable_power_increasing_fall_off: f32,
    /// DCB field: `miningStartTrigger` (Class)
    #[serde(default)]
    pub mining_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `miningStopTrigger` (Class)
    #[serde(default)]
    pub mining_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `goodFracturedTrigger` (Class)
    #[serde(default)]
    pub good_fractured_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `badFracturedTrigger` (Class)
    #[serde(default)]
    pub bad_fractured_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `extractedTrigger` (Class)
    #[serde(default)]
    pub extracted_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for MiningAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mining_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mining_audio_params }
}

impl<'a> Extract<'a> for MiningAudioParams {
    const TYPE_NAME: &'static str = "MiningAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mineable_power_level_rtpc: match inst.get("mineablePowerLevelRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_danger_breaking_rtpc: match inst.get("mineableDangerBreakingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_optimal_breaking_rtpc: match inst.get("mineableOptimalBreakingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_mass_rtpc: match inst.get("mineableMassRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_power_increasing_rtpc: match inst.get("mineablePowerIncreasingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_crack_glow_strength_rtpc: match inst.get("mineableCrackGlowStrengthRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_break_zone_indicator_rtpc: match inst.get("mineableBreakZoneIndicatorRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mineable_power_increasing_fall_off: inst.get_f32("mineablePowerIncreasingFallOff").unwrap_or_default(),
            mining_start_trigger: match inst.get("miningStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mining_stop_trigger: match inst.get("miningStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            good_fractured_trigger: match inst.get("goodFracturedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bad_fractured_trigger: match inst.get("badFracturedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            extracted_trigger: match inst.get("extractedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MineableElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineableElement {
    /// DCB field: `resourceType` (Reference)
    #[serde(default)]
    pub resource_type: Option<CigGuid>,
    /// DCB field: `elementInstability` (Single)
    #[serde(default)]
    pub element_instability: f32,
    /// DCB field: `elementResistance` (Single)
    #[serde(default)]
    pub element_resistance: f32,
    /// DCB field: `elementOptimalWindowMidpoint` (Single)
    #[serde(default)]
    pub element_optimal_window_midpoint: f32,
    /// DCB field: `elementOptimalWindowMidpointRandomness` (Single)
    #[serde(default)]
    pub element_optimal_window_midpoint_randomness: f32,
    /// DCB field: `elementOptimalWindowThinness` (Single)
    #[serde(default)]
    pub element_optimal_window_thinness: f32,
    /// DCB field: `elementExplosionMultiplier` (Single)
    #[serde(default)]
    pub element_explosion_multiplier: f32,
    /// DCB field: `elementClusterFactor` (Single)
    #[serde(default)]
    pub element_cluster_factor: f32,
}

impl Pooled for MineableElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mineable_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mineable_element }
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
    /// DCB field: `mineableElement` (Reference)
    #[serde(default)]
    pub mineable_element: Option<CigGuid>,
    /// DCB field: `minPercentage` (Single)
    #[serde(default)]
    pub min_percentage: f32,
    /// DCB field: `maxPercentage` (Single)
    #[serde(default)]
    pub max_percentage: f32,
    /// DCB field: `probability` (Single)
    #[serde(default)]
    pub probability: f32,
    /// DCB field: `curveExponent` (Single)
    #[serde(default)]
    pub curve_exponent: f32,
    /// DCB field: `qualityScale` (Single)
    #[serde(default)]
    pub quality_scale: f32,
}

impl Pooled for MineableCompositionPart {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mineable_composition_part }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mineable_composition_part }
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
    /// DCB field: `depositName` (Locale)
    #[serde(default)]
    pub deposit_name: String,
    /// DCB field: `minimumDistinctElements` (Int32)
    #[serde(default)]
    pub minimum_distinct_elements: i32,
    /// DCB field: `compositionArray` (Class (array))
    #[serde(default)]
    pub composition_array: Vec<Handle<MineableCompositionPart>>,
}

impl Pooled for MineableComposition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mineable_composition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mineable_composition }
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
    /// DCB field: `blockThrottleChangeWhenNotFiring` (Boolean)
    #[serde(default)]
    pub block_throttle_change_when_not_firing: bool,
    /// DCB field: `throttleResetOnStopFire` (Boolean)
    #[serde(default)]
    pub throttle_reset_on_stop_fire: bool,
    /// DCB field: `throttleChangePerAction` (Single)
    #[serde(default)]
    pub throttle_change_per_action: f32,
    /// DCB field: `throttleAccPeriod` (Single)
    #[serde(default)]
    pub throttle_acc_period: f32,
    /// DCB field: `throttleAccFactor` (Single)
    #[serde(default)]
    pub throttle_acc_factor: f32,
    /// DCB field: `throttleHoldAccFactor` (Single)
    #[serde(default)]
    pub throttle_hold_acc_factor: f32,
    /// DCB field: `throttleRTPC` (Class)
    #[serde(default)]
    pub throttle_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineableGlowStrengthRTPC` (Class)
    #[serde(default)]
    pub mineable_glow_strength_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `mineableOptimalBreakZoneRTPC` (Class)
    #[serde(default)]
    pub mineable_optimal_break_zone_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for MiningLaserGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mining_laser_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mining_laser_global_params }
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

/// DCB type: `MissionLocationTags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationTags {
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for MissionLocationTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_location_tags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_location_tags }
}

impl<'a> Extract<'a> for MissionLocationTags {
    const TYPE_NAME: &'static str = "MissionLocationTags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionStringVariant`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionStringVariant {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `string` (Locale)
    #[serde(default)]
    pub string: String,
}

impl Pooled for MissionStringVariant {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_string_variant }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_string_variant }
}

impl<'a> Extract<'a> for MissionStringVariant {
    const TYPE_NAME: &'static str = "MissionStringVariant";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            string: inst.get_str("string").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionStringVariants`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionStringVariants {
    /// DCB field: `variants` (Class (array))
    #[serde(default)]
    pub variants: Vec<Handle<MissionStringVariant>>,
}

impl Pooled for MissionStringVariants {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_string_variants }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_string_variants }
}

impl<'a> Extract<'a> for MissionStringVariants {
    const TYPE_NAME: &'static str = "MissionStringVariants";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variants: inst.get_array("variants")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionStringVariant>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionStringVariant>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionLocationData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationData {
    /// DCB field: `generalTags` (Class)
    #[serde(default)]
    pub general_tags: Option<Handle<MissionLocationTags>>,
    /// DCB field: `producesTags` (Class)
    #[serde(default)]
    pub produces_tags: Option<Handle<TagSearchTerm>>,
    /// DCB field: `consumesTags` (Class)
    #[serde(default)]
    pub consumes_tags: Option<Handle<TagSearchTerm>>,
    /// DCB field: `aiSpawnTags` (Class)
    #[serde(default)]
    pub ai_spawn_tags: Option<Handle<TagList>>,
    /// DCB field: `stringVariants` (Class)
    #[serde(default)]
    pub string_variants: Option<Handle<MissionStringVariants>>,
    /// DCB field: `missionModules` (Class (array))
    #[serde(default)]
    pub mission_modules: Vec<Handle<SMissionLocationModule>>,
    /// DCB field: `missionLimits` (Class (array))
    #[serde(default)]
    pub mission_limits: Vec<Handle<LocationMissionLimit>>,
    /// DCB field: `autoSpawnSettings` (Class (array))
    #[serde(default)]
    pub auto_spawn_settings: Vec<Handle<AutoSpawnSettings>>,
    /// DCB field: `isSecurityNetworkHost` (Boolean)
    #[serde(default)]
    pub is_security_network_host: bool,
    /// DCB field: `defaultSecurityNetworkManifest` (Reference)
    #[serde(default)]
    pub default_security_network_manifest: Option<CigGuid>,
    /// DCB field: `disabled` (Boolean)
    #[serde(default)]
    pub disabled: bool,
    /// DCB field: `entityClusterMember` (Reference)
    #[serde(default)]
    pub entity_cluster_member: Option<CigGuid>,
}

impl Pooled for MissionLocationData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_location_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_location_data }
}

impl<'a> Extract<'a> for MissionLocationData {
    const TYPE_NAME: &'static str = "MissionLocationData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            general_tags: match inst.get("generalTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionLocationTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionLocationTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            produces_tags: match inst.get("producesTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagSearchTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagSearchTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            consumes_tags: match inst.get("consumesTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagSearchTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagSearchTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ai_spawn_tags: match inst.get("aiSpawnTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            string_variants: match inst.get("stringVariants") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionStringVariants>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionStringVariants>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mission_modules: inst.get_array("missionModules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMissionLocationModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SMissionLocationModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            mission_limits: inst.get_array("missionLimits")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LocationMissionLimit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LocationMissionLimit>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            auto_spawn_settings: inst.get_array("autoSpawnSettings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AutoSpawnSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AutoSpawnSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            is_security_network_host: inst.get_bool("isSecurityNetworkHost").unwrap_or_default(),
            default_security_network_manifest: inst.get("defaultSecurityNetworkManifest").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            disabled: inst.get_bool("disabled").unwrap_or_default(),
            entity_cluster_member: inst.get("entityClusterMember").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionLocationTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationTemplate {
    /// DCB field: `locationData` (Class)
    #[serde(default)]
    pub location_data: Option<Handle<MissionLocationData>>,
}

impl Pooled for MissionLocationTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_location_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_location_template }
}

impl<'a> Extract<'a> for MissionLocationTemplate {
    const TYPE_NAME: &'static str = "MissionLocationTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_data: match inst.get("locationData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionLocationData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionLocationData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionLocationValidation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationValidation {
}

impl Pooled for MissionLocationValidation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_location_validation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_location_validation }
}

impl<'a> Extract<'a> for MissionLocationValidation {
    const TYPE_NAME: &'static str = "MissionLocationValidation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MissionItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionItem {
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// DCB field: `stringVariants` (Class)
    #[serde(default)]
    pub string_variants: Option<Handle<MissionStringVariants>>,
    /// DCB field: `weighting` (Single)
    #[serde(default)]
    pub weighting: f32,
    /// DCB field: `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
}

impl Pooled for MissionItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_item }
}

impl<'a> Extract<'a> for MissionItem {
    const TYPE_NAME: &'static str = "MissionItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            string_variants: match inst.get("stringVariants") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionStringVariants>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionStringVariants>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weighting: inst.get_f32("weighting").unwrap_or_default(),
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionOrganization`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionOrganization {
    /// DCB field: `organizationTags` (Class)
    #[serde(default)]
    pub organization_tags: Option<Handle<MissionLocationTags>>,
    /// DCB field: `stringVariants` (Class)
    #[serde(default)]
    pub string_variants: Option<Handle<MissionStringVariants>>,
    /// DCB field: `weighting` (Single)
    #[serde(default)]
    pub weighting: f32,
    /// DCB field: `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
}

impl Pooled for MissionOrganization {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_organization }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_organization }
}

impl<'a> Extract<'a> for MissionOrganization {
    const TYPE_NAME: &'static str = "MissionOrganization";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            organization_tags: match inst.get("organizationTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionLocationTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionLocationTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            string_variants: match inst.get("stringVariants") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionStringVariants>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionStringVariants>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weighting: inst.get_f32("weighting").unwrap_or_default(),
            faction_reputation: inst.get("factionReputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionFailConditionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFailConditionParams {
    /// DCB field: `triggerCondition` (Reference)
    #[serde(default)]
    pub trigger_condition: Option<CigGuid>,
    /// DCB field: `warningLevel` (Int32)
    #[serde(default)]
    pub warning_level: i32,
    /// DCB field: `displayText` (Locale)
    #[serde(default)]
    pub display_text: String,
    /// DCB field: `useAutomaticFailureScreen` (Boolean)
    #[serde(default)]
    pub use_automatic_failure_screen: bool,
}

impl Pooled for MissionFailConditionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_fail_condition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_fail_condition_params }
}

impl<'a> Extract<'a> for MissionFailConditionParams {
    const TYPE_NAME: &'static str = "MissionFailConditionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger_condition: inst.get("triggerCondition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            warning_level: inst.get_i32("warningLevel").unwrap_or_default(),
            display_text: inst.get_str("displayText").map(String::from).unwrap_or_default(),
            use_automatic_failure_screen: inst.get_bool("useAutomaticFailureScreen").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionFailConditionsList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFailConditionsList {
    /// DCB field: `failureConditions` (Class (array))
    #[serde(default)]
    pub failure_conditions: Vec<Handle<MissionFailConditionParams>>,
}

impl Pooled for MissionFailConditionsList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_fail_conditions_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_fail_conditions_list }
}

impl<'a> Extract<'a> for MissionFailConditionsList {
    const TYPE_NAME: &'static str = "MissionFailConditionsList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            failure_conditions: inst.get_array("failureConditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionFailConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionFailConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionFlowConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFlowConditionBase {
}

impl Pooled for MissionFlowConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_flow_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_flow_condition_base }
}

impl<'a> Extract<'a> for MissionFlowConditionBase {
    const TYPE_NAME: &'static str = "MissionFlowConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MissionFlowActionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFlowActionBase {
}

impl Pooled for MissionFlowActionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_flow_action_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_flow_action_base }
}

impl<'a> Extract<'a> for MissionFlowActionBase {
    const TYPE_NAME: &'static str = "MissionFlowActionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MissionFlowTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFlowTrigger {
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `condition` (StrongPointer)
    #[serde(default)]
    pub condition: Option<Handle<MissionFlowConditionBase>>,
    /// DCB field: `action` (StrongPointer)
    #[serde(default)]
    pub action: Option<Handle<MissionFlowActionBase>>,
}

impl Pooled for MissionFlowTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_flow_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_flow_trigger }
}

impl<'a> Extract<'a> for MissionFlowTrigger {
    const TYPE_NAME: &'static str = "MissionFlowTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            condition: match inst.get("condition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionFlowConditionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionFlowConditionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            action: match inst.get("action") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionFlowActionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionFlowActionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionFlow`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFlow {
    /// DCB field: `triggers` (Class (array))
    #[serde(default)]
    pub triggers: Vec<Handle<MissionFlowTrigger>>,
}

impl Pooled for MissionFlow {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_flow }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_flow }
}

impl<'a> Extract<'a> for MissionFlow {
    const TYPE_NAME: &'static str = "MissionFlow";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            triggers: inst.get_array("triggers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionFlowTrigger>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionFlowTrigger>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionPropertyValue_AIName`
///
/// Inherits from: `BaseMissionPropertyValue` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionPropertyValue_AIName {
    /// DCB field: `randomName` (Boolean)
    #[serde(default)]
    pub random_name: bool,
    /// DCB field: `randomLastName` (Boolean)
    #[serde(default)]
    pub random_last_name: bool,
    /// DCB field: `randomNickName` (Boolean)
    #[serde(default)]
    pub random_nick_name: bool,
    /// DCB field: `characterGivenName` (Locale)
    #[serde(default)]
    pub character_given_name: String,
    /// DCB field: `characterGivenLastName` (Locale)
    #[serde(default)]
    pub character_given_last_name: String,
    /// DCB field: `characterGivenNickName` (Locale)
    #[serde(default)]
    pub character_given_nick_name: String,
    /// DCB field: `characterNameData` (Reference)
    #[serde(default)]
    pub character_name_data: Option<CigGuid>,
    /// DCB field: `chanceOfNickName` (Single)
    #[serde(default)]
    pub chance_of_nick_name: f32,
}

impl Pooled for MissionPropertyValue_AIName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_property_value_ainame }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_property_value_ainame }
}

impl<'a> Extract<'a> for MissionPropertyValue_AIName {
    const TYPE_NAME: &'static str = "MissionPropertyValue_AIName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            random_name: inst.get_bool("randomName").unwrap_or_default(),
            random_last_name: inst.get_bool("randomLastName").unwrap_or_default(),
            random_nick_name: inst.get_bool("randomNickName").unwrap_or_default(),
            character_given_name: inst.get_str("characterGivenName").map(String::from).unwrap_or_default(),
            character_given_last_name: inst.get_str("characterGivenLastName").map(String::from).unwrap_or_default(),
            character_given_nick_name: inst.get_str("characterGivenNickName").map(String::from).unwrap_or_default(),
            character_name_data: inst.get("characterNameData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            chance_of_nick_name: inst.get_f32("chanceOfNickName").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionProperty`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionProperty {
    /// DCB field: `missionVariableName` (String)
    #[serde(default)]
    pub mission_variable_name: String,
    /// DCB field: `extendedTextToken` (String)
    #[serde(default)]
    pub extended_text_token: String,
    /// DCB field: `value` (StrongPointer)
    #[serde(default)]
    pub value: Option<Handle<BaseMissionPropertyValue>>,
}

impl Pooled for MissionProperty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_property }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_property }
}

impl<'a> Extract<'a> for MissionProperty {
    const TYPE_NAME: &'static str = "MissionProperty";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_variable_name: inst.get_str("missionVariableName").map(String::from).unwrap_or_default(),
            extended_text_token: inst.get_str("extendedTextToken").map(String::from).unwrap_or_default(),
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseMissionPropertyValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseMissionPropertyValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionVariableBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionVariableBase {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
}

impl Pooled for MissionVariableBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_variable_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_variable_base }
}

impl<'a> Extract<'a> for MissionVariableBase {
    const TYPE_NAME: &'static str = "MissionVariableBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionModuleHierarchySubMission`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionModuleHierarchySubMission {
    /// DCB field: `subMissionModule` (String)
    #[serde(default)]
    pub sub_mission_module: String,
    /// DCB field: `subModuleHierarchy` (Reference)
    #[serde(default)]
    pub sub_module_hierarchy: Option<CigGuid>,
}

impl Pooled for MissionModuleHierarchySubMission {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_module_hierarchy_sub_mission }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_module_hierarchy_sub_mission }
}

impl<'a> Extract<'a> for MissionModuleHierarchySubMission {
    const TYPE_NAME: &'static str = "MissionModuleHierarchySubMission";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sub_mission_module: inst.get_str("subMissionModule").map(String::from).unwrap_or_default(),
            sub_module_hierarchy: inst.get("subModuleHierarchy").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionModuleHierarchy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionModuleHierarchy {
    /// DCB field: `missionModule` (String)
    #[serde(default)]
    pub mission_module: String,
    /// DCB field: `subMissionModules` (Class (array))
    #[serde(default)]
    pub sub_mission_modules: Vec<Handle<MissionModuleHierarchySubMission>>,
}

impl Pooled for MissionModuleHierarchy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_module_hierarchy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_module_hierarchy }
}

impl<'a> Extract<'a> for MissionModuleHierarchy {
    const TYPE_NAME: &'static str = "MissionModuleHierarchy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_module: inst.get_str("missionModule").map(String::from).unwrap_or_default(),
            sub_mission_modules: inst.get_array("subMissionModules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionModuleHierarchySubMission>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionModuleHierarchySubMission>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioCyclePhase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioCyclePhase {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `duration_seconds` (UInt32)
    #[serde(default)]
    pub duration_seconds: u32,
}

impl Pooled for MissionScenarioCyclePhase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_scenario_cycle_phase }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_scenario_cycle_phase }
}

impl<'a> Extract<'a> for MissionScenarioCyclePhase {
    const TYPE_NAME: &'static str = "MissionScenarioCyclePhase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            duration_seconds: inst.get_u32("duration_seconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioCycle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioCycle {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `phases` (Class (array))
    #[serde(default)]
    pub phases: Vec<Handle<MissionScenarioCyclePhase>>,
}

impl Pooled for MissionScenarioCycle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_scenario_cycle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_scenario_cycle }
}

impl<'a> Extract<'a> for MissionScenarioCycle {
    const TYPE_NAME: &'static str = "MissionScenarioCycle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            phases: inst.get_array("phases")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionScenarioCyclePhase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionScenarioCyclePhase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioScheduleConstraint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioScheduleConstraint {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `epoch` (Int32)
    #[serde(default)]
    pub epoch: i32,
}

impl Pooled for MissionScenarioScheduleConstraint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_scenario_schedule_constraint }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_scenario_schedule_constraint }
}

impl<'a> Extract<'a> for MissionScenarioScheduleConstraint {
    const TYPE_NAME: &'static str = "MissionScenarioScheduleConstraint";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            epoch: inst.get_i32("epoch").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioScheduleRecurrence`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioScheduleRecurrence {
    /// DCB field: `cron` (String)
    #[serde(default)]
    pub cron: String,
    /// DCB field: `duration_seconds` (UInt32)
    #[serde(default)]
    pub duration_seconds: u32,
}

impl Pooled for MissionScenarioScheduleRecurrence {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_scenario_schedule_recurrence }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_scenario_schedule_recurrence }
}

impl<'a> Extract<'a> for MissionScenarioScheduleRecurrence {
    const TYPE_NAME: &'static str = "MissionScenarioScheduleRecurrence";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            cron: inst.get_str("cron").map(String::from).unwrap_or_default(),
            duration_seconds: inst.get_u32("duration_seconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioSchedule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioSchedule {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `start_constraint` (StrongPointer)
    #[serde(default)]
    pub start_constraint: Option<Handle<MissionScenarioScheduleConstraint>>,
    /// DCB field: `end_constraint` (StrongPointer)
    #[serde(default)]
    pub end_constraint: Option<Handle<MissionScenarioScheduleConstraint>>,
    /// DCB field: `recurrence` (StrongPointer)
    #[serde(default)]
    pub recurrence: Option<Handle<MissionScenarioScheduleRecurrence>>,
}

impl Pooled for MissionScenarioSchedule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_scenario_schedule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_scenario_schedule }
}

impl<'a> Extract<'a> for MissionScenarioSchedule {
    const TYPE_NAME: &'static str = "MissionScenarioSchedule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            start_constraint: match inst.get("start_constraint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end_constraint: match inst.get("end_constraint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            recurrence: match inst.get("recurrence") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioScheduleRecurrence>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioScheduleRecurrence>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionScenario`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenario {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `variables` (StrongPointer (array))
    #[serde(default)]
    pub variables: Vec<Handle<MissionVariableBase>>,
    /// DCB field: `cycles` (Class (array))
    #[serde(default)]
    pub cycles: Vec<Handle<MissionScenarioCycle>>,
    /// DCB field: `schedule` (StrongPointer)
    #[serde(default)]
    pub schedule: Option<Handle<MissionScenarioSchedule>>,
    /// DCB field: `auto_create` (Boolean)
    #[serde(default)]
    pub auto_create: bool,
    /// DCB field: `track_progress` (Boolean)
    #[serde(default)]
    pub track_progress: bool,
}

impl Pooled for MissionScenario {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_scenario }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_scenario }
}

impl<'a> Extract<'a> for MissionScenario {
    const TYPE_NAME: &'static str = "MissionScenario";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            variables: inst.get_array("variables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionVariableBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionVariableBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cycles: inst.get_array("cycles")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionScenarioCycle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionScenarioCycle>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            schedule: match inst.get("schedule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioSchedule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioSchedule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_create: inst.get_bool("auto_create").unwrap_or_default(),
            track_progress: inst.get_bool("track_progress").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionType {
    /// DCB field: `IconName` (String)
    #[serde(default)]
    pub icon_name: String,
    /// DCB field: `LocalisedTypeName` (Locale)
    #[serde(default)]
    pub localised_type_name: String,
    /// DCB field: `svgIconPath` (String)
    #[serde(default)]
    pub svg_icon_path: String,
    /// DCB field: `DisplayTime` (Single)
    #[serde(default)]
    pub display_time: f32,
}

impl Pooled for MissionType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_type }
}

impl<'a> Extract<'a> for MissionType {
    const TYPE_NAME: &'static str = "MissionType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            icon_name: inst.get_str("IconName").map(String::from).unwrap_or_default(),
            localised_type_name: inst.get_str("LocalisedTypeName").map(String::from).unwrap_or_default(),
            svg_icon_path: inst.get_str("svgIconPath").map(String::from).unwrap_or_default(),
            display_time: inst.get_f32("DisplayTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionDeadline`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDeadline {
    /// DCB field: `missionCompletionTime` (Single)
    #[serde(default)]
    pub mission_completion_time: f32,
    /// DCB field: `missionAutoEnd` (Boolean)
    #[serde(default)]
    pub mission_auto_end: bool,
    /// DCB field: `missionResultAfterTimerEnd` (EnumChoice)
    #[serde(default)]
    pub mission_result_after_timer_end: String,
    /// DCB field: `remainingTimeToShowTimer` (Single)
    #[serde(default)]
    pub remaining_time_to_show_timer: f32,
    /// DCB field: `missionEndReason` (Locale)
    #[serde(default)]
    pub mission_end_reason: String,
}

impl Pooled for MissionDeadline {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_deadline }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_deadline }
}

impl<'a> Extract<'a> for MissionDeadline {
    const TYPE_NAME: &'static str = "MissionDeadline";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_completion_time: inst.get_f32("missionCompletionTime").unwrap_or_default(),
            mission_auto_end: inst.get_bool("missionAutoEnd").unwrap_or_default(),
            mission_result_after_timer_end: inst.get_str("missionResultAfterTimerEnd").map(String::from).unwrap_or_default(),
            remaining_time_to_show_timer: inst.get_f32("remainingTimeToShowTimer").unwrap_or_default(),
            mission_end_reason: inst.get_str("missionEndReason").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionReward`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionReward {
    /// DCB field: `reward` (Int32)
    #[serde(default)]
    pub reward: i32,
    /// DCB field: `max` (Int32)
    #[serde(default)]
    pub max: i32,
    /// DCB field: `plusBonuses` (Boolean)
    #[serde(default)]
    pub plus_bonuses: bool,
    /// DCB field: `currencyType` (EnumChoice)
    #[serde(default)]
    pub currency_type: String,
    /// DCB field: `reputationBonus` (Reference)
    #[serde(default)]
    pub reputation_bonus: Option<CigGuid>,
}

impl Pooled for MissionReward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_reward }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_reward }
}

impl<'a> Extract<'a> for MissionReward {
    const TYPE_NAME: &'static str = "MissionReward";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reward: inst.get_i32("reward").unwrap_or_default(),
            max: inst.get_i32("max").unwrap_or_default(),
            plus_bonuses: inst.get_bool("plusBonuses").unwrap_or_default(),
            currency_type: inst.get_str("currencyType").map(String::from).unwrap_or_default(),
            reputation_bonus: inst.get("reputationBonus").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionLocality`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocality {
    /// DCB field: `availableLocations` (Reference (array))
    #[serde(default)]
    pub available_locations: Vec<CigGuid>,
}

impl Pooled for MissionLocality {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_locality }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_locality }
}

impl<'a> Extract<'a> for MissionLocality {
    const TYPE_NAME: &'static str = "MissionLocality";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            available_locations: inst.get_array("availableLocations")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MinRequiredMissions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinRequiredMissions {
    /// DCB field: `minRequiredCompletedMissions` (Int32)
    #[serde(default)]
    pub min_required_completed_missions: i32,
    /// DCB field: `completionTags` (Class)
    #[serde(default)]
    pub completion_tags: Option<Handle<TagList>>,
    /// DCB field: `requiredMissions` (Reference (array))
    #[serde(default)]
    pub required_missions: Vec<CigGuid>,
    /// DCB field: `journalEntryLabel` (Locale)
    #[serde(default)]
    pub journal_entry_label: String,
}

impl Pooled for MinRequiredMissions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.min_required_missions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.min_required_missions }
}

impl<'a> Extract<'a> for MinRequiredMissions {
    const TYPE_NAME: &'static str = "MinRequiredMissions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_required_completed_missions: inst.get_i32("minRequiredCompletedMissions").unwrap_or_default(),
            completion_tags: match inst.get("completionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            required_missions: inst.get_array("requiredMissions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            journal_entry_label: inst.get_str("journalEntryLabel").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionBrokerEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionBrokerEntry {
    /// DCB field: `notForRelease` (Boolean)
    #[serde(default)]
    pub not_for_release: bool,
    /// DCB field: `owner` (Reference)
    #[serde(default)]
    pub owner: Option<CigGuid>,
    /// DCB field: `missionModule` (String)
    #[serde(default)]
    pub mission_module: String,
    /// DCB field: `playerFacingDebugName` (String)
    #[serde(default)]
    pub player_facing_debug_name: String,
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `titleHUD` (Locale)
    #[serde(default)]
    pub title_hud: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `missionGiver` (Locale)
    #[serde(default)]
    pub mission_giver: String,
    /// DCB field: `commsChannelName` (Locale)
    #[serde(default)]
    pub comms_channel_name: String,
    /// DCB field: `type` (Reference)
    #[serde(default)]
    pub r#type: Option<CigGuid>,
    /// DCB field: `associatedMissions` (Reference (array))
    #[serde(default)]
    pub associated_missions: Vec<CigGuid>,
    /// DCB field: `missionDifficulty` (Int32)
    #[serde(default)]
    pub mission_difficulty: i32,
    /// DCB field: `localityAvailable` (Reference)
    #[serde(default)]
    pub locality_available: Option<CigGuid>,
    /// DCB field: `locationMissionAvailable` (Reference)
    #[serde(default)]
    pub location_mission_available: Option<CigGuid>,
    /// DCB field: `availableDateSchedule` (Class (array))
    #[serde(default)]
    pub available_date_schedule: Vec<Handle<DateTimeSchedule>>,
    /// DCB field: `onlyAvailableIfAllMissionsNotAvailable` (Reference (array))
    #[serde(default)]
    pub only_available_if_all_missions_not_available: Vec<CigGuid>,
    /// DCB field: `missionReward` (Class)
    #[serde(default)]
    pub mission_reward: Option<Handle<MissionReward>>,
    /// DCB field: `partialRewardPayout` (StrongPointer)
    #[serde(default)]
    pub partial_reward_payout: Option<Handle<PartialContractRewards>>,
    /// DCB field: `missionResultReputationRewards` (Class)
    #[serde(default)]
    pub mission_result_reputation_rewards: Option<Handle<SReputationAmountListParams>>,
    /// DCB field: `journalEntriesToAddOnComplete` (Reference (array))
    #[serde(default)]
    pub journal_entries_to_add_on_complete: Vec<CigGuid>,
    /// DCB field: `journalEntriesToRemoveOnComplete` (Reference (array))
    #[serde(default)]
    pub journal_entries_to_remove_on_complete: Vec<CigGuid>,
    /// DCB field: `initiallyActive` (Boolean)
    #[serde(default)]
    pub initially_active: bool,
    /// DCB field: `notifyOnAvailable` (Boolean)
    #[serde(default)]
    pub notify_on_available: bool,
    /// DCB field: `preShowObjectives` (Boolean)
    #[serde(default)]
    pub pre_show_objectives: bool,
    /// DCB field: `showAsOffer` (Boolean)
    #[serde(default)]
    pub show_as_offer: bool,
    /// DCB field: `missionBuyInAmount` (Int32)
    #[serde(default)]
    pub mission_buy_in_amount: i32,
    /// DCB field: `refundBuyInOnWithdraw` (Boolean)
    #[serde(default)]
    pub refund_buy_in_on_withdraw: bool,
    /// DCB field: `hasCompleteButton` (Boolean)
    #[serde(default)]
    pub has_complete_button: bool,
    /// DCB field: `onlyOwnerCanComplete` (Boolean)
    #[serde(default)]
    pub only_owner_can_complete: bool,
    /// DCB field: `handlesAbandonRequest` (Boolean)
    #[serde(default)]
    pub handles_abandon_request: bool,
    /// DCB field: `missionModulePerPlayer` (Boolean)
    #[serde(default)]
    pub mission_module_per_player: bool,
    /// DCB field: `maxInstances` (Int32)
    #[serde(default)]
    pub max_instances: i32,
    /// DCB field: `maxPlayersPerInstance` (Int32)
    #[serde(default)]
    pub max_players_per_instance: i32,
    /// DCB field: `maxInstancesPerPlayer` (Int32)
    #[serde(default)]
    pub max_instances_per_player: i32,
    /// DCB field: `canBeShared` (Boolean)
    #[serde(default)]
    pub can_be_shared: bool,
    /// DCB field: `onceOnly` (Boolean)
    #[serde(default)]
    pub once_only: bool,
    /// DCB field: `tutorial` (Boolean)
    #[serde(default)]
    pub tutorial: bool,
    /// DCB field: `missionDeadline` (Class)
    #[serde(default)]
    pub mission_deadline: Option<Handle<MissionDeadline>>,
    /// DCB field: `displayAlliedMarkers` (Boolean)
    #[serde(default)]
    pub display_allied_markers: bool,
    /// DCB field: `availableInPrison` (Boolean)
    #[serde(default)]
    pub available_in_prison: bool,
    /// DCB field: `failIfSentToPrison` (Boolean)
    #[serde(default)]
    pub fail_if_sent_to_prison: bool,
    /// DCB field: `failIfBecameCriminal` (Boolean)
    #[serde(default)]
    pub fail_if_became_criminal: bool,
    /// DCB field: `failIfLeavePrison` (Boolean)
    #[serde(default)]
    pub fail_if_leave_prison: bool,
    /// DCB field: `completionTags` (Class)
    #[serde(default)]
    pub completion_tags: Option<Handle<TagList>>,
    /// DCB field: `applyCompletionTagsOnFailed` (Boolean)
    #[serde(default)]
    pub apply_completion_tags_on_failed: bool,
    /// DCB field: `applyCompletionTagsOnAbandoned` (Boolean)
    #[serde(default)]
    pub apply_completion_tags_on_abandoned: bool,
    /// DCB field: `requestOnly` (Boolean)
    #[serde(default)]
    pub request_only: bool,
    /// DCB field: `respawnTime` (Single)
    #[serde(default)]
    pub respawn_time: f32,
    /// DCB field: `respawnTimeVariation` (Single)
    #[serde(default)]
    pub respawn_time_variation: f32,
    /// DCB field: `instanceHasLifeTime` (Boolean)
    #[serde(default)]
    pub instance_has_life_time: bool,
    /// DCB field: `showLifeTimeInMobiGlas` (Boolean)
    #[serde(default)]
    pub show_life_time_in_mobi_glas: bool,
    /// DCB field: `instanceLifeTime` (Single)
    #[serde(default)]
    pub instance_life_time: f32,
    /// DCB field: `instanceLifeTimeVariation` (Single)
    #[serde(default)]
    pub instance_life_time_variation: f32,
    /// DCB field: `canReacceptAfterAbandoning` (Boolean)
    #[serde(default)]
    pub can_reaccept_after_abandoning: bool,
    /// DCB field: `abandonedCooldownTime` (Single)
    #[serde(default)]
    pub abandoned_cooldown_time: f32,
    /// DCB field: `abandonedCooldownTimeVariation` (Single)
    #[serde(default)]
    pub abandoned_cooldown_time_variation: f32,
    /// DCB field: `canReacceptAfterFailing` (Boolean)
    #[serde(default)]
    pub can_reaccept_after_failing: bool,
    /// DCB field: `hasPersonalCooldown` (Boolean)
    #[serde(default)]
    pub has_personal_cooldown: bool,
    /// DCB field: `personalCooldownTime` (Single)
    #[serde(default)]
    pub personal_cooldown_time: f32,
    /// DCB field: `personalCooldownTimeVariation` (Single)
    #[serde(default)]
    pub personal_cooldown_time_variation: f32,
    /// DCB field: `moduleHandlesOwnShutdown` (Boolean)
    #[serde(default)]
    pub module_handles_own_shutdown: bool,
    /// DCB field: `linkedMission` (Reference)
    #[serde(default)]
    pub linked_mission: Option<CigGuid>,
    /// DCB field: `missionCompletePerk` (StrongPointer)
    #[serde(default)]
    pub mission_complete_perk: Option<Handle<MissionCompletePerkBaseDef>>,
    /// DCB field: `modifiers` (StrongPointer (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<BaseMissionModifier>>,
    /// DCB field: `lawfulMission` (Boolean)
    #[serde(default)]
    pub lawful_mission: bool,
    /// DCB field: `missionGiverRecord` (Reference)
    #[serde(default)]
    pub mission_giver_record: Option<CigGuid>,
    /// DCB field: `invitationMission` (Reference)
    #[serde(default)]
    pub invitation_mission: Option<CigGuid>,
    /// DCB field: `missionTags` (Reference (array))
    #[serde(default)]
    pub mission_tags: Vec<CigGuid>,
    /// DCB field: `missionGiverFragmentTags` (String)
    #[serde(default)]
    pub mission_giver_fragment_tags: String,
    /// DCB field: `reputationPrerequisites` (Class)
    #[serde(default)]
    pub reputation_prerequisites: Option<Handle<ReputationPrerequisites>>,
    /// DCB field: `reputationRequirements` (StrongPointer)
    #[serde(default)]
    pub reputation_requirements: Option<Handle<SReputationMissionRequirementsParams>>,
    /// DCB field: `minRequiredMissions` (Class (array))
    #[serde(default)]
    pub min_required_missions: Vec<Handle<MinRequiredMissions>>,
    /// DCB field: `requiredMissions` (Reference (array))
    #[serde(default)]
    pub required_missions: Vec<CigGuid>,
    /// DCB field: `requiredCompletedMissionTags` (Class (array))
    #[serde(default)]
    pub required_completed_mission_tags: Vec<Handle<TagSearchTerm>>,
    /// DCB field: `requiredJournalEntries` (Reference (array))
    #[serde(default)]
    pub required_journal_entries: Vec<CigGuid>,
    /// DCB field: `requiredAreaTags` (Class)
    #[serde(default)]
    pub required_area_tags: Option<Handle<TagList>>,
    /// DCB field: `excludedAreaTags` (Class)
    #[serde(default)]
    pub excluded_area_tags: Option<Handle<TagList>>,
    /// DCB field: `properties` (Class (array))
    #[serde(default)]
    pub properties: Vec<Handle<MissionProperty>>,
    /// DCB field: `objectiveTokens` (Class (array))
    #[serde(default)]
    pub objective_tokens: Vec<Handle<ObjectiveToken>>,
    /// DCB field: `missionFlow` (Class)
    #[serde(default)]
    pub mission_flow: Option<Handle<MissionFlow>>,
}

impl Pooled for MissionBrokerEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_broker_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_broker_entry }
}

impl<'a> Extract<'a> for MissionBrokerEntry {
    const TYPE_NAME: &'static str = "MissionBrokerEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            owner: inst.get("owner").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_module: inst.get_str("missionModule").map(String::from).unwrap_or_default(),
            player_facing_debug_name: inst.get_str("playerFacingDebugName").map(String::from).unwrap_or_default(),
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            title_hud: inst.get_str("titleHUD").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            mission_giver: inst.get_str("missionGiver").map(String::from).unwrap_or_default(),
            comms_channel_name: inst.get_str("commsChannelName").map(String::from).unwrap_or_default(),
            r#type: inst.get("type").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            associated_missions: inst.get_array("associatedMissions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            mission_difficulty: inst.get_i32("missionDifficulty").unwrap_or_default(),
            locality_available: inst.get("localityAvailable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            location_mission_available: inst.get("locationMissionAvailable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            available_date_schedule: inst.get_array("availableDateSchedule")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DateTimeSchedule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DateTimeSchedule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            only_available_if_all_missions_not_available: inst.get_array("onlyAvailableIfAllMissionsNotAvailable")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            mission_reward: match inst.get("missionReward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionReward>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionReward>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            partial_reward_payout: match inst.get("partialRewardPayout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PartialContractRewards>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PartialContractRewards>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mission_result_reputation_rewards: match inst.get("missionResultReputationRewards") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationAmountListParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationAmountListParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            journal_entries_to_add_on_complete: inst.get_array("journalEntriesToAddOnComplete")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            journal_entries_to_remove_on_complete: inst.get_array("journalEntriesToRemoveOnComplete")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            initially_active: inst.get_bool("initiallyActive").unwrap_or_default(),
            notify_on_available: inst.get_bool("notifyOnAvailable").unwrap_or_default(),
            pre_show_objectives: inst.get_bool("preShowObjectives").unwrap_or_default(),
            show_as_offer: inst.get_bool("showAsOffer").unwrap_or_default(),
            mission_buy_in_amount: inst.get_i32("missionBuyInAmount").unwrap_or_default(),
            refund_buy_in_on_withdraw: inst.get_bool("refundBuyInOnWithdraw").unwrap_or_default(),
            has_complete_button: inst.get_bool("hasCompleteButton").unwrap_or_default(),
            only_owner_can_complete: inst.get_bool("onlyOwnerCanComplete").unwrap_or_default(),
            handles_abandon_request: inst.get_bool("handlesAbandonRequest").unwrap_or_default(),
            mission_module_per_player: inst.get_bool("missionModulePerPlayer").unwrap_or_default(),
            max_instances: inst.get_i32("maxInstances").unwrap_or_default(),
            max_players_per_instance: inst.get_i32("maxPlayersPerInstance").unwrap_or_default(),
            max_instances_per_player: inst.get_i32("maxInstancesPerPlayer").unwrap_or_default(),
            can_be_shared: inst.get_bool("canBeShared").unwrap_or_default(),
            once_only: inst.get_bool("onceOnly").unwrap_or_default(),
            tutorial: inst.get_bool("tutorial").unwrap_or_default(),
            mission_deadline: match inst.get("missionDeadline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionDeadline>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionDeadline>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_allied_markers: inst.get_bool("displayAlliedMarkers").unwrap_or_default(),
            available_in_prison: inst.get_bool("availableInPrison").unwrap_or_default(),
            fail_if_sent_to_prison: inst.get_bool("failIfSentToPrison").unwrap_or_default(),
            fail_if_became_criminal: inst.get_bool("failIfBecameCriminal").unwrap_or_default(),
            fail_if_leave_prison: inst.get_bool("failIfLeavePrison").unwrap_or_default(),
            completion_tags: match inst.get("completionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            apply_completion_tags_on_failed: inst.get_bool("applyCompletionTagsOnFailed").unwrap_or_default(),
            apply_completion_tags_on_abandoned: inst.get_bool("applyCompletionTagsOnAbandoned").unwrap_or_default(),
            request_only: inst.get_bool("requestOnly").unwrap_or_default(),
            respawn_time: inst.get_f32("respawnTime").unwrap_or_default(),
            respawn_time_variation: inst.get_f32("respawnTimeVariation").unwrap_or_default(),
            instance_has_life_time: inst.get_bool("instanceHasLifeTime").unwrap_or_default(),
            show_life_time_in_mobi_glas: inst.get_bool("showLifeTimeInMobiGlas").unwrap_or_default(),
            instance_life_time: inst.get_f32("instanceLifeTime").unwrap_or_default(),
            instance_life_time_variation: inst.get_f32("instanceLifeTimeVariation").unwrap_or_default(),
            can_reaccept_after_abandoning: inst.get_bool("canReacceptAfterAbandoning").unwrap_or_default(),
            abandoned_cooldown_time: inst.get_f32("abandonedCooldownTime").unwrap_or_default(),
            abandoned_cooldown_time_variation: inst.get_f32("abandonedCooldownTimeVariation").unwrap_or_default(),
            can_reaccept_after_failing: inst.get_bool("canReacceptAfterFailing").unwrap_or_default(),
            has_personal_cooldown: inst.get_bool("hasPersonalCooldown").unwrap_or_default(),
            personal_cooldown_time: inst.get_f32("personalCooldownTime").unwrap_or_default(),
            personal_cooldown_time_variation: inst.get_f32("personalCooldownTimeVariation").unwrap_or_default(),
            module_handles_own_shutdown: inst.get_bool("moduleHandlesOwnShutdown").unwrap_or_default(),
            linked_mission: inst.get("linkedMission").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_complete_perk: match inst.get("missionCompletePerk") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionCompletePerkBaseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionCompletePerkBaseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BaseMissionModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BaseMissionModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            lawful_mission: inst.get_bool("lawfulMission").unwrap_or_default(),
            mission_giver_record: inst.get("missionGiverRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            invitation_mission: inst.get("invitationMission").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_tags: inst.get_array("missionTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            mission_giver_fragment_tags: inst.get_str("missionGiverFragmentTags").map(String::from).unwrap_or_default(),
            reputation_prerequisites: match inst.get("reputationPrerequisites") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ReputationPrerequisites>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ReputationPrerequisites>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reputation_requirements: match inst.get("reputationRequirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationMissionRequirementsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationMissionRequirementsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_required_missions: inst.get_array("minRequiredMissions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MinRequiredMissions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MinRequiredMissions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            required_missions: inst.get_array("requiredMissions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            required_completed_mission_tags: inst.get_array("requiredCompletedMissionTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagSearchTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagSearchTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            required_journal_entries: inst.get_array("requiredJournalEntries")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            required_area_tags: match inst.get("requiredAreaTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            excluded_area_tags: match inst.get("excludedAreaTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            objective_tokens: inst.get_array("objectiveTokens")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ObjectiveToken>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ObjectiveToken>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            mission_flow: match inst.get("missionFlow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionFlow>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionFlow>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionGiver`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionGiver {
    /// DCB field: `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
    /// DCB field: `reputation` (Reference)
    #[serde(default)]
    pub reputation: Option<CigGuid>,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `headquarters` (Locale)
    #[serde(default)]
    pub headquarters: String,
    /// DCB field: `invitationTimeout` (Single)
    #[serde(default)]
    pub invitation_timeout: f32,
    /// DCB field: `visitTimeout` (Single)
    #[serde(default)]
    pub visit_timeout: f32,
    /// DCB field: `shortCooldown` (Single)
    #[serde(default)]
    pub short_cooldown: f32,
    /// DCB field: `mediumCooldown` (Single)
    #[serde(default)]
    pub medium_cooldown: f32,
    /// DCB field: `longCooldown` (Single)
    #[serde(default)]
    pub long_cooldown: f32,
    /// DCB field: `Allies` (Locale (array))
    #[serde(default)]
    pub allies: Vec<String>,
    /// DCB field: `Enemies` (Locale (array))
    #[serde(default)]
    pub enemies: Vec<String>,
    /// DCB field: `propertiesBB` (Class (array))
    #[serde(default)]
    pub properties_bb: Vec<Handle<SReputationContextBBPropertyParams>>,
}

impl Pooled for MissionGiver {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_giver }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_giver }
}

impl<'a> Extract<'a> for MissionGiver {
    const TYPE_NAME: &'static str = "MissionGiver";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation: inst.get("reputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            headquarters: inst.get_str("headquarters").map(String::from).unwrap_or_default(),
            invitation_timeout: inst.get_f32("invitationTimeout").unwrap_or_default(),
            visit_timeout: inst.get_f32("visitTimeout").unwrap_or_default(),
            short_cooldown: inst.get_f32("shortCooldown").unwrap_or_default(),
            medium_cooldown: inst.get_f32("mediumCooldown").unwrap_or_default(),
            long_cooldown: inst.get_f32("longCooldown").unwrap_or_default(),
            allies: inst.get_array("Allies")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            enemies: inst.get_array("Enemies")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            properties_bb: inst.get_array("propertiesBB")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionCompletePerkBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionCompletePerkBaseDef {
}

impl Pooled for MissionCompletePerkBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mission_complete_perk_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mission_complete_perk_base_def }
}

impl<'a> Extract<'a> for MissionCompletePerkBaseDef {
    const TYPE_NAME: &'static str = "MissionCompletePerkBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MiningCameraShakeConfig`
///
/// Inherits from: `CameraShakeConfig` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningCameraShakeConfig {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `offsetPosition` (Class)
    #[serde(default)]
    pub offset_position: Option<Handle<Vec3>>,
    /// DCB field: `offsetAngle` (Class)
    #[serde(default)]
    pub offset_angle: Option<Handle<Ang3>>,
    /// DCB field: `timePeriod` (Single)
    #[serde(default)]
    pub time_period: f32,
    /// DCB field: `frequencyNoiseFactor` (Single)
    #[serde(default)]
    pub frequency_noise_factor: f32,
    /// DCB field: `translationNoise` (Single)
    #[serde(default)]
    pub translation_noise: f32,
    /// DCB field: `rotationNoise` (Single)
    #[serde(default)]
    pub rotation_noise: f32,
    /// DCB field: `maxShakeWhenUnderOptimalWindow` (Single)
    #[serde(default)]
    pub max_shake_when_under_optimal_window: f32,
    /// DCB field: `shakeInOptimalWindow` (Single)
    #[serde(default)]
    pub shake_in_optimal_window: f32,
    /// DCB field: `minShakeInDangerWindow` (Single)
    #[serde(default)]
    pub min_shake_in_danger_window: f32,
    /// DCB field: `shakeChangeLerpSpeed` (Single)
    #[serde(default)]
    pub shake_change_lerp_speed: f32,
}

impl Pooled for MiningCameraShakeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mining_camera_shake_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mining_camera_shake_config }
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
    /// DCB field: `highlightColor` (Class)
    #[serde(default)]
    pub highlight_color: Option<Handle<RGBA>>,
    /// DCB field: `highlightColorAbsorbable` (Class)
    #[serde(default)]
    pub highlight_color_absorbable: Option<Handle<RGBA>>,
    /// DCB field: `highlightColorDistant` (Class)
    #[serde(default)]
    pub highlight_color_distant: Option<Handle<RGBA>>,
    /// DCB field: `highlightColorDistantScanned` (Class)
    #[serde(default)]
    pub highlight_color_distant_scanned: Option<Handle<RGBA>>,
    /// DCB field: `highlightOccludedAlpha` (Single)
    #[serde(default)]
    pub highlight_occluded_alpha: f32,
    /// DCB field: `highlightOutlineWidth` (Single)
    #[serde(default)]
    pub highlight_outline_width: f32,
    /// DCB field: `highlightDistantMineablesRange` (Single)
    #[serde(default)]
    pub highlight_distant_mineables_range: f32,
    /// DCB field: `cameraShakeConfig` (Class)
    #[serde(default)]
    pub camera_shake_config: Option<Handle<MiningCameraShakeConfig>>,
    /// DCB field: `showChildRockRadarIcon` (Boolean)
    #[serde(default)]
    pub show_child_rock_radar_icon: bool,
    /// DCB field: `scalePowerGraphMin` (Boolean)
    #[serde(default)]
    pub scale_power_graph_min: bool,
    /// DCB field: `noProgressHintTime` (Single)
    #[serde(default)]
    pub no_progress_hint_time: f32,
    /// DCB field: `noProgressHintPower` (Single)
    #[serde(default)]
    pub no_progress_hint_power: f32,
    /// DCB field: `fractureDoneFeedbackDuration` (Single)
    #[serde(default)]
    pub fracture_done_feedback_duration: f32,
    /// DCB field: `maxScanRaycastDistance` (Single)
    #[serde(default)]
    pub max_scan_raycast_distance: f32,
}

impl Pooled for MiningControllerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.mining_controller_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.mining_controller_global_params }
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

/// DCB type: `MissileLockReticleSegmentDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissileLockReticleSegmentDef {
    /// DCB field: `widthRatio` (Single)
    #[serde(default)]
    pub width_ratio: f32,
    /// DCB field: `heightRatio` (Single)
    #[serde(default)]
    pub height_ratio: f32,
    /// DCB field: `anchorX` (Single)
    #[serde(default)]
    pub anchor_x: f32,
    /// DCB field: `anchorY` (Single)
    #[serde(default)]
    pub anchor_y: f32,
    /// DCB field: `geometryPath` (String)
    #[serde(default)]
    pub geometry_path: String,
}

impl Pooled for MissileLockReticleSegmentDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.missile_lock_reticle_segment_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.missile_lock_reticle_segment_def }
}

impl<'a> Extract<'a> for MissileLockReticleSegmentDef {
    const TYPE_NAME: &'static str = "MissileLockReticleSegmentDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            width_ratio: inst.get_f32("widthRatio").unwrap_or_default(),
            height_ratio: inst.get_f32("heightRatio").unwrap_or_default(),
            anchor_x: inst.get_f32("anchorX").unwrap_or_default(),
            anchor_y: inst.get_f32("anchorY").unwrap_or_default(),
            geometry_path: inst.get_str("geometryPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissileLockReticle_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissileLockReticle_Config {
    /// DCB field: `segments` (Class (array))
    #[serde(default)]
    pub segments: Vec<Handle<MissileLockReticleSegmentDef>>,
}

impl Pooled for MissileLockReticle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mi.missile_lock_reticle_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mi.missile_lock_reticle_config }
}

impl<'a> Extract<'a> for MissileLockReticle_Config {
    const TYPE_NAME: &'static str = "MissileLockReticle_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            segments: inst.get_array("segments")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissileLockReticleSegmentDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissileLockReticleSegmentDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

