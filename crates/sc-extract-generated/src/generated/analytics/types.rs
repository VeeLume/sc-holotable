// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `analytics`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SAnalyticsEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAnalyticsEvent {
    /// `eventName` (String)
    #[serde(default)]
    pub event_name: String,
    /// `includePlayerData` (Boolean)
    #[serde(default)]
    pub include_player_data: bool,
    /// `includeLocationData` (Boolean)
    #[serde(default)]
    pub include_location_data: bool,
    /// `thisEntityNameField` (String)
    #[serde(default)]
    pub this_entity_name_field: String,
}

impl Pooled for SAnalyticsEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.analytics.sanalytics_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.analytics.sanalytics_event }
}

impl<'a> Extract<'a> for SAnalyticsEvent {
    const TYPE_NAME: &'static str = "SAnalyticsEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            event_name: inst.get_str("eventName").map(String::from).unwrap_or_default(),
            include_player_data: inst.get_bool("includePlayerData").unwrap_or_default(),
            include_location_data: inst.get_bool("includeLocationData").unwrap_or_default(),
            this_entity_name_field: inst.get_str("thisEntityNameField").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SAnalyticsEventDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAnalyticsEventDatabase {
    /// `analyticsEvents` (Reference (array))
    #[serde(default)]
    pub analytics_events: Vec<CigGuid>,
}

impl Pooled for SAnalyticsEventDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.analytics.sanalytics_event_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.analytics.sanalytics_event_database }
}

impl<'a> Extract<'a> for SAnalyticsEventDatabase {
    const TYPE_NAME: &'static str = "SAnalyticsEventDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            analytics_events: inst.get_array("analyticsEvents")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

