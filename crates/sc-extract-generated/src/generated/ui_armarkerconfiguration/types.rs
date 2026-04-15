// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-armarkerconfiguration`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `GlobalMarkerConfigs`
pub struct GlobalMarkerConfigs {
    /// `missionPointMarkerConfig` (Reference)
    pub mission_point_marker_config: Option<CigGuid>,
    /// `partyMemberMarkerConfig` (Reference)
    pub party_member_marker_config: Option<CigGuid>,
    /// `landingAreaMarkerConfig` (Reference)
    pub landing_area_marker_config: Option<CigGuid>,
    /// `unattendedVehicleMarkerConfig` (Reference)
    pub unattended_vehicle_marker_config: Option<CigGuid>,
}

impl Pooled for GlobalMarkerConfigs {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_armarkerconfiguration.global_marker_configs }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_armarkerconfiguration.global_marker_configs }
}

impl<'a> Extract<'a> for GlobalMarkerConfigs {
    const TYPE_NAME: &'static str = "GlobalMarkerConfigs";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_point_marker_config: inst.get("missionPointMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            party_member_marker_config: inst.get("partyMemberMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            landing_area_marker_config: inst.get("landingAreaMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            unattended_vehicle_marker_config: inst.get("unattendedVehicleMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `Marker_AbilityNearestFace`
/// Inherits from: `Marker_AbilityBase`
pub struct Marker_AbilityNearestFace {
    /// `flipAngle` (Single)
    pub flip_angle: f32,
    /// `flipTime` (Single)
    pub flip_time: f32,
    /// `easeType` (EnumChoice)
    pub ease_type: InterpolationMode,
}

impl Pooled for Marker_AbilityNearestFace {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_armarkerconfiguration.marker_ability_nearest_face }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_armarkerconfiguration.marker_ability_nearest_face }
}

impl<'a> Extract<'a> for Marker_AbilityNearestFace {
    const TYPE_NAME: &'static str = "Marker_AbilityNearestFace";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            flip_angle: inst.get_f32("flipAngle").unwrap_or_default(),
            flip_time: inst.get_f32("flipTime").unwrap_or_default(),
            ease_type: InterpolationMode::from_dcb_str(inst.get_str("easeType").unwrap_or("")),
        }
    }
}

