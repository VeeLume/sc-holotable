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

/// DCB type: `SStatusTriggerThresholdLevelModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SStatusTriggerThresholdLevelModifier {
    /// DCB field: `minThresholdModifier` (Single)
    #[serde(default)]
    pub min_threshold_modifier: f32,
    /// DCB field: `interpMinThresholdModifier` (Single)
    #[serde(default)]
    pub interp_min_threshold_modifier: f32,
    /// DCB field: `interpMaxThresholdModifier` (Single)
    #[serde(default)]
    pub interp_max_threshold_modifier: f32,
    /// DCB field: `guranteedThresholdModifier` (Single)
    #[serde(default)]
    pub guranteed_threshold_modifier: f32,
    /// DCB field: `maxThresholdModifier` (Single)
    #[serde(default)]
    pub max_threshold_modifier: f32,
}

impl Pooled for SStatusTriggerThresholdLevelModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sstatus_trigger_threshold_level_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sstatus_trigger_threshold_level_modifier }
}

impl<'a> Extract<'a> for SStatusTriggerThresholdLevelModifier {
    const TYPE_NAME: &'static str = "SStatusTriggerThresholdLevelModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_threshold_modifier: inst.get_f32("minThresholdModifier").unwrap_or_default(),
            interp_min_threshold_modifier: inst.get_f32("interpMinThresholdModifier").unwrap_or_default(),
            interp_max_threshold_modifier: inst.get_f32("interpMaxThresholdModifier").unwrap_or_default(),
            guranteed_threshold_modifier: inst.get_f32("guranteedThresholdModifier").unwrap_or_default(),
            max_threshold_modifier: inst.get_f32("maxThresholdModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SStatusFortitudeLevelModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SStatusFortitudeLevelModifier {
    /// DCB field: `triggerThresholdLevelModifier` (Class (array))
    #[serde(default)]
    pub trigger_threshold_level_modifier: Vec<Handle<SStatusTriggerThresholdLevelModifier>>,
}

impl Pooled for SStatusFortitudeLevelModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sstatus_fortitude_level_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sstatus_fortitude_level_modifier }
}

impl<'a> Extract<'a> for SStatusFortitudeLevelModifier {
    const TYPE_NAME: &'static str = "SStatusFortitudeLevelModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_threshold_level_modifier: inst.get_array("triggerThresholdLevelModifier")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SStatusTriggerThresholdLevelModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SStatusTriggerThresholdLevelModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SScanTargeting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScanTargeting {
    /// DCB field: `targetingMethodRecord` (Reference)
    #[serde(default)]
    pub targeting_method_record: Option<CigGuid>,
}

impl Pooled for SScanTargeting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscan_targeting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscan_targeting }
}

impl<'a> Extract<'a> for SScanTargeting {
    const TYPE_NAME: &'static str = "SScanTargeting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method_record: inst.get("targetingMethodRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SScoreEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScoreEvent {
    /// DCB field: `points` (Int32)
    #[serde(default)]
    pub points: i32,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for SScoreEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscore_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscore_event }
}

impl<'a> Extract<'a> for SScoreEvent {
    const TYPE_NAME: &'static str = "SScoreEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            points: inst.get_i32("points").unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSoftbodyGeometryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSoftbodyGeometryParams {
    /// DCB field: `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `SimulationParams` (Class)
    #[serde(default)]
    pub simulation_params: Option<Handle<SEntitySoftExPhysicsControllerParams>>,
}

impl Pooled for SSoftbodyGeometryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssoftbody_geometry_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssoftbody_geometry_params }
}

impl<'a> Extract<'a> for SSoftbodyGeometryParams {
    const TYPE_NAME: &'static str = "SSoftbodyGeometryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            simulation_params: match inst.get("SimulationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntitySoftExPhysicsControllerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntitySoftExPhysicsControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSalvageRepairHighlightColorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairHighlightColorParams {
    /// DCB field: `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGB>>,
    /// DCB field: `hullThreshold` (Single)
    #[serde(default)]
    pub hull_threshold: f32,
}

impl Pooled for SSalvageRepairHighlightColorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_highlight_color_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_highlight_color_params }
}

impl<'a> Extract<'a> for SSalvageRepairHighlightColorParams {
    const TYPE_NAME: &'static str = "SSalvageRepairHighlightColorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hull_threshold: inst.get_f32("hullThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairHighlightOutlineValues`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairHighlightOutlineValues {
    /// DCB field: `occludedAlpha` (Single)
    #[serde(default)]
    pub occluded_alpha: f32,
    /// DCB field: `outlineWidth` (Single)
    #[serde(default)]
    pub outline_width: f32,
    /// DCB field: `outlineOnly` (Boolean)
    #[serde(default)]
    pub outline_only: bool,
}

impl Pooled for SSalvageRepairHighlightOutlineValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_highlight_outline_values }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_highlight_outline_values }
}

