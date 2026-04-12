// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `interactionconditions`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `InteractionConditionPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditionPreset {
    /// `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<InteractionConditionParams>>,
    /// `conditionToHideParams` (StrongPointer (array))
    #[serde(default)]
    pub condition_to_hide_params: Vec<Handle<InteractionConditionParams>>,
}

impl Pooled for InteractionConditionPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.interactionconditions.interaction_condition_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.interactionconditions.interaction_condition_preset }
}

impl<'a> Extract<'a> for InteractionConditionPreset {
    const TYPE_NAME: &'static str = "InteractionConditionPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InteractionConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InteractionConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            condition_to_hide_params: inst.get_array("conditionToHideParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InteractionConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InteractionConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ShopInteractionData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopInteractionData {
    /// `quickBuyConditionList` (Class)
    #[serde(default)]
    pub quick_buy_condition_list: Option<Handle<InteractionConditionList>>,
    /// `quickBuyInteractionText` (Locale)
    #[serde(default)]
    pub quick_buy_interaction_text: String,
    /// `quickBuyPriceStringToken` (String)
    #[serde(default)]
    pub quick_buy_price_string_token: String,
    /// `moreInfoInteractionText` (Locale)
    #[serde(default)]
    pub more_info_interaction_text: String,
}

impl Pooled for ShopInteractionData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.interactionconditions.shop_interaction_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.interactionconditions.shop_interaction_data }
}

impl<'a> Extract<'a> for ShopInteractionData {
    const TYPE_NAME: &'static str = "ShopInteractionData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            quick_buy_condition_list: match inst.get("quickBuyConditionList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InteractionConditionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InteractionConditionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            quick_buy_interaction_text: inst.get_str("quickBuyInteractionText").map(String::from).unwrap_or_default(),
            quick_buy_price_string_token: inst.get_str("quickBuyPriceStringToken").map(String::from).unwrap_or_default(),
            more_info_interaction_text: inst.get_str("moreInfoInteractionText").map(String::from).unwrap_or_default(),
        }
    }
}

