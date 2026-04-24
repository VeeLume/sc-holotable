// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-dockingslotvisibility`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `DockingSlotVisibilityTagSet`
pub struct DockingSlotVisibilityTagSet {
    /// `tags` (String (array))
    pub tags: Vec<String>,
}

impl Pooled for DockingSlotVisibilityTagSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_dockingslotvisibility.docking_slot_visibility_tag_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_dockingslotvisibility.docking_slot_visibility_tag_set }
}

impl<'a> Extract<'a> for DockingSlotVisibilityTagSet {
    const TYPE_NAME: &'static str = "DockingSlotVisibilityTagSet";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSlotVisibilityRule`
pub struct DockingSlotVisibilityRule {
    /// `modes` (EnumChoice (array))
    pub modes: Vec<UIBlockingMode>,
    /// `tagSets` (Class (array))
    pub tag_sets: Vec<Handle<DockingSlotVisibilityTagSet>>,
}

impl Pooled for DockingSlotVisibilityRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_dockingslotvisibility.docking_slot_visibility_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_dockingslotvisibility.docking_slot_visibility_rule }
}

impl<'a> Extract<'a> for DockingSlotVisibilityRule {
    const TYPE_NAME: &'static str = "DockingSlotVisibilityRule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            modes: inst.get_array("modes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(UIBlockingMode::from_dcb_str)).collect())
                .unwrap_or_default(),
            tag_sets: inst.get_array("tagSets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DockingSlotVisibilityTagSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<DockingSlotVisibilityTagSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSlotVisibility`
pub struct DockingSlotVisibility {
    /// `rules` (Class (array))
    pub rules: Vec<Handle<DockingSlotVisibilityRule>>,
}

impl Pooled for DockingSlotVisibility {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_dockingslotvisibility.docking_slot_visibility }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_dockingslotvisibility.docking_slot_visibility }
}

impl<'a> Extract<'a> for DockingSlotVisibility {
    const TYPE_NAME: &'static str = "DockingSlotVisibility";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DockingSlotVisibilityRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<DockingSlotVisibilityRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