impl<'a> Extract<'a> for SSalvageRepairHighlightOutlineValues {
    const TYPE_NAME: &'static str = "SSalvageRepairHighlightOutlineValues";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            occluded_alpha: inst.get_f32("occludedAlpha").unwrap_or_default(),
            outline_width: inst.get_f32("outlineWidth").unwrap_or_default(),
            outline_only: inst.get_bool("outlineOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairHighlightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairHighlightParams {
    /// DCB field: `colors` (Class (array))
    #[serde(default)]
    pub colors: Vec<Handle<SSalvageRepairHighlightColorParams>>,
    /// DCB field: `validOutlineValues` (Class)
    #[serde(default)]
    pub valid_outline_values: Option<Handle<SSalvageRepairHighlightOutlineValues>>,
    /// DCB field: `invalidTargetColor` (Class)
    #[serde(default)]
    pub invalid_target_color: Option<Handle<RGB>>,
    /// DCB field: `invalidOutlineValues` (Class)
    #[serde(default)]
    pub invalid_outline_values: Option<Handle<SSalvageRepairHighlightOutlineValues>>,
    /// DCB field: `filterItemTypes` (Class (array))
    #[serde(default)]
    pub filter_item_types: Vec<Handle<SItemTypeFilter>>,
}

impl Pooled for SSalvageRepairHighlightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_highlight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_highlight_params }
}

impl<'a> Extract<'a> for SSalvageRepairHighlightParams {
    const TYPE_NAME: &'static str = "SSalvageRepairHighlightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            colors: inst.get_array("colors")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSalvageRepairHighlightColorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SSalvageRepairHighlightColorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            valid_outline_values: match inst.get("validOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageRepairHighlightOutlineValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invalid_target_color: match inst.get("invalidTargetColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invalid_outline_values: match inst.get("invalidOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageRepairHighlightOutlineValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            filter_item_types: inst.get_array("filterItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairCardParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairCardParams {
    /// DCB field: `cardLerpSpeed` (Single)
    #[serde(default)]
    pub card_lerp_speed: f32,
    /// DCB field: `attachPointLerpSpeed` (Single)
    #[serde(default)]
    pub attach_point_lerp_speed: f32,
    /// DCB field: `closingTransitionTime` (Single)
    #[serde(default)]
    pub closing_transition_time: f32,
    /// DCB field: `nearDistance` (Single)
    #[serde(default)]
    pub near_distance: f32,
    /// DCB field: `defaultScreenPos` (Class)
    #[serde(default)]
    pub default_screen_pos: Option<Handle<Vec2>>,
    /// DCB field: `maxDistScreenPosScale` (Single)
    #[serde(default)]
    pub max_dist_screen_pos_scale: f32,
}

impl Pooled for SSalvageRepairCardParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_card_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_card_params }
}

impl<'a> Extract<'a> for SSalvageRepairCardParams {
    const TYPE_NAME: &'static str = "SSalvageRepairCardParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            card_lerp_speed: inst.get_f32("cardLerpSpeed").unwrap_or_default(),
            attach_point_lerp_speed: inst.get_f32("attachPointLerpSpeed").unwrap_or_default(),
            closing_transition_time: inst.get_f32("closingTransitionTime").unwrap_or_default(),
            near_distance: inst.get_f32("nearDistance").unwrap_or_default(),
            default_screen_pos: match inst.get("defaultScreenPos") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_dist_screen_pos_scale: inst.get_f32("maxDistScreenPosScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairItemTypeLocalizationPair`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairItemTypeLocalizationPair {
    /// DCB field: `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `typeLoc` (Locale)
    #[serde(default)]
    pub type_loc: String,
}

impl Pooled for SSalvageRepairItemTypeLocalizationPair {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_item_type_localization_pair }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_item_type_localization_pair }
}

impl<'a> Extract<'a> for SSalvageRepairItemTypeLocalizationPair {
    const TYPE_NAME: &'static str = "SSalvageRepairItemTypeLocalizationPair";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            type_loc: inst.get_str("typeLoc").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairLocalizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairLocalizationParams {
    /// DCB field: `hullLoc` (Locale)
    #[serde(default)]
    pub hull_loc: String,
    /// DCB field: `itemTypeLocalizationPairs` (Class (array))
    #[serde(default)]
    pub item_type_localization_pairs: Vec<Handle<SSalvageRepairItemTypeLocalizationPair>>,
    /// DCB field: `itemTypeNotFoundLoc` (Locale)
    #[serde(default)]
    pub item_type_not_found_loc: String,
}

impl Pooled for SSalvageRepairLocalizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_localization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_localization_params }
}

