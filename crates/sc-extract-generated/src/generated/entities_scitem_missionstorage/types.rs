// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-missionstorage`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SDespawnRule_OnPartsDetached`
/// Inherits from: `SDespawnRule`
pub struct SDespawnRule_OnPartsDetached {
    /// `ruleDelaySeconds` (Single)
    pub rule_delay_seconds: f32,
}

impl Pooled for SDespawnRule_OnPartsDetached {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_missionstorage
            .sdespawn_rule_on_parts_detached
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_missionstorage
            .sdespawn_rule_on_parts_detached
    }
}

impl<'a> Extract<'a> for SDespawnRule_OnPartsDetached {
    const TYPE_NAME: &'static str = "SDespawnRule_OnPartsDetached";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rule_delay_seconds: inst.get_f32("ruleDelaySeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `InteractionConditionDeliveryMissionItem`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionDeliveryMissionItem {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
}

impl Pooled for InteractionConditionDeliveryMissionItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_missionstorage
            .interaction_condition_delivery_mission_item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_missionstorage
            .interaction_condition_delivery_mission_item
    }
}

impl<'a> Extract<'a> for InteractionConditionDeliveryMissionItem {
    const TYPE_NAME: &'static str = "InteractionConditionDeliveryMissionItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ConditionDisplayParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `DeliveryItemPortComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct DeliveryItemPortComponentParams {}

impl Pooled for DeliveryItemPortComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_missionstorage
            .delivery_item_port_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_missionstorage
            .delivery_item_port_component_params
    }
}

impl<'a> Extract<'a> for DeliveryItemPortComponentParams {
    const TYPE_NAME: &'static str = "DeliveryItemPortComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `GameplayTriggerConditionCheckStateOther`
/// Inherits from: `GameplayTriggerCondition`
pub struct GameplayTriggerConditionCheckStateOther {
    /// `stateTypeName` (String)
    pub state_type_name: String,
    /// `stateName` (String)
    pub state_name: String,
}

impl Pooled for GameplayTriggerConditionCheckStateOther {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_missionstorage
            .gameplay_trigger_condition_check_state_other
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_missionstorage
            .gameplay_trigger_condition_check_state_other
    }
}

impl<'a> Extract<'a> for GameplayTriggerConditionCheckStateOther {
    const TYPE_NAME: &'static str = "GameplayTriggerConditionCheckStateOther";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_type_name: inst
                .get_str("stateTypeName")
                .map(String::from)
                .unwrap_or_default(),
            state_name: inst
                .get_str("stateName")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UserVariableCheckBoolEqual`
/// Inherits from: `UserVariableCheck`
pub struct UserVariableCheckBoolEqual {
    /// `variableName` (String)
    pub variable_name: String,
    /// `valueToCheck` (Boolean)
    pub value_to_check: bool,
}

impl Pooled for UserVariableCheckBoolEqual {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_missionstorage
            .user_variable_check_bool_equal
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_missionstorage
            .user_variable_check_bool_equal
    }
}

impl<'a> Extract<'a> for UserVariableCheckBoolEqual {
    const TYPE_NAME: &'static str = "UserVariableCheckBoolEqual";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            value_to_check: inst.get_bool("valueToCheck").unwrap_or_default(),
        }
    }
}
