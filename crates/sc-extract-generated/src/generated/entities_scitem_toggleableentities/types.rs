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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ShakeComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ShakeComponentParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `unifiedShakeParams` (Reference)
    pub unified_shake_params: Option<CigGuid>,
    /// `distanceIntensityCurve` (Reference)
    pub distance_intensity_curve: Option<CigGuid>,
}

impl Pooled for ShakeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_toggleableentities
            .shake_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_toggleableentities
            .shake_component_params
    }
}

impl<'a> Extract<'a> for ShakeComponentParams {
    const TYPE_NAME: &'static str = "ShakeComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            unified_shake_params: inst
                .get("unifiedShakeParams")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            distance_intensity_curve: inst
                .get("distanceIntensityCurve")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SetLightningEnabledStateGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
pub struct SetLightningEnabledStateGameplayTrigger {
    /// `enabled` (Boolean)
    pub enabled: bool,
}

impl Pooled for SetLightningEnabledStateGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_toggleableentities
            .set_lightning_enabled_state_gameplay_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_toggleableentities
            .set_lightning_enabled_state_gameplay_trigger
    }
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
pub struct SSetParticleEnabledStateGameplayTrigger {
    /// `Enable` (Boolean)
    pub enable: bool,
}

impl Pooled for SSetParticleEnabledStateGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_toggleableentities
            .sset_particle_enabled_state_gameplay_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_toggleableentities
            .sset_particle_enabled_state_gameplay_trigger
    }
}

impl<'a> Extract<'a> for SSetParticleEnabledStateGameplayTrigger {
    const TYPE_NAME: &'static str = "SSetParticleEnabledStateGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("Enable").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActivateScreenShakeAreaOneShotGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
pub struct ActivateScreenShakeAreaOneShotGameplayTrigger {}

impl Pooled for ActivateScreenShakeAreaOneShotGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_toggleableentities
            .activate_screen_shake_area_one_shot_gameplay_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_toggleableentities
            .activate_screen_shake_area_one_shot_gameplay_trigger
    }
}

impl<'a> Extract<'a> for ActivateScreenShakeAreaOneShotGameplayTrigger {
    const TYPE_NAME: &'static str = "ActivateScreenShakeAreaOneShotGameplayTrigger";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