impl<'a> Extract<'a> for SSalvageRepairLocalizationParams {
    const TYPE_NAME: &'static str = "SSalvageRepairLocalizationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hull_loc: inst.get_str("hullLoc").map(String::from).unwrap_or_default(),
            item_type_localization_pairs: inst.get_array("itemTypeLocalizationPairs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSalvageRepairItemTypeLocalizationPair>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SSalvageRepairItemTypeLocalizationPair>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_type_not_found_loc: inst.get_str("itemTypeNotFoundLoc").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairMaterialParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairMaterialParams {
    /// DCB field: `hullThicknessMeters` (Single)
    #[serde(default)]
    pub hull_thickness_meters: f32,
    /// DCB field: `ammoToMaterialFactor` (Single)
    #[serde(default)]
    pub ammo_to_material_factor: f32,
    /// DCB field: `RMCResourceType` (Reference)
    #[serde(default)]
    pub rmcresource_type: Option<CigGuid>,
}

impl Pooled for SSalvageRepairMaterialParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_material_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_material_params }
}

impl<'a> Extract<'a> for SSalvageRepairMaterialParams {
    const TYPE_NAME: &'static str = "SSalvageRepairMaterialParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hull_thickness_meters: inst.get_f32("hullThicknessMeters").unwrap_or_default(),
            ammo_to_material_factor: inst.get_f32("ammoToMaterialFactor").unwrap_or_default(),
            rmcresource_type: inst.get("RMCResourceType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SSalvageRepairAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairAudioParams {
    /// DCB field: `salvageCargoOccupancyFactorRTPC` (Class)
    #[serde(default)]
    pub salvage_cargo_occupancy_factor_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `friendlyFireMessageCooldownScale` (Single)
    #[serde(default)]
    pub friendly_fire_message_cooldown_scale: f32,
}

impl Pooled for SSalvageRepairAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_repair_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_repair_audio_params }
}

impl<'a> Extract<'a> for SSalvageRepairAudioParams {
    const TYPE_NAME: &'static str = "SSalvageRepairAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            salvage_cargo_occupancy_factor_rtpc: match inst.get("salvageCargoOccupancyFactorRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            friendly_fire_message_cooldown_scale: inst.get_f32("friendlyFireMessageCooldownScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSharedInteractionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSharedInteractionParams {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `QueryTags` (Reference (array))
    #[serde(default)]
    pub query_tags: Vec<CigGuid>,
    /// DCB field: `RoomTag` (Reference)
    #[serde(default)]
    pub room_tag: Option<CigGuid>,
    /// DCB field: `UsableTag` (Reference)
    #[serde(default)]
    pub usable_tag: Option<CigGuid>,
    /// DCB field: `LinkingTag` (Reference)
    #[serde(default)]
    pub linking_tag: Option<CigGuid>,
    /// DCB field: `DisplayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `DisplayType` (EnumChoice)
    #[serde(default)]
    pub display_type: String,
    /// DCB field: `GenericCursor` (EnumChoice)
    #[serde(default)]
    pub generic_cursor: String,
    /// DCB field: `FocusModeOnly` (Boolean)
    #[serde(default)]
    pub focus_mode_only: bool,
    /// DCB field: `Sendable` (Boolean)
    #[serde(default)]
    pub sendable: bool,
    /// DCB field: `Linkable` (Boolean)
    #[serde(default)]
    pub linkable: bool,
    /// DCB field: `LockedByLinks` (Boolean)
    #[serde(default)]
    pub locked_by_links: bool,
    /// DCB field: `RequiresAuthorizedUser` (Boolean)
    #[serde(default)]
    pub requires_authorized_user: bool,
    /// DCB field: `available` (Boolean)
    #[serde(default)]
    pub available: bool,
    /// DCB field: `InheritsConditionDisplaysThroughLinks` (Boolean)
    #[serde(default)]
    pub inherits_condition_displays_through_links: bool,
    /// DCB field: `conditionList` (StrongPointer)
    #[serde(default)]
    pub condition_list: Option<Handle<InteractionConditionList>>,
    /// DCB field: `playerAnimatedInteractionBase` (StrongPointer)
    #[serde(default)]
    pub player_animated_interaction_base: Option<Handle<PlayerAnimatedInteractionBase>>,
}

impl Pooled for SSharedInteractionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sshared_interaction_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sshared_interaction_params }
}

