// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missionfailureconditions`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `MissionFailConditionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFailConditionParams {
    /// `triggerCondition` (Reference)
    #[serde(default)]
    pub trigger_condition: Option<CigGuid>,
    /// `warningLevel` (Int32)
    #[serde(default)]
    pub warning_level: i32,
    /// `displayText` (Locale)
    #[serde(default)]
    pub display_text: LocaleKey,
    /// `useAutomaticFailureScreen` (Boolean)
    #[serde(default)]
    pub use_automatic_failure_screen: bool,
}

impl Pooled for MissionFailConditionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionfailureconditions.mission_fail_condition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionfailureconditions.mission_fail_condition_params }
}

impl<'a> Extract<'a> for MissionFailConditionParams {
    const TYPE_NAME: &'static str = "MissionFailConditionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger_condition: inst.get("triggerCondition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            warning_level: inst.get_i32("warningLevel").unwrap_or_default(),
            display_text: inst.get_str("displayText").map(LocaleKey::from).unwrap_or_default(),
            use_automatic_failure_screen: inst.get_bool("useAutomaticFailureScreen").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionFailConditionsList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFailConditionsList {
    /// `failureConditions` (Class (array))
    #[serde(default)]
    pub failure_conditions: Vec<Handle<MissionFailConditionParams>>,
}

impl Pooled for MissionFailConditionsList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionfailureconditions.mission_fail_conditions_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionfailureconditions.mission_fail_conditions_list }
}

impl<'a> Extract<'a> for MissionFailConditionsList {
    const TYPE_NAME: &'static str = "MissionFailConditionsList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            failure_conditions: inst.get_array("failureConditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionFailConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<MissionFailConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

