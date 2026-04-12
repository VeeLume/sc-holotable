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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MarkerDeclutteringCullingOrder`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerDeclutteringCullingOrder {
    /// `systemLimit` (Int32)
    #[serde(default)]
    pub system_limit: i32,
    /// `standardLimit` (Int32)
    #[serde(default)]
    pub standard_limit: i32,
    /// `revealedLimit` (Int32)
    #[serde(default)]
    pub revealed_limit: i32,
    /// `revealDuration` (Single)
    #[serde(default)]
    pub reveal_duration: f32,
    /// `referenceDistance` (Single)
    #[serde(default)]
    pub reference_distance: f32,
    /// `distanceWeight` (Single)
    #[serde(default)]
    pub distance_weight: f32,
    /// `screenDistanceWeight` (Single)
    #[serde(default)]
    pub screen_distance_weight: f32,
    /// `categories` (EnumChoice (array))
    #[serde(default)]
    pub categories: Vec<String>,
}

impl Pooled for MarkerDeclutteringCullingOrder {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_armarkerconfiguration.marker_decluttering_culling_order }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_armarkerconfiguration.marker_decluttering_culling_order }
}

impl<'a> Extract<'a> for MarkerDeclutteringCullingOrder {
    const TYPE_NAME: &'static str = "MarkerDeclutteringCullingOrder";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            system_limit: inst.get_i32("systemLimit").unwrap_or_default(),
            standard_limit: inst.get_i32("standardLimit").unwrap_or_default(),
            revealed_limit: inst.get_i32("revealedLimit").unwrap_or_default(),
            reveal_duration: inst.get_f32("revealDuration").unwrap_or_default(),
            reference_distance: inst.get_f32("referenceDistance").unwrap_or_default(),
            distance_weight: inst.get_f32("distanceWeight").unwrap_or_default(),
            screen_distance_weight: inst.get_f32("screenDistanceWeight").unwrap_or_default(),
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalMarkerConfigs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMarkerConfigs {
    /// `missionPointMarkerConfig` (Reference)
    #[serde(default)]
    pub mission_point_marker_config: Option<CigGuid>,
    /// `partyMemberMarkerConfig` (Reference)
    #[serde(default)]
    pub party_member_marker_config: Option<CigGuid>,
    /// `landingAreaMarkerConfig` (Reference)
    #[serde(default)]
    pub landing_area_marker_config: Option<CigGuid>,
    /// `unattendedVehicleMarkerConfig` (Reference)
    #[serde(default)]
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

/// DCB type: `Marker_Configuration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_Configuration {
    /// `defaultMaterial` (String)
    #[serde(default)]
    pub default_material: String,
    /// `defaultModel` (String)
    #[serde(default)]
    pub default_model: String,
    /// `offScreenModel` (String)
    #[serde(default)]
    pub off_screen_model: String,
    /// `objectiveStyleModel` (String)
    #[serde(default)]
    pub objective_style_model: String,
    /// `alwaysShowOffScreenModel` (Boolean)
    #[serde(default)]
    pub always_show_off_screen_model: bool,
    /// `ARcullingCategory` (EnumChoice)
    #[serde(default)]
    pub arculling_category: String,
    /// `useSmallIcon` (Boolean)
    #[serde(default)]
    pub use_small_icon: bool,
    /// `useStandardIcon` (Boolean)
    #[serde(default)]
    pub use_standard_icon: bool,
    /// `useARIcon` (Boolean)
    #[serde(default)]
    pub use_aricon: bool,
    /// `useModel` (Boolean)
    #[serde(default)]
    pub use_model: bool,
    /// `smallIconIndex` (Int32)
    #[serde(default)]
    pub small_icon_index: i32,
    /// `standardIconIndex` (Int32)
    #[serde(default)]
    pub standard_icon_index: i32,
    /// `smallIcon` (String)
    #[serde(default)]
    pub small_icon: String,
    /// `standardIcon` (String)
    #[serde(default)]
    pub standard_icon: String,
    /// `mapLabelDisplayType` (EnumChoice)
    #[serde(default)]
    pub map_label_display_type: String,
    /// `stackPositionAlignment` (EnumChoice)
    #[serde(default)]
    pub stack_position_alignment: String,
    /// `layoutCanvas` (Reference)
    #[serde(default)]
    pub layout_canvas: Option<CigGuid>,
    /// `abilities` (StrongPointer (array))
    #[serde(default)]
    pub abilities: Vec<Handle<Marker_AbilityBase>>,
    /// `showRules` (StrongPointer (array))
    #[serde(default)]
    pub show_rules: Vec<Handle<Marker_ShowRule>>,
    /// `mapShowRules` (Class (array))
    #[serde(default)]
    pub map_show_rules: Vec<Handle<Marker_ShowRuleMapDisplayMode>>,
    /// `mapBoxoutSectionTypes` (EnumChoice (array))
    #[serde(default)]
    pub map_boxout_section_types: Vec<String>,
    /// `markerOffset` (Class)
    #[serde(default)]
    pub marker_offset: Option<Handle<Vec3>>,
}