impl<'a> Extract<'a> for SSharedInteractionParams {
    const TYPE_NAME: &'static str = "SSharedInteractionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            query_tags: inst.get_array("QueryTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            room_tag: inst.get("RoomTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            usable_tag: inst.get("UsableTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            linking_tag: inst.get("LinkingTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_name: inst.get_str("DisplayName").map(String::from).unwrap_or_default(),
            display_type: inst.get_str("DisplayType").map(String::from).unwrap_or_default(),
            generic_cursor: inst.get_str("GenericCursor").map(String::from).unwrap_or_default(),
            focus_mode_only: inst.get_bool("FocusModeOnly").unwrap_or_default(),
            sendable: inst.get_bool("Sendable").unwrap_or_default(),
            linkable: inst.get_bool("Linkable").unwrap_or_default(),
            locked_by_links: inst.get_bool("LockedByLinks").unwrap_or_default(),
            requires_authorized_user: inst.get_bool("RequiresAuthorizedUser").unwrap_or_default(),
            available: inst.get_bool("available").unwrap_or_default(),
            inherits_condition_displays_through_links: inst.get_bool("InheritsConditionDisplaysThroughLinks").unwrap_or_default(),
            condition_list: match inst.get("conditionList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InteractionConditionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InteractionConditionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_animated_interaction_base: match inst.get("playerAnimatedInteractionBase") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerAnimatedInteractionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerAnimatedInteractionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSilhouetteParamsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSilhouetteParamsDef {
    /// DCB field: `Enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `ColourSource` (EnumChoice)
    #[serde(default)]
    pub colour_source: String,
    /// DCB field: `TintColour` (Class)
    #[serde(default)]
    pub tint_colour: Option<Handle<RGB>>,
    /// DCB field: `TintStrength` (Single)
    #[serde(default)]
    pub tint_strength: f32,
    /// DCB field: `Brightness` (Single)
    #[serde(default)]
    pub brightness: f32,
    /// DCB field: `EdgeWidth` (Single)
    #[serde(default)]
    pub edge_width: f32,
    /// DCB field: `EdgeIntensity` (Single)
    #[serde(default)]
    pub edge_intensity: f32,
    /// DCB field: `FillIntensity` (Single)
    #[serde(default)]
    pub fill_intensity: f32,
    /// DCB field: `BlurRadius` (Single)
    #[serde(default)]
    pub blur_radius: f32,
    /// DCB field: `EdgeGradient` (Single)
    #[serde(default)]
    pub edge_gradient: f32,
}

impl Pooled for SSilhouetteParamsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssilhouette_params_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssilhouette_params_def }
}

impl<'a> Extract<'a> for SSilhouetteParamsDef {
    const TYPE_NAME: &'static str = "SSilhouetteParamsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("Enable").unwrap_or_default(),
            colour_source: inst.get_str("ColourSource").map(String::from).unwrap_or_default(),
            tint_colour: match inst.get("TintColour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tint_strength: inst.get_f32("TintStrength").unwrap_or_default(),
            brightness: inst.get_f32("Brightness").unwrap_or_default(),
            edge_width: inst.get_f32("EdgeWidth").unwrap_or_default(),
            edge_intensity: inst.get_f32("EdgeIntensity").unwrap_or_default(),
            fill_intensity: inst.get_f32("FillIntensity").unwrap_or_default(),
            blur_radius: inst.get_f32("BlurRadius").unwrap_or_default(),
            edge_gradient: inst.get_f32("EdgeGradient").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageGlobalStructuralVFXParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageGlobalStructuralVFXParams {
    /// DCB field: `sourceResonateTag` (Reference)
    #[serde(default)]
    pub source_resonate_tag: Option<CigGuid>,
    /// DCB field: `sourceDisintegrateTag` (Reference)
    #[serde(default)]
    pub source_disintegrate_tag: Option<CigGuid>,
    /// DCB field: `sourceDisintegrateBeamTag` (Reference)
    #[serde(default)]
    pub source_disintegrate_beam_tag: Option<CigGuid>,
    /// DCB field: `sourceFieldMaterialsTag` (Reference)
    #[serde(default)]
    pub source_field_materials_tag: Option<CigGuid>,
    /// DCB field: `sourceIdleFieldStrengthValue` (Single)
    #[serde(default)]
    pub source_idle_field_strength_value: f32,
    /// DCB field: `sourceMinEngagedFieldStrengthValue` (Single)
    #[serde(default)]
    pub source_min_engaged_field_strength_value: f32,
    /// DCB field: `sourceFieldStrengthLinkTag` (Reference)
    #[serde(default)]
    pub source_field_strength_link_tag: Option<CigGuid>,
    /// DCB field: `targetDisintegrateDissolveTime` (Single)
    #[serde(default)]
    pub target_disintegrate_dissolve_time: f32,
    /// DCB field: `targetResonateParticle` (Class)
    #[serde(default)]
    pub target_resonate_particle: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `targetBreakParticle` (Class)
    #[serde(default)]
    pub target_break_particle: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `targetDisintegrateParticle` (Class)
    #[serde(default)]
    pub target_disintegrate_particle: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `targetDisintegrateDebrisParticle` (Class)
    #[serde(default)]
    pub target_disintegrate_debris_particle: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `vectorFieldPath` (String)
    #[serde(default)]
    pub vector_field_path: String,
}

impl Pooled for SSalvageGlobalStructuralVFXParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_global_structural_vfxparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_global_structural_vfxparams }
}

