// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `targetselector`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `STargetingMethodBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetingMethodBase {
}

impl Pooled for STargetingMethodBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.stargeting_method_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.stargeting_method_base }
}

impl<'a> Extract<'a> for STargetingMethodBase {
    const TYPE_NAME: &'static str = "STargetingMethodBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `STargetingMethodRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetingMethodRecord {
    /// `targetingMethod` (StrongPointer)
    #[serde(default)]
    pub targeting_method: Option<Handle<STargetingMethodBase>>,
}

impl Pooled for STargetingMethodRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.stargeting_method_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.stargeting_method_record }
}

impl<'a> Extract<'a> for STargetingMethodRecord {
    const TYPE_NAME: &'static str = "STargetingMethodRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method: match inst.get("targetingMethod") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetingMethodBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STargetingMethodBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SPrecisionTargetingItemName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPrecisionTargetingItemName {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
}

impl Pooled for SPrecisionTargetingItemName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.sprecision_targeting_item_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.sprecision_targeting_item_name }
}

impl<'a> Extract<'a> for SPrecisionTargetingItemName {
    const TYPE_NAME: &'static str = "SPrecisionTargetingItemName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `STargetableItemType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetableItemType {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `subItemTypes` (EnumChoice (array))
    #[serde(default)]
    pub sub_item_types: Vec<String>,
}

impl Pooled for STargetableItemType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.stargetable_item_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.stargetable_item_type }
}

impl<'a> Extract<'a> for STargetableItemType {
    const TYPE_NAME: &'static str = "STargetableItemType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            sub_item_types: inst.get_array("subItemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `STargetableItemTypesRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetableItemTypesRecord {
    /// `targetableItemTypes` (Class (array))
    #[serde(default)]
    pub targetable_item_types: Vec<Handle<STargetableItemType>>,
    /// `precisionTargetingNames` (Class (array))
    #[serde(default)]
    pub precision_targeting_names: Vec<Handle<SPrecisionTargetingItemName>>,
}

impl Pooled for STargetableItemTypesRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.stargetable_item_types_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.stargetable_item_types_record }
}

impl<'a> Extract<'a> for STargetableItemTypesRecord {
    const TYPE_NAME: &'static str = "STargetableItemTypesRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            targetable_item_types: inst.get_array("targetableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<STargetableItemType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<STargetableItemType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            precision_targeting_names: inst.get_array("precisionTargetingNames")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SPrecisionTargetingItemName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SPrecisionTargetingItemName>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCombatTargeting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCombatTargeting {
    /// `targetingMethodRecord` (Reference)
    #[serde(default)]
    pub targeting_method_record: Option<CigGuid>,
}

impl Pooled for SCombatTargeting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.scombat_targeting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.scombat_targeting }
}

impl<'a> Extract<'a> for SCombatTargeting {
    const TYPE_NAME: &'static str = "SCombatTargeting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method_record: inst.get("targetingMethodRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SScanTargeting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScanTargeting {
    /// `targetingMethodRecord` (Reference)
    #[serde(default)]
    pub targeting_method_record: Option<CigGuid>,
}

impl Pooled for SScanTargeting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.sscan_targeting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.sscan_targeting }
}

impl<'a> Extract<'a> for SScanTargeting {
    const TYPE_NAME: &'static str = "SScanTargeting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method_record: inst.get("targetingMethodRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMiningTargeting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMiningTargeting {
    /// `targetingMethodRecord` (Reference)
    #[serde(default)]
    pub targeting_method_record: Option<CigGuid>,
}

impl Pooled for SMiningTargeting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.smining_targeting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.smining_targeting }
}

impl<'a> Extract<'a> for SMiningTargeting {
    const TYPE_NAME: &'static str = "SMiningTargeting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method_record: inst.get("targetingMethodRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `STargetSelectorGlobalTargetingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetSelectorGlobalTargetingParams {
    /// `combatTargetingMethodRecord` (Class)
    #[serde(default)]
    pub combat_targeting_method_record: Option<Handle<SCombatTargeting>>,
    /// `scanningTargetingMethodRecord` (Class)
    #[serde(default)]
    pub scanning_targeting_method_record: Option<Handle<SScanTargeting>>,
    /// `miningTargetingMethodRecord` (Class)
    #[serde(default)]
    pub mining_targeting_method_record: Option<Handle<SMiningTargeting>>,
}

impl Pooled for STargetSelectorGlobalTargetingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.targetselector.starget_selector_global_targeting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.targetselector.starget_selector_global_targeting_params }
}

impl<'a> Extract<'a> for STargetSelectorGlobalTargetingParams {
    const TYPE_NAME: &'static str = "STargetSelectorGlobalTargetingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            combat_targeting_method_record: match inst.get("combatTargetingMethodRecord") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCombatTargeting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCombatTargeting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scanning_targeting_method_record: match inst.get("scanningTargetingMethodRecord") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SScanTargeting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SScanTargeting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mining_targeting_method_record: match inst.get("miningTargetingMethodRecord") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMiningTargeting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMiningTargeting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

