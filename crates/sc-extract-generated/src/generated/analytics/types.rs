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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SAnalyticsEventDatabase`
pub struct SAnalyticsEventDatabase {
    /// `analyticsEvents` (Reference (array))
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

