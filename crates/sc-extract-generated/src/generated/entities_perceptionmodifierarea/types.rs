// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-perceptionmodifierarea`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `LightStatusMultiplier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightStatusMultiplier {
    /// `status` (EnumChoice)
    #[serde(default)]
    pub status: ELightState,
    /// `multiplier` (Single)
    #[serde(default)]
    pub multiplier: f32,
}

impl Pooled for LightStatusMultiplier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_perceptionmodifierarea.light_status_multiplier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_perceptionmodifierarea.light_status_multiplier }
}

impl<'a> Extract<'a> for LightStatusMultiplier {
    const TYPE_NAME: &'static str = "LightStatusMultiplier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            status: ELightState::from_dcb_str(inst.get_str("status").unwrap_or("")),
            multiplier: inst.get_f32("multiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActionAreaAudioNoiseExtension`
/// Inherits from: `ActionAreaExtensionType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionAreaAudioNoiseExtension {
    /// `defaultAudioNoiseLevel` (Single)
    #[serde(default)]
    pub default_audio_noise_level: f32,
}

impl Pooled for ActionAreaAudioNoiseExtension {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_perceptionmodifierarea.action_area_audio_noise_extension }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_perceptionmodifierarea.action_area_audio_noise_extension }
}

impl<'a> Extract<'a> for ActionAreaAudioNoiseExtension {
    const TYPE_NAME: &'static str = "ActionAreaAudioNoiseExtension";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_audio_noise_level: inst.get_f32("defaultAudioNoiseLevel").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActionAreaLightExtension`
/// Inherits from: `ActionAreaExtensionType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionAreaLightExtension {
    /// `lightStatusMultipliers` (StrongPointer (array))
    #[serde(default)]
    pub light_status_multipliers: Vec<Handle<LightStatusMultiplier>>,
}

impl Pooled for ActionAreaLightExtension {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_perceptionmodifierarea.action_area_light_extension }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_perceptionmodifierarea.action_area_light_extension }
}

impl<'a> Extract<'a> for ActionAreaLightExtension {
    const TYPE_NAME: &'static str = "ActionAreaLightExtension";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_status_multipliers: inst.get_array("lightStatusMultipliers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LightStatusMultiplier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