impl<'a> Extract<'a> for SSalvageGlobalStructuralVFXParams {
    const TYPE_NAME: &'static str = "SSalvageGlobalStructuralVFXParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            source_resonate_tag: inst.get("sourceResonateTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            source_disintegrate_tag: inst.get("sourceDisintegrateTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            source_disintegrate_beam_tag: inst.get("sourceDisintegrateBeamTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            source_field_materials_tag: inst.get("sourceFieldMaterialsTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            source_idle_field_strength_value: inst.get_f32("sourceIdleFieldStrengthValue").unwrap_or_default(),
            source_min_engaged_field_strength_value: inst.get_f32("sourceMinEngagedFieldStrengthValue").unwrap_or_default(),
            source_field_strength_link_tag: inst.get("sourceFieldStrengthLinkTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            target_disintegrate_dissolve_time: inst.get_f32("targetDisintegrateDissolveTime").unwrap_or_default(),
            target_resonate_particle: match inst.get("targetResonateParticle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_break_particle: match inst.get("targetBreakParticle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_disintegrate_particle: match inst.get("targetDisintegrateParticle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_disintegrate_debris_particle: match inst.get("targetDisintegrateDebrisParticle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vector_field_path: inst.get_str("vectorFieldPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageGlobalStructuralHighlightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageGlobalStructuralHighlightParams {
    /// DCB field: `highlightOccludedAlpha` (Single)
    #[serde(default)]
    pub highlight_occluded_alpha: f32,
    /// DCB field: `highlightOutlineWidth` (Single)
    #[serde(default)]
    pub highlight_outline_width: f32,
    /// DCB field: `highlightOutlineOnly` (Boolean)
    #[serde(default)]
    pub highlight_outline_only: bool,
    /// DCB field: `fractureValidHighlightColor` (Class)
    #[serde(default)]
    pub fracture_valid_highlight_color: Option<Handle<RGB>>,
    /// DCB field: `fractureErrorHighlightColor` (Class)
    #[serde(default)]
    pub fracture_error_highlight_color: Option<Handle<RGB>>,
    /// DCB field: `disintegrationValidHighlightColor` (Class)
    #[serde(default)]
    pub disintegration_valid_highlight_color: Option<Handle<RGB>>,
    /// DCB field: `disintegrationWarningHighlightColor` (Class)
    #[serde(default)]
    pub disintegration_warning_highlight_color: Option<Handle<RGB>>,
    /// DCB field: `disintegrationErrorHighlightColor` (Class)
    #[serde(default)]
    pub disintegration_error_highlight_color: Option<Handle<RGB>>,
}

impl Pooled for SSalvageGlobalStructuralHighlightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_global_structural_highlight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_global_structural_highlight_params }
}

impl<'a> Extract<'a> for SSalvageGlobalStructuralHighlightParams {
    const TYPE_NAME: &'static str = "SSalvageGlobalStructuralHighlightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            highlight_occluded_alpha: inst.get_f32("highlightOccludedAlpha").unwrap_or_default(),
            highlight_outline_width: inst.get_f32("highlightOutlineWidth").unwrap_or_default(),
            highlight_outline_only: inst.get_bool("highlightOutlineOnly").unwrap_or_default(),
            fracture_valid_highlight_color: match inst.get("fractureValidHighlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fracture_error_highlight_color: match inst.get("fractureErrorHighlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disintegration_valid_highlight_color: match inst.get("disintegrationValidHighlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disintegration_warning_highlight_color: match inst.get("disintegrationWarningHighlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disintegration_error_highlight_color: match inst.get("disintegrationErrorHighlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSpreadModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSpreadModifier {
    /// DCB field: `minMultiplier` (Single)
    #[serde(default)]
    pub min_multiplier: f32,
    /// DCB field: `maxMultiplier` (Single)
    #[serde(default)]
    pub max_multiplier: f32,
    /// DCB field: `firstAttackMultiplier` (Single)
    #[serde(default)]
    pub first_attack_multiplier: f32,
    /// DCB field: `attackMultiplier` (Single)
    #[serde(default)]
    pub attack_multiplier: f32,
    /// DCB field: `decayMultiplier` (Single)
    #[serde(default)]
    pub decay_multiplier: f32,
    /// DCB field: `additiveModifier` (Single)
    #[serde(default)]
    pub additive_modifier: f32,
}

impl Pooled for SSpreadModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sspread_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sspread_modifier }
}

impl<'a> Extract<'a> for SSpreadModifier {
    const TYPE_NAME: &'static str = "SSpreadModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_multiplier: inst.get_f32("minMultiplier").unwrap_or_default(),
            max_multiplier: inst.get_f32("maxMultiplier").unwrap_or_default(),
            first_attack_multiplier: inst.get_f32("firstAttackMultiplier").unwrap_or_default(),
            attack_multiplier: inst.get_f32("attackMultiplier").unwrap_or_default(),
            decay_multiplier: inst.get_f32("decayMultiplier").unwrap_or_default(),
            additive_modifier: inst.get_f32("additiveModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageModifier {
    /// DCB field: `salvageSpeedMultiplier` (Single)
    #[serde(default)]
    pub salvage_speed_multiplier: f32,
    /// DCB field: `radiusMultiplier` (Single)
    #[serde(default)]
    pub radius_multiplier: f32,
    /// DCB field: `extractionEfficiency` (Single)
    #[serde(default)]
    pub extraction_efficiency: f32,
}

impl Pooled for SSalvageModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssalvage_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssalvage_modifier }
}

impl<'a> Extract<'a> for SSalvageModifier {
    const TYPE_NAME: &'static str = "SSalvageModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            salvage_speed_multiplier: inst.get_f32("salvageSpeedMultiplier").unwrap_or_default(),
            radius_multiplier: inst.get_f32("radiusMultiplier").unwrap_or_default(),
            extraction_efficiency: inst.get_f32("extractionEfficiency").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSCRadarContactProperites`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCRadarContactProperites {
    /// DCB field: `contactType` (Reference)
    #[serde(default)]
    pub contact_type: Option<CigGuid>,
    /// DCB field: `baseSignatureParams` (StrongPointer)
    #[serde(default)]
    pub base_signature_params: Option<Handle<SSCSignatureParamsBase>>,
    /// DCB field: `crossSectionParams` (StrongPointer)
    #[serde(default)]
    pub cross_section_params: Option<Handle<SSCSignatureSystemCrossSectionParams>>,
    /// DCB field: `emissionModifierParams` (StrongPointer)
    #[serde(default)]
    pub emission_modifier_params: Option<Handle<SSCSignatureEmissionBaseModifier>>,
    /// DCB field: `deathParams` (StrongPointer)
    #[serde(default)]
    pub death_params: Option<Handle<SCSignatureDeathParams>>,
    /// DCB field: `scanBounds` (StrongPointer)
    #[serde(default)]
    pub scan_bounds: Option<Handle<SSCSignatureSystemScanBounds>>,
    /// DCB field: `roomParams` (Class)
    #[serde(default)]
    pub room_params: Option<Handle<SCSignatureSystemRoomParams>>,
}

impl Pooled for SSCRadarContactProperites {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscradar_contact_properites }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscradar_contact_properites }
}

impl<'a> Extract<'a> for SSCRadarContactProperites {
    const TYPE_NAME: &'static str = "SSCRadarContactProperites";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            contact_type: inst.get("contactType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            base_signature_params: match inst.get("baseSignatureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cross_section_params: match inst.get("crossSectionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureSystemCrossSectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureSystemCrossSectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emission_modifier_params: match inst.get("emissionModifierParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureEmissionBaseModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureEmissionBaseModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            death_params: match inst.get("deathParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCSignatureDeathParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCSignatureDeathParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_bounds: match inst.get("scanBounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureSystemScanBounds>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureSystemScanBounds>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_params: match inst.get("roomParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCSignatureSystemRoomParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCSignatureSystemRoomParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSCSignatureSystemScanBounds`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemScanBounds {
    /// DCB field: `min` (Class)
    #[serde(default)]
    pub min: Option<Handle<Vec3>>,
    /// DCB field: `max` (Class)
    #[serde(default)]
    pub max: Option<Handle<Vec3>>,
    /// DCB field: `localRotation` (Class)
    #[serde(default)]
    pub local_rotation: Option<Handle<Deg3>>,
}

impl Pooled for SSCSignatureSystemScanBounds {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_scan_bounds }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_scan_bounds }
}

impl<'a> Extract<'a> for SSCSignatureSystemScanBounds {
    const TYPE_NAME: &'static str = "SSCSignatureSystemScanBounds";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min: match inst.get("min") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max: match inst.get("max") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            local_rotation: match inst.get("localRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Deg3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSCSignatureEmissionBaseModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureEmissionBaseModifier {
}

impl Pooled for SSCSignatureEmissionBaseModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_emission_base_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_emission_base_modifier }
}

