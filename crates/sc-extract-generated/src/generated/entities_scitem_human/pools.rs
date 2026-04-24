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

/// Pool storage for the `entities-scitem-human` feature.
#[derive(Default)]
pub struct EntitiesScitemHumanPools {
    pub commodity_crate_component_params: Vec<Option<CommodityCrateComponentParams>>,
    pub mining_shop_provider_entity_component_params:
        Vec<Option<MiningShopProviderEntityComponentParams>>,
    pub sdespawn_rule_on_fall_below: Vec<Option<SDespawnRule_OnFallBelow>>,
    pub delivery_locker_component_params: Vec<Option<DeliveryLockerComponentParams>>,
    pub class_entity_filter: Vec<Option<ClassEntityFilter>>,
    pub gameplay_trigger_condition_not: Vec<Option<GameplayTriggerConditionNOT>>,
    pub user_variable_check_int_less: Vec<Option<UserVariableCheckIntLess>>,
    pub user_variable_subtract_int_value: Vec<Option<UserVariableSubtractIntValue>>,
}