impl Pooled for Marker_Configuration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_armarkerconfiguration.marker_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_armarkerconfiguration.marker_configuration }
}

impl<'a> Extract<'a> for Marker_Configuration {
    const TYPE_NAME: &'static str = "Marker_Configuration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_material: inst.get_str("defaultMaterial").map(String::from).unwrap_or_default(),
            default_model: inst.get_str("defaultModel").map(String::from).unwrap_or_default(),
            off_screen_model: inst.get_str("offScreenModel").map(String::from).unwrap_or_default(),
            objective_style_model: inst.get_str("objectiveStyleModel").map(String::from).unwrap_or_default(),
            always_show_off_screen_model: inst.get_bool("alwaysShowOffScreenModel").unwrap_or_default(),
            arculling_category: inst.get_str("ARcullingCategory").map(String::from).unwrap_or_default(),
            use_small_icon: inst.get_bool("useSmallIcon").unwrap_or_default(),
            use_standard_icon: inst.get_bool("useStandardIcon").unwrap_or_default(),
            use_aricon: inst.get_bool("useARIcon").unwrap_or_default(),
            use_model: inst.get_bool("useModel").unwrap_or_default(),
            small_icon_index: inst.get_i32("smallIconIndex").unwrap_or_default(),
            standard_icon_index: inst.get_i32("standardIconIndex").unwrap_or_default(),
            small_icon: inst.get_str("smallIcon").map(String::from).unwrap_or_default(),
            standard_icon: inst.get_str("standardIcon").map(String::from).unwrap_or_default(),
            map_label_display_type: inst.get_str("mapLabelDisplayType").map(String::from).unwrap_or_default(),
            stack_position_alignment: inst.get_str("stackPositionAlignment").map(String::from).unwrap_or_default(),
            layout_canvas: inst.get("layoutCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            abilities: inst.get_array("abilities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Marker_AbilityBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Marker_AbilityBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            show_rules: inst.get_array("showRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Marker_ShowRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Marker_ShowRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            map_show_rules: inst.get_array("mapShowRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Marker_ShowRuleMapDisplayMode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Marker_ShowRuleMapDisplayMode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            map_boxout_section_types: inst.get_array("mapBoxoutSectionTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            marker_offset: match inst.get("markerOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Marker_ShowRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_ShowRule {
}

impl Pooled for Marker_ShowRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_armarkerconfiguration.marker_show_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_armarkerconfiguration.marker_show_rule }
}

impl<'a> Extract<'a> for Marker_ShowRule {
    const TYPE_NAME: &'static str = "Marker_ShowRule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Marker_ShowRuleMapDisplayMode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_ShowRuleMapDisplayMode {
    /// `displayModeMap` (EnumChoice)
    #[serde(default)]
    pub display_mode_map: String,
}

impl Pooled for Marker_ShowRuleMapDisplayMode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_armarkerconfiguration.marker_show_rule_map_display_mode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_armarkerconfiguration.marker_show_rule_map_display_mode }
}

impl<'a> Extract<'a> for Marker_ShowRuleMapDisplayMode {
    const TYPE_NAME: &'static str = "Marker_ShowRuleMapDisplayMode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_mode_map: inst.get_str("displayModeMap").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `Marker_AbilityBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_AbilityBase {
}

impl Pooled for Marker_AbilityBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_armarkerconfiguration.marker_ability_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_armarkerconfiguration.marker_ability_base }
}

impl<'a> Extract<'a> for Marker_AbilityBase {
    const TYPE_NAME: &'static str = "Marker_AbilityBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

