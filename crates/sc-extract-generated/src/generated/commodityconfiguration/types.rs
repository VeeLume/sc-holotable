// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `commodityconfiguration`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `CommodityDamageConfiguration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityDamageConfiguration {
    /// `minimumSpeed` (Single)
    #[serde(default)]
    pub minimum_speed: f32,
    /// `speedSquaredToDamage` (Single)
    #[serde(default)]
    pub speed_squared_to_damage: f32,
    /// `defaultExplosionParams` (Class)
    #[serde(default)]
    pub default_explosion_params: Option<Handle<ExplosionParams>>,
    /// `volatilePowerRtpc` (Class)
    #[serde(default)]
    pub volatile_power_rtpc: Option<Handle<AudioRtpc>>,
    /// `volatilityRadiusFactor` (Single)
    #[serde(default)]
    pub volatility_radius_factor: f32,
    /// `volatilityDamageFactor` (Single)
    #[serde(default)]
    pub volatility_damage_factor: f32,
    /// `volatilityForceFactor` (Single)
    #[serde(default)]
    pub volatility_force_factor: f32,
    /// `volatilityCommodityDamageFactor` (Single)
    #[serde(default)]
    pub volatility_commodity_damage_factor: f32,
    /// `volatilityParticleStrengthFactor` (Single)
    #[serde(default)]
    pub volatility_particle_strength_factor: f32,
    /// `gracePeriod` (Single)
    #[serde(default)]
    pub grace_period: f32,
}

impl Pooled for CommodityDamageConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commodityconfiguration.commodity_damage_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commodityconfiguration.commodity_damage_configuration }
}

impl<'a> Extract<'a> for CommodityDamageConfiguration {
    const TYPE_NAME: &'static str = "CommodityDamageConfiguration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_speed: inst.get_f32("minimumSpeed").unwrap_or_default(),
            speed_squared_to_damage: inst.get_f32("speedSquaredToDamage").unwrap_or_default(),
            default_explosion_params: match inst.get("defaultExplosionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            volatile_power_rtpc: match inst.get("volatilePowerRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            volatility_radius_factor: inst.get_f32("volatilityRadiusFactor").unwrap_or_default(),
            volatility_damage_factor: inst.get_f32("volatilityDamageFactor").unwrap_or_default(),
            volatility_force_factor: inst.get_f32("volatilityForceFactor").unwrap_or_default(),
            volatility_commodity_damage_factor: inst.get_f32("volatilityCommodityDamageFactor").unwrap_or_default(),
            volatility_particle_strength_factor: inst.get_f32("volatilityParticleStrengthFactor").unwrap_or_default(),
            grace_period: inst.get_f32("gracePeriod").unwrap_or_default(),
        }
    }
}