impl<'a> Extract<'a> for SSCSignatureEmissionBaseModifier {
    const TYPE_NAME: &'static str = "SSCSignatureEmissionBaseModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureParamsBase {
}

impl Pooled for SSCSignatureParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_params_base }
}

impl<'a> Extract<'a> for SSCSignatureParamsBase {
    const TYPE_NAME: &'static str = "SSCSignatureParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemCrossSectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemCrossSectionParams {
}

impl Pooled for SSCSignatureSystemCrossSectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_cross_section_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_cross_section_params }
}

impl<'a> Extract<'a> for SSCSignatureSystemCrossSectionParams {
    const TYPE_NAME: &'static str = "SSCSignatureSystemCrossSectionParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioModifier {
}

impl Pooled for SSCSignatureSystemAudioModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_audio_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_audio_modifier }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioModifier {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioSubRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioSubRule {
}

impl Pooled for SSCSignatureSystemAudioSubRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_audio_sub_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_audio_sub_rule }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioSubRule {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioSubRule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioRule {
    /// DCB field: `subRules` (StrongPointer (array))
    #[serde(default)]
    pub sub_rules: Vec<Handle<SSCSignatureSystemAudioSubRule>>,
    /// DCB field: `modifier` (StrongPointer)
    #[serde(default)]
    pub modifier: Option<Handle<SSCSignatureSystemAudioModifier>>,
}

impl Pooled for SSCSignatureSystemAudioRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_audio_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_audio_rule }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioRule {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioRule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sub_rules: inst.get_array("subRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSCSignatureSystemAudioSubRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SSCSignatureSystemAudioSubRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            modifier: match inst.get("modifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureSystemAudioModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureSystemAudioModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioRuleset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioRuleset {
    /// DCB field: `rules` (StrongPointer (array))
    #[serde(default)]
    pub rules: Vec<Handle<SSCSignatureSystemAudioRule>>,
}

impl Pooled for SSCSignatureSystemAudioRuleset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_audio_ruleset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_audio_ruleset }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioRuleset {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioRuleset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSCSignatureSystemAudioRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SSCSignatureSystemAudioRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioParams {
    /// DCB field: `rulesets` (Reference (array))
    #[serde(default)]
    pub rulesets: Vec<CigGuid>,
}

impl Pooled for SSCSignatureSystemAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_audio_params }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioParams {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rulesets: inst.get_array("rulesets")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SSCSignatureSystemParams`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemParams {
    /// DCB field: `bindingURLPrefix` (String)
    #[serde(default)]
    pub binding_urlprefix: String,
    /// DCB field: `radarProperties` (StrongPointer)
    #[serde(default)]
    pub radar_properties: Option<Handle<SSCRadarContactProperites>>,
    /// DCB field: `audioParams` (StrongPointer)
    #[serde(default)]
    pub audio_params: Option<Handle<SSCSignatureSystemAudioParams>>,
    /// DCB field: `scanCustomData` (Reference)
    #[serde(default)]
    pub scan_custom_data: Option<CigGuid>,
    /// DCB field: `embeddedScanInfo` (StrongPointer)
    #[serde(default)]
    pub embedded_scan_info: Option<Handle<ScanCustomDataInfo>>,
    /// DCB field: `scanDisplayLayoutOverride` (Reference)
    #[serde(default)]
    pub scan_display_layout_override: Option<CigGuid>,
    /// DCB field: `detectionTags` (Reference (array))
    #[serde(default)]
    pub detection_tags: Vec<CigGuid>,
    /// DCB field: `isOverridden` (Boolean)
    #[serde(default)]
    pub is_overridden: bool,
    /// DCB field: `overriddenSize` (Class)
    #[serde(default)]
    pub overridden_size: Option<Handle<Vec3>>,
    /// DCB field: `enableDetectionOnItemPort` (Boolean)
    #[serde(default)]
    pub enable_detection_on_item_port: bool,
    /// DCB field: `ignoreHighlightWhenDetectorInsideBounds` (Boolean)
    #[serde(default)]
    pub ignore_highlight_when_detector_inside_bounds: bool,
    /// DCB field: `linkedObjectives` (Reference (array))
    #[serde(default)]
    pub linked_objectives: Vec<CigGuid>,
    /// DCB field: `ignoreHighlightWhenNoLinkedOrActiveObjectives` (Boolean)
    #[serde(default)]
    pub ignore_highlight_when_no_linked_or_active_objectives: bool,
    /// DCB field: `priorityBoxoutTag` (Reference)
    #[serde(default)]
    pub priority_boxout_tag: Option<CigGuid>,
    /// DCB field: `isObjectOfInterest` (Boolean)
    #[serde(default)]
    pub is_object_of_interest: bool,
}

impl Pooled for SSCSignatureSystemParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscsignature_system_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscsignature_system_params }
}

impl<'a> Extract<'a> for SSCSignatureSystemParams {
    const TYPE_NAME: &'static str = "SSCSignatureSystemParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            binding_urlprefix: inst.get_str("bindingURLPrefix").map(String::from).unwrap_or_default(),
            radar_properties: match inst.get("radarProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCRadarContactProperites>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCRadarContactProperites>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_params: match inst.get("audioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureSystemAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureSystemAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_custom_data: inst.get("scanCustomData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            embedded_scan_info: match inst.get("embeddedScanInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanCustomDataInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanCustomDataInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_display_layout_override: inst.get("scanDisplayLayoutOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            detection_tags: inst.get_array("detectionTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            is_overridden: inst.get_bool("isOverridden").unwrap_or_default(),
            overridden_size: match inst.get("overriddenSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_detection_on_item_port: inst.get_bool("enableDetectionOnItemPort").unwrap_or_default(),
            ignore_highlight_when_detector_inside_bounds: inst.get_bool("ignoreHighlightWhenDetectorInsideBounds").unwrap_or_default(),
            linked_objectives: inst.get_array("linkedObjectives")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            ignore_highlight_when_no_linked_or_active_objectives: inst.get_bool("ignoreHighlightWhenNoLinkedOrActiveObjectives").unwrap_or_default(),
            priority_boxout_tag: inst.get("priorityBoxoutTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            is_object_of_interest: inst.get_bool("isObjectOfInterest").unwrap_or_default(),
        }
    }
}

/// DCB type: `SScenarioProgressRewardsTiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScenarioProgressRewardsTiers {
    /// DCB field: `faction` (Reference)
    #[serde(default)]
    pub faction: Option<CigGuid>,
    /// DCB field: `convertTiersPointsToPercent` (Boolean)
    #[serde(default)]
    pub convert_tiers_points_to_percent: bool,
    /// DCB field: `tierProgressions` (Class (array))
    #[serde(default)]
    pub tier_progressions: Vec<Handle<STierProgressions>>,
    /// DCB field: `rewardPeriodical` (Class (array))
    #[serde(default)]
    pub reward_periodical: Vec<Handle<SRewardPeriodical>>,
}

impl Pooled for SScenarioProgressRewardsTiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscenario_progress_rewards_tiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscenario_progress_rewards_tiers }
}

