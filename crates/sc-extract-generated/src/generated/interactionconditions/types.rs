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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `InteractionConditionCanAffordItem`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionCanAffordItem {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
}

impl Pooled for InteractionConditionCanAffordItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.interactionconditions.interaction_condition_can_afford_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.interactionconditions.interaction_condition_can_afford_item }
}

impl<'a> Extract<'a> for InteractionConditionCanAffordItem {
    const TYPE_NAME: &'static str = "InteractionConditionCanAffordItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConditionDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InteractionConditionCanBeBodyDragged`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionCanBeBodyDragged {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
}

impl Pooled for InteractionConditionCanBeBodyDragged {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.interactionconditions.interaction_condition_can_be_body_dragged }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.interactionconditions.interaction_condition_can_be_body_dragged }
}

impl<'a> Extract<'a> for InteractionConditionCanBeBodyDragged {
    const TYPE_NAME: &'static str = "InteractionConditionCanBeBodyDragged";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConditionDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ShopInteractionData`
pub struct ShopInteractionData {
    /// `quickBuyConditionList` (Class)
    pub quick_buy_condition_list: Option<Handle<InteractionConditionList>>,
    /// `quickBuyInteractionText` (Locale)
    pub quick_buy_interaction_text: LocaleKey,
    /// `quickBuyPriceStringToken` (String)
    pub quick_buy_price_string_token: String,
    /// `moreInfoInteractionText` (Locale)
    pub more_info_interaction_text: LocaleKey,
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
                _ => None,
            },
            quick_buy_interaction_text: inst.get_str("quickBuyInteractionText").map(LocaleKey::from).unwrap_or_default(),
            quick_buy_price_string_token: inst.get_str("quickBuyPriceStringToken").map(String::from).unwrap_or_default(),
            more_info_interaction_text: inst.get_str("moreInfoInteractionText").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

