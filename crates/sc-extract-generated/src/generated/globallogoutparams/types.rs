// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globallogoutparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SDCLogoutBehaviourDef`
pub struct SDCLogoutBehaviourDef {
    /// `itemRules` (Class (array))
    pub item_rules: Vec<Handle<SDCLogoutRuleEntry>>,
}

impl Pooled for SDCLogoutBehaviourDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.globallogoutparams.sdclogout_behaviour_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.globallogoutparams.sdclogout_behaviour_def
    }
}

impl<'a> Extract<'a> for SDCLogoutBehaviourDef {
    const TYPE_NAME: &'static str = "SDCLogoutBehaviourDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_rules: inst
                .get_array("itemRules")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SDCLogoutRuleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SDCLogoutRuleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SDCLogoutRuleEntry`
pub struct SDCLogoutRuleEntry {
    /// `entitySelector` (StrongPointer)
    pub entity_selector: Option<SDCLogoutEntitySelector_BasePtr>,
    /// `ruleDescription` (StrongPointer)
    pub rule_description: Option<SDCLogoutRuleDescription_BasePtr>,
    /// `action` (EnumChoice)
    pub action: ELogoutActionType,
}

impl Pooled for SDCLogoutRuleEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.globallogoutparams.sdclogout_rule_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.globallogoutparams.sdclogout_rule_entry
    }
}

impl<'a> Extract<'a> for SDCLogoutRuleEntry {
    const TYPE_NAME: &'static str = "SDCLogoutRuleEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_selector: match inst.get("entitySelector") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SDCLogoutEntitySelector_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
            rule_description: match inst.get("ruleDescription") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SDCLogoutRuleDescription_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
            action: ELogoutActionType::from_dcb_str(inst.get_str("action").unwrap_or("")),
        }
    }
}

/// DCB type: `SDCLogoutRuleDescription_Location`
/// Inherits from: `SDCLogoutRuleDescription_Base`
pub struct SDCLogoutRuleDescription_Location {
    /// `positiveLocationSelector` (Class)
    pub positive_location_selector: Option<Handle<SDCLogoutRuleDescription_LocationSelector>>,
    /// `negativeLocationSelector` (Class)
    pub negative_location_selector: Option<Handle<SDCLogoutRuleDescription_LocationSelector>>,
}

impl Pooled for SDCLogoutRuleDescription_Location {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.globallogoutparams.sdclogout_rule_description_location
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.globallogoutparams.sdclogout_rule_description_location
    }
}

impl<'a> Extract<'a> for SDCLogoutRuleDescription_Location {
    const TYPE_NAME: &'static str = "SDCLogoutRuleDescription_Location";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            positive_location_selector: match inst.get("positiveLocationSelector") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SDCLogoutRuleDescription_LocationSelector>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            negative_location_selector: match inst.get("negativeLocationSelector") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SDCLogoutRuleDescription_LocationSelector>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SDCLogoutRuleDescription_LocationSelector`
pub struct SDCLogoutRuleDescription_LocationSelector {
    /// `starmapRecords` (Reference (array))
    pub starmap_records: Vec<CigGuid>,
    /// `starmapTypes` (Reference (array))
    pub starmap_types: Vec<CigGuid>,
}

impl Pooled for SDCLogoutRuleDescription_LocationSelector {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .globallogoutparams
            .sdclogout_rule_description_location_selector
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .globallogoutparams
            .sdclogout_rule_description_location_selector
    }
}

impl<'a> Extract<'a> for SDCLogoutRuleDescription_LocationSelector {
    const TYPE_NAME: &'static str = "SDCLogoutRuleDescription_LocationSelector";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            starmap_records: inst
                .get_array("starmapRecords")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            starmap_types: inst
                .get_array("starmapTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SDCLogoutEntitySelector_ClassList`
/// Inherits from: `SDCLogoutEntitySelector_Base`
pub struct SDCLogoutEntitySelector_ClassList {
    /// `entityClasses` (Reference (array))
    pub entity_classes: Vec<CigGuid>,
}

impl Pooled for SDCLogoutEntitySelector_ClassList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .globallogoutparams
            .sdclogout_entity_selector_class_list
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .globallogoutparams
            .sdclogout_entity_selector_class_list
    }
}

impl<'a> Extract<'a> for SDCLogoutEntitySelector_ClassList {
    const TYPE_NAME: &'static str = "SDCLogoutEntitySelector_ClassList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_classes: inst
                .get_array("entityClasses")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
