// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-toggleableentities`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SetLightningEnabledStateGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLightningEnabledStateGameplayTrigger {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for SetLightningEnabledStateGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_toggleableentities.set_lightning_enabled_state_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_toggleableentities.set_lightning_enabled_state_gameplay_trigger }
}

impl<'a> Extract<'a> for SetLightningEnabledStateGameplayTrigger {
    const TYPE_NAME: &'static str = "SetLightningEnabledStateGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSetParticleEnabledStateGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSetParticleEnabledStateGameplayTrigger {
    /// `Enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
}

impl Pooled for SSetParticleEnabledStateGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_toggleableentities.sset_particle_enabled_state_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_toggleableentities.sset_particle_enabled_state_gameplay_trigger }
}

impl<'a> Extract<'a> for SSetParticleEnabledStateGameplayTrigger {
    const TYPE_NAME: &'static str = "SSetParticleEnabledStateGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("Enable").unwrap_or_default(),
        }
    }
}

