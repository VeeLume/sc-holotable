// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sglobalelectronparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SGlobalElectronParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalElectronParams {
    /// `time` (Single)
    #[serde(default)]
    pub time: f32,
    /// `damagePerCharge` (Single)
    #[serde(default)]
    pub damage_per_charge: f32,
    /// `metersPerCharge` (Single)
    #[serde(default)]
    pub meters_per_charge: f32,
    /// `damageScalePerJump` (Single)
    #[serde(default)]
    pub damage_scale_per_jump: f32,
    /// `cooldownBetweenJumps` (Single)
    #[serde(default)]
    pub cooldown_between_jumps: f32,
    /// `residualChargeMultiplier` (Single)
    #[serde(default)]
    pub residual_charge_multiplier: f32,
    /// `residualChargeInterference` (Single)
    #[serde(default)]
    pub residual_charge_interference: f32,
    /// `explosionParams` (Class)
    #[serde(default)]
    pub explosion_params: Option<Handle<ExplosionParams>>,
    /// `chargedTag` (Reference)
    #[serde(default)]
    pub charged_tag: Option<CigGuid>,
    /// `chainLightningParticleEffect` (Class)
    #[serde(default)]
    pub chain_lightning_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `chainLightningEffectDuration` (Single)
    #[serde(default)]
    pub chain_lightning_effect_duration: f32,
    /// `chainLightningJoint` (String)
    #[serde(default)]
    pub chain_lightning_joint: String,
    /// `explosionJoint` (String)
    #[serde(default)]
    pub explosion_joint: String,
    /// `residualChargeAudioStartTrigger` (Class)
    #[serde(default)]
    pub residual_charge_audio_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `residualChargeAudioStopTrigger` (Class)
    #[serde(default)]
    pub residual_charge_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `residualChargeTimeRemainingRtpc` (Class)
    #[serde(default)]
    pub residual_charge_time_remaining_rtpc: Option<Handle<AudioRtpc>>,
    /// `residualChargeDamageRtpc` (Class)
    #[serde(default)]
    pub residual_charge_damage_rtpc: Option<Handle<AudioRtpc>>,
    /// `chainLightningSourceAudioStartTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_source_audio_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `chainLightningSourceAudioStopTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_source_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `chainLightningTargetAudioStartTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_target_audio_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `chainLightningTargetAudioStopTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_target_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `chainLightningTimeRemainingRtpc` (Class)
    #[serde(default)]
    pub chain_lightning_time_remaining_rtpc: Option<Handle<AudioRtpc>>,
    /// `chainLightningParticleStrengthRtpc` (Class)
    #[serde(default)]
    pub chain_lightning_particle_strength_rtpc: Option<Handle<AudioRtpc>>,
    /// `audioBoneName` (String)
    #[serde(default)]
    pub audio_bone_name: String,
}

impl Pooled for SGlobalElectronParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalelectronparams.sglobal_electron_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalelectronparams.sglobal_electron_params }
}

impl<'a> Extract<'a> for SGlobalElectronParams {
    const TYPE_NAME: &'static str = "SGlobalElectronParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            time: inst.get_f32("time").unwrap_or_default(),
            damage_per_charge: inst.get_f32("damagePerCharge").unwrap_or_default(),
            meters_per_charge: inst.get_f32("metersPerCharge").unwrap_or_default(),
            damage_scale_per_jump: inst.get_f32("damageScalePerJump").unwrap_or_default(),
            cooldown_between_jumps: inst.get_f32("cooldownBetweenJumps").unwrap_or_default(),
            residual_charge_multiplier: inst.get_f32("residualChargeMultiplier").unwrap_or_default(),
            residual_charge_interference: inst.get_f32("residualChargeInterference").unwrap_or_default(),
            explosion_params: match inst.get("explosionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            charged_tag: inst.get("chargedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            chain_lightning_particle_effect: match inst.get("chainLightningParticleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chain_lightning_effect_duration: inst.get_f32("chainLightningEffectDuration").unwrap_or_default(),
            chain_lightning_joint: inst.get_str("chainLightningJoint").map(String::from).unwrap_or_default(),
            explosion_joint: inst.get_str("explosionJoint").map(String::from).unwrap_or_default(),
            residual_charge_audio_start_trigger: match inst.get("residualChargeAudioStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            residual_charge_audio_stop_trigger: match inst.get("residualChargeAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            residual_charge_time_remaining_rtpc: match inst.get("residualChargeTimeRemainingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            residual_charge_damage_rtpc: match inst.get("residualChargeDamageRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chain_lightning_source_audio_start_trigger: match inst.get("chainLightningSourceAudioStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chain_lightning_source_audio_stop_trigger: match inst.get("chainLightningSourceAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chain_lightning_target_audio_start_trigger: match inst.get("chainLightningTargetAudioStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chain_lightning_target_audio_stop_trigger: match inst.get("chainLightningTargetAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chain_lightning_time_remaining_rtpc: match inst.get("chainLightningTimeRemainingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chain_lightning_particle_strength_rtpc: match inst.get("chainLightningParticleStrengthRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_bone_name: inst.get_str("audioBoneName").map(String::from).unwrap_or_default(),
        }
    }
}

