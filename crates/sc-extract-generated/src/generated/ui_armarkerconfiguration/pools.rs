// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-armarkerconfiguration` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiArmarkerconfigurationPools {
    #[serde(default)]
    pub marker_decluttering_culling_order: Vec<Option<MarkerDeclutteringCullingOrder>>,
    #[serde(default)]
    pub global_marker_configs: Vec<Option<GlobalMarkerConfigs>>,
    #[serde(default)]
    pub marker_configuration: Vec<Option<Marker_Configuration>>,
    #[serde(default)]
    pub marker_show_rule: Vec<Option<Marker_ShowRule>>,
    #[serde(default)]
    pub marker_show_rule_map_display_mode: Vec<Option<Marker_ShowRuleMapDisplayMode>>,
    #[serde(default)]
    pub marker_ability_base: Vec<Option<Marker_AbilityBase>>,
}
