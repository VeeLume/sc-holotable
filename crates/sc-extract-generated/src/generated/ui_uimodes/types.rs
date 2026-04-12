// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-uimodes`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `UIModeVisibilitySettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIModeVisibilitySettings {
    /// `itemPortTrackers` (Boolean)
    #[serde(default)]
    pub item_port_trackers: bool,
    /// `grenadeTrackers` (Boolean)
    #[serde(default)]
    pub grenade_trackers: bool,
    /// `missionObjectiveTrackers` (Boolean)
    #[serde(default)]
    pub mission_objective_trackers: bool,
    /// `unattendedVehicleTrackers` (Boolean)
    #[serde(default)]
    pub unattended_vehicle_trackers: bool,
    /// `radarObjectTrackers` (Boolean)
    #[serde(default)]
    pub radar_object_trackers: bool,
}

impl Pooled for UIModeVisibilitySettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uimodes.uimode_visibility_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uimodes.uimode_visibility_settings }
}

impl<'a> Extract<'a> for UIModeVisibilitySettings {
    const TYPE_NAME: &'static str = "UIModeVisibilitySettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_port_trackers: inst.get_bool("itemPortTrackers").unwrap_or_default(),
            grenade_trackers: inst.get_bool("grenadeTrackers").unwrap_or_default(),
            mission_objective_trackers: inst.get_bool("missionObjectiveTrackers").unwrap_or_default(),
            unattended_vehicle_trackers: inst.get_bool("unattendedVehicleTrackers").unwrap_or_default(),
            radar_object_trackers: inst.get_bool("radarObjectTrackers").unwrap_or_default(),
        }
    }
}

