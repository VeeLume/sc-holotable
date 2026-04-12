// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `aimotive`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `AIMotiveActionDetails`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveActionDetails {
    /// `detailsId` (String)
    #[serde(default)]
    pub details_id: String,
}

impl Pooled for AIMotiveActionDetails {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aimotive.aimotive_action_details }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aimotive.aimotive_action_details }
}

impl<'a> Extract<'a> for AIMotiveActionDetails {
    const TYPE_NAME: &'static str = "AIMotiveActionDetails";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            details_id: inst.get_str("detailsId").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotiveAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveAction {
    /// `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// `actionName` (String)
    #[serde(default)]
    pub action_name: String,
    /// `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<AIMotiveCondition>>,
    /// `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// `actionDetails` (StrongPointer (array))
    #[serde(default)]
    pub action_details: Vec<Handle<AIMotiveActionDetails>>,
}

impl Pooled for AIMotiveAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aimotive.aimotive_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aimotive.aimotive_action }
}

impl<'a> Extract<'a> for AIMotiveAction {
    const TYPE_NAME: &'static str = "AIMotiveAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            action_name: inst.get_str("actionName").map(String::from).unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotiveCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotiveCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            action_details: inst.get_array("actionDetails")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotiveActionDetails>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotiveActionDetails>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotiveCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveCondition {
    /// `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// `interrupt` (Boolean)
    #[serde(default)]
    pub interrupt: bool,
    /// `minSatisfactionDuration` (Single)
    #[serde(default)]
    pub min_satisfaction_duration: f32,
}

impl Pooled for AIMotiveCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aimotive.aimotive_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aimotive.aimotive_condition }
}

impl<'a> Extract<'a> for AIMotiveCondition {
    const TYPE_NAME: &'static str = "AIMotiveCondition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            interrupt: inst.get_bool("interrupt").unwrap_or_default(),
            min_satisfaction_duration: inst.get_f32("minSatisfactionDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotive`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotive {
    /// `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// `actionDefs` (Class (array))
    #[serde(default)]
    pub action_defs: Vec<Handle<AIMotiveAction>>,
}

impl Pooled for AIMotive {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aimotive.aimotive }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aimotive.aimotive }
}

impl<'a> Extract<'a> for AIMotive {
    const TYPE_NAME: &'static str = "AIMotive";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            action_defs: inst.get_array("actionDefs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotiveAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotiveAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotiveList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveList {
    /// `motiveList` (StrongPointer (array))
    #[serde(default)]
    pub motive_list: Vec<Handle<AIMotive>>,
}

impl Pooled for AIMotiveList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aimotive.aimotive_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aimotive.aimotive_list }
}

impl<'a> Extract<'a> for AIMotiveList {
    const TYPE_NAME: &'static str = "AIMotiveList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            motive_list: inst.get_array("motiveList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotive>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotive>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

