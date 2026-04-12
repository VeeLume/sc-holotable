// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `awardservice`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `AwardService_Award`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwardService_Award {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `triggerId` (String)
    #[serde(default)]
    pub trigger_id: String,
    /// `displayTitle` (Locale)
    #[serde(default)]
    pub display_title: String,
    /// `displayMessage` (Locale)
    #[serde(default)]
    pub display_message: String,
    /// `badgeId` (UInt32)
    #[serde(default)]
    pub badge_id: u32,
    /// `prerequisiteBadgeIds` (UInt32 (array))
    #[serde(default)]
    pub prerequisite_badge_ids: Vec<u32>,
    /// `pushCommLinkNotification` (Boolean)
    #[serde(default)]
    pub push_comm_link_notification: bool,
}

impl Pooled for AwardService_Award {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.awardservice.award_service_award }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.awardservice.award_service_award }
}

impl<'a> Extract<'a> for AwardService_Award {
    const TYPE_NAME: &'static str = "AwardService_Award";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            trigger_id: inst.get_str("triggerId").map(String::from).unwrap_or_default(),
            display_title: inst.get_str("displayTitle").map(String::from).unwrap_or_default(),
            display_message: inst.get_str("displayMessage").map(String::from).unwrap_or_default(),
            badge_id: inst.get_u32("badgeId").unwrap_or_default(),
            prerequisite_badge_ids: inst.get_array("prerequisiteBadgeIds")
                .map(|arr| arr.filter_map(|v| v.as_u32()).collect())
                .unwrap_or_default(),
            push_comm_link_notification: inst.get_bool("pushCommLinkNotification").unwrap_or_default(),
        }
    }
}

/// DCB type: `AwardService_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwardService_Config {
    /// `Awards` (Class)
    #[serde(default)]
    pub awards: Option<Handle<AwardService_Award>>,
    /// `Played` (Class)
    #[serde(default)]
    pub played: Option<Handle<AwardService_Award>>,
}

impl Pooled for AwardService_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.awardservice.award_service_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.awardservice.award_service_config }
}

impl<'a> Extract<'a> for AwardService_Config {
    const TYPE_NAME: &'static str = "AwardService_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            awards: match inst.get("Awards") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AwardService_Award>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AwardService_Award>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            played: match inst.get("Played") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AwardService_Award>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AwardService_Award>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

