// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-objectdatabankentrymarkerconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SCObjectDataBankEntryMarkerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCObjectDataBankEntryMarkerConfig {
    /// `managedLandingZoneMinimumDist` (Single)
    #[serde(default)]
    pub managed_landing_zone_minimum_dist: f32,
    /// `managedLandingZoneMaximumDist` (Single)
    #[serde(default)]
    pub managed_landing_zone_maximum_dist: f32,
    /// `unmanagedLandingZoneMinimumDist` (Single)
    #[serde(default)]
    pub unmanaged_landing_zone_minimum_dist: f32,
    /// `unmanagedLandingZoneMaximumDist` (Single)
    #[serde(default)]
    pub unmanaged_landing_zone_maximum_dist: f32,
}

impl Pooled for SCObjectDataBankEntryMarkerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_objectdatabankentrymarkerconfig.scobject_data_bank_entry_marker_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_objectdatabankentrymarkerconfig.scobject_data_bank_entry_marker_config }
}

impl<'a> Extract<'a> for SCObjectDataBankEntryMarkerConfig {
    const TYPE_NAME: &'static str = "SCObjectDataBankEntryMarkerConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            managed_landing_zone_minimum_dist: inst.get_f32("managedLandingZoneMinimumDist").unwrap_or_default(),
            managed_landing_zone_maximum_dist: inst.get_f32("managedLandingZoneMaximumDist").unwrap_or_default(),
            unmanaged_landing_zone_minimum_dist: inst.get_f32("unmanagedLandingZoneMinimumDist").unwrap_or_default(),
            unmanaged_landing_zone_maximum_dist: inst.get_f32("unmanagedLandingZoneMaximumDist").unwrap_or_default(),
        }
    }
}

