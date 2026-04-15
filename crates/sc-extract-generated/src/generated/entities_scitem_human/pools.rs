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

/// Pool storage for the `entities-scitem-human` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemHumanPools {
    #[serde(default)]
    pub commodity_crate_component_params: Vec<Option<CommodityCrateComponentParams>>,
    #[serde(default)]
    pub mining_shop_provider_entity_component_params: Vec<Option<MiningShopProviderEntityComponentParams>>,
    #[serde(default)]
    pub sdespawn_rule_on_fall_below: Vec<Option<SDespawnRule_OnFallBelow>>,
    #[serde(default)]
    pub delivery_locker_component_params: Vec<Option<DeliveryLockerComponentParams>>,
    #[serde(default)]
    pub class_entity_filter: Vec<Option<ClassEntityFilter>>,
    #[serde(default)]
    pub gameplay_trigger_condition_not: Vec<Option<GameplayTriggerConditionNOT>>,
    #[serde(default)]
    pub user_variable_check_int_less: Vec<Option<UserVariableCheckIntLess>>,
    #[serde(default)]
    pub user_variable_subtract_int_value: Vec<Option<UserVariableSubtractIntValue>>,
}
