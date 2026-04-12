// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `ObjectivePropertyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectivePropertyBase {
}

impl Pooled for ObjectivePropertyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ob.objective_property_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ob.objective_property_base }
}

impl<'a> Extract<'a> for ObjectivePropertyBase {
    const TYPE_NAME: &'static str = "ObjectivePropertyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ObjectiveDisplayInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveDisplayInfo {
    /// DCB field: `shortDescription` (Locale)
    #[serde(default)]
    pub short_description: String,
    /// DCB field: `longDescription` (Locale)
    #[serde(default)]
    pub long_description: String,
    /// DCB field: `objectiveMarkerLabel` (Locale)
    #[serde(default)]
    pub objective_marker_label: String,
    /// DCB field: `category` (EnumChoice)
    #[serde(default)]
    pub category: String,
    /// DCB field: `hideOnHUD` (Boolean)
    #[serde(default)]
    pub hide_on_hud: bool,
}

impl Pooled for ObjectiveDisplayInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ob.objective_display_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ob.objective_display_info }
}

impl<'a> Extract<'a> for ObjectiveDisplayInfo {
    const TYPE_NAME: &'static str = "ObjectiveDisplayInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            short_description: inst.get_str("shortDescription").map(String::from).unwrap_or_default(),
            long_description: inst.get_str("longDescription").map(String::from).unwrap_or_default(),
            objective_marker_label: inst.get_str("objectiveMarkerLabel").map(String::from).unwrap_or_default(),
            category: inst.get_str("category").map(String::from).unwrap_or_default(),
            hide_on_hud: inst.get_bool("hideOnHUD").unwrap_or_default(),
        }
    }
}

/// DCB type: `ObjectiveHandlerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveHandlerBase {
}

impl Pooled for ObjectiveHandlerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ob.objective_handler_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ob.objective_handler_base }
}

impl<'a> Extract<'a> for ObjectiveHandlerBase {
    const TYPE_NAME: &'static str = "ObjectiveHandlerBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ObjectiveRewardContributionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveRewardContributionBase {
}

impl Pooled for ObjectiveRewardContributionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ob.objective_reward_contribution_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ob.objective_reward_contribution_base }
}

impl<'a> Extract<'a> for ObjectiveRewardContributionBase {
    const TYPE_NAME: &'static str = "ObjectiveRewardContributionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ObjectiveToken`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveToken {
    /// DCB field: `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `startsActive` (Boolean)
    #[serde(default)]
    pub starts_active: bool,
    /// DCB field: `properties` (StrongPointer (array))
    #[serde(default)]
    pub properties: Vec<Handle<ObjectivePropertyBase>>,
    /// DCB field: `objectiveHandler` (StrongPointer)
    #[serde(default)]
    pub objective_handler: Option<Handle<ObjectiveHandlerBase>>,
    /// DCB field: `rewardContribution` (StrongPointer)
    #[serde(default)]
    pub reward_contribution: Option<Handle<ObjectiveRewardContributionBase>>,
    /// DCB field: `displayInfo` (Class)
    #[serde(default)]
    pub display_info: Option<Handle<ObjectiveDisplayInfo>>,
    /// DCB field: `commsNotifications` (Class (array))
    #[serde(default)]
    pub comms_notifications: Vec<Handle<CommsNotificationSelector>>,
    /// DCB field: `childMissionPhases` (Class (array))
    #[serde(default)]
    pub child_mission_phases: Vec<Handle<ChildMissionPhase>>,
}

impl Pooled for ObjectiveToken {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ob.objective_token }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ob.objective_token }
}

impl<'a> Extract<'a> for ObjectiveToken {
    const TYPE_NAME: &'static str = "ObjectiveToken";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            starts_active: inst.get_bool("startsActive").unwrap_or_default(),
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ObjectivePropertyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ObjectivePropertyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            objective_handler: match inst.get("objectiveHandler") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ObjectiveHandlerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ObjectiveHandlerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reward_contribution: match inst.get("rewardContribution") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ObjectiveRewardContributionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ObjectiveRewardContributionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_info: match inst.get("displayInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ObjectiveDisplayInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ObjectiveDisplayInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            comms_notifications: inst.get_array("commsNotifications")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommsNotificationSelector>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommsNotificationSelector>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            child_mission_phases: inst.get_array("childMissionPhases")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChildMissionPhase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChildMissionPhase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

