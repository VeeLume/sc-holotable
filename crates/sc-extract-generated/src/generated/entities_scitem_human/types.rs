// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-human`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `CommodityCrateComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct CommodityCrateComponentParams {}

impl Pooled for CommodityCrateComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.commodity_crate_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.commodity_crate_component_params
    }
}

impl<'a> Extract<'a> for CommodityCrateComponentParams {
    const TYPE_NAME: &'static str = "CommodityCrateComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `MiningShopProviderEntityComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct MiningShopProviderEntityComponentParams {}

impl Pooled for MiningShopProviderEntityComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_human
            .mining_shop_provider_entity_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_human
            .mining_shop_provider_entity_component_params
    }
}

impl<'a> Extract<'a> for MiningShopProviderEntityComponentParams {
    const TYPE_NAME: &'static str = "MiningShopProviderEntityComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SDespawnRule_OnFallBelow`
/// Inherits from: `SDespawnRule`
pub struct SDespawnRule_OnFallBelow {
    /// `ruleDelaySeconds` (Single)
    pub rule_delay_seconds: f32,
    /// `distance` (Single)
    pub distance: f32,
}

impl Pooled for SDespawnRule_OnFallBelow {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.sdespawn_rule_on_fall_below
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.sdespawn_rule_on_fall_below
    }
}

impl<'a> Extract<'a> for SDespawnRule_OnFallBelow {
    const TYPE_NAME: &'static str = "SDespawnRule_OnFallBelow";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rule_delay_seconds: inst.get_f32("ruleDelaySeconds").unwrap_or_default(),
            distance: inst.get_f32("distance").unwrap_or_default(),
        }
    }
}

/// DCB type: `ClassEntityFilter`
/// Inherits from: `EntityFilter`
pub struct ClassEntityFilter {
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
}

impl Pooled for ClassEntityFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.class_entity_filter
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.class_entity_filter
    }
}

impl<'a> Extract<'a> for ClassEntityFilter {
    const TYPE_NAME: &'static str = "ClassEntityFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `UserVariableCheckIntLess`
/// Inherits from: `UserVariableCheck`
pub struct UserVariableCheckIntLess {
    /// `variableName` (String)
    pub variable_name: String,
    /// `valueToCheck` (Int32)
    pub value_to_check: i32,
}

impl Pooled for UserVariableCheckIntLess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.user_variable_check_int_less
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.user_variable_check_int_less
    }
}

impl<'a> Extract<'a> for UserVariableCheckIntLess {
    const TYPE_NAME: &'static str = "UserVariableCheckIntLess";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            value_to_check: inst.get_i32("valueToCheck").unwrap_or_default(),
        }
    }
}