impl<'a> Extract<'a> for SScenarioProgressRewardsTiers {
    const TYPE_NAME: &'static str = "SScenarioProgressRewardsTiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            faction: inst.get("faction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            convert_tiers_points_to_percent: inst.get_bool("convertTiersPointsToPercent").unwrap_or_default(),
            tier_progressions: inst.get_array("tierProgressions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<STierProgressions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<STierProgressions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            reward_periodical: inst.get_array("rewardPeriodical")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SRewardPeriodical>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SRewardPeriodical>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SServiceBeaconCreatorParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SServiceBeaconCreatorParamsBase {
}

impl Pooled for SServiceBeaconCreatorParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sservice_beacon_creator_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sservice_beacon_creator_params_base }
}

impl<'a> Extract<'a> for SServiceBeaconCreatorParamsBase {
    const TYPE_NAME: &'static str = "SServiceBeaconCreatorParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SServiceBeaconCreatorParams`
///
/// Inherits from: `SServiceBeaconCreatorParamsBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SServiceBeaconCreatorParams {
    /// DCB field: `missionEntry` (Reference)
    #[serde(default)]
    pub mission_entry: Option<CigGuid>,
}

impl Pooled for SServiceBeaconCreatorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sservice_beacon_creator_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sservice_beacon_creator_params }
}

impl<'a> Extract<'a> for SServiceBeaconCreatorParams {
    const TYPE_NAME: &'static str = "SServiceBeaconCreatorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_entry: inst.get("missionEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SSolarSystem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSolarSystem {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `landingZoneInventory` (Reference)
    #[serde(default)]
    pub landing_zone_inventory: Option<CigGuid>,
    /// DCB field: `DefaultLocation` (Reference)
    #[serde(default)]
    pub default_location: Option<CigGuid>,
    /// DCB field: `SolarSystemRecord` (Reference)
    #[serde(default)]
    pub solar_system_record: Option<CigGuid>,
    /// DCB field: `galacticPosition` (Class)
    #[serde(default)]
    pub galactic_position: Option<Handle<Vec3>>,
    /// DCB field: `organicFallbackBedrockTint` (Class)
    #[serde(default)]
    pub organic_fallback_bedrock_tint: Option<Handle<RGB>>,
    /// DCB field: `organicFallbackSoilTint` (Class)
    #[serde(default)]
    pub organic_fallback_soil_tint: Option<Handle<RGB>>,
}

impl Pooled for SSolarSystem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.ssolar_system }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.ssolar_system }
}

impl<'a> Extract<'a> for SSolarSystem {
    const TYPE_NAME: &'static str = "SSolarSystem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            landing_zone_inventory: inst.get("landingZoneInventory").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_location: inst.get("DefaultLocation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            solar_system_record: inst.get("SolarSystemRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            galactic_position: match inst.get("galacticPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            organic_fallback_bedrock_tint: match inst.get("organicFallbackBedrockTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            organic_fallback_soil_tint: match inst.get("organicFallbackSoilTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SScenePlayerChoiceSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScenePlayerChoiceSettings {
    /// DCB field: `iconParams` (Class)
    #[serde(default)]
    pub icon_params: Option<Handle<SConversationIconParams>>,
}

impl Pooled for SScenePlayerChoiceSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ss.sscene_player_choice_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ss.sscene_player_choice_settings }
}

impl<'a> Extract<'a> for SScenePlayerChoiceSettings {
    const TYPE_NAME: &'static str = "SScenePlayerChoiceSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            icon_params: match inst.get("iconParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SConversationIconParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SConversationIconParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

