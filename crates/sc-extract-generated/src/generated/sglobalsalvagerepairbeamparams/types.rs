// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sglobalsalvagerepairbeamparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SSalvageRepairHighlightColorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairHighlightColorParams {
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGB>>,
    /// `hullThreshold` (Single)
    #[serde(default)]
    pub hull_threshold: f32,
}

impl Pooled for SSalvageRepairHighlightColorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_highlight_color_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_highlight_color_params }
}

impl<'a> Extract<'a> for SSalvageRepairHighlightColorParams {
    const TYPE_NAME: &'static str = "SSalvageRepairHighlightColorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hull_threshold: inst.get_f32("hullThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairHighlightOutlineValues`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairHighlightOutlineValues {
    /// `occludedAlpha` (Single)
    #[serde(default)]
    pub occluded_alpha: f32,
    /// `outlineWidth` (Single)
    #[serde(default)]
    pub outline_width: f32,
    /// `outlineOnly` (Boolean)
    #[serde(default)]
    pub outline_only: bool,
}

impl Pooled for SSalvageRepairHighlightOutlineValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_highlight_outline_values }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_highlight_outline_values }
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

/// DCB type: `SItemTypeFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemTypeFilter {
    /// `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: EItemType,
    /// `itemSubTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_sub_types: Vec<EItemSubType>,
}

impl Pooled for SItemTypeFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.sitem_type_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.sitem_type_filter }
}

impl<'a> Extract<'a> for SItemTypeFilter {
    const TYPE_NAME: &'static str = "SItemTypeFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: EItemType::from_dcb_str(inst.get_str("itemType").unwrap_or("")),
            item_sub_types: inst.get_array("itemSubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(EItemSubType::from_dcb_str)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairHighlightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairHighlightParams {
    /// `colors` (Class (array))
    #[serde(default)]
    pub colors: Vec<Handle<SSalvageRepairHighlightColorParams>>,
    /// `validOutlineValues` (Class)
    #[serde(default)]
    pub valid_outline_values: Option<Handle<SSalvageRepairHighlightOutlineValues>>,
    /// `invalidTargetColor` (Class)
    #[serde(default)]
    pub invalid_target_color: Option<Handle<RGB>>,
    /// `invalidOutlineValues` (Class)
    #[serde(default)]
    pub invalid_outline_values: Option<Handle<SSalvageRepairHighlightOutlineValues>>,
    /// `filterItemTypes` (Class (array))
    #[serde(default)]
    pub filter_item_types: Vec<Handle<SItemTypeFilter>>,
}

impl Pooled for SSalvageRepairHighlightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_highlight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_highlight_params }
}

impl<'a> Extract<'a> for SSalvageRepairHighlightParams {
    const TYPE_NAME: &'static str = "SSalvageRepairHighlightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            colors: inst.get_array("colors")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSalvageRepairHighlightColorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SSalvageRepairHighlightColorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            valid_outline_values: match inst.get("validOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            invalid_target_color: match inst.get("invalidTargetColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            invalid_outline_values: match inst.get("invalidOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            filter_item_types: inst.get_array("filterItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairCardParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairCardParams {
    /// `cardLerpSpeed` (Single)
    #[serde(default)]
    pub card_lerp_speed: f32,
    /// `attachPointLerpSpeed` (Single)
    #[serde(default)]
    pub attach_point_lerp_speed: f32,
    /// `closingTransitionTime` (Single)
    #[serde(default)]
    pub closing_transition_time: f32,
    /// `nearDistance` (Single)
    #[serde(default)]
    pub near_distance: f32,
    /// `defaultScreenPos` (Class)
    #[serde(default)]
    pub default_screen_pos: Option<Handle<Vec2>>,
    /// `maxDistScreenPosScale` (Single)
    #[serde(default)]
    pub max_dist_screen_pos_scale: f32,
}

impl Pooled for SSalvageRepairCardParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_card_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_card_params }
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
                _ => None,
            },
            max_dist_screen_pos_scale: inst.get_f32("maxDistScreenPosScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairItemTypeLocalizationPair`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairItemTypeLocalizationPair {
    /// `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: EItemType,
    /// `typeLoc` (Locale)
    #[serde(default)]
    pub type_loc: LocaleKey,
}

impl Pooled for SSalvageRepairItemTypeLocalizationPair {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_item_type_localization_pair }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_item_type_localization_pair }
}

impl<'a> Extract<'a> for SSalvageRepairItemTypeLocalizationPair {
    const TYPE_NAME: &'static str = "SSalvageRepairItemTypeLocalizationPair";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: EItemType::from_dcb_str(inst.get_str("itemType").unwrap_or("")),
            type_loc: inst.get_str("typeLoc").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairLocalizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairLocalizationParams {
    /// `hullLoc` (Locale)
    #[serde(default)]
    pub hull_loc: LocaleKey,
    /// `itemTypeLocalizationPairs` (Class (array))
    #[serde(default)]
    pub item_type_localization_pairs: Vec<Handle<SSalvageRepairItemTypeLocalizationPair>>,
    /// `itemTypeNotFoundLoc` (Locale)
    #[serde(default)]
    pub item_type_not_found_loc: LocaleKey,
}

impl Pooled for SSalvageRepairLocalizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_localization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_localization_params }
}

impl<'a> Extract<'a> for SSalvageRepairLocalizationParams {
    const TYPE_NAME: &'static str = "SSalvageRepairLocalizationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hull_loc: inst.get_str("hullLoc").map(LocaleKey::from).unwrap_or_default(),
            item_type_localization_pairs: inst.get_array("itemTypeLocalizationPairs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSalvageRepairItemTypeLocalizationPair>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SSalvageRepairItemTypeLocalizationPair>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_type_not_found_loc: inst.get_str("itemTypeNotFoundLoc").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSalvageRepairMaterialParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSalvageRepairMaterialParams {
    /// `hullThicknessMeters` (Single)
    #[serde(default)]
    pub hull_thickness_meters: f32,
    /// `ammoToMaterialFactor` (Single)
    #[serde(default)]
    pub ammo_to_material_factor: f32,
    /// `RMCResourceType` (Reference)
    #[serde(default)]
    pub rmcresource_type: Option<CigGuid>,
}

impl Pooled for SSalvageRepairMaterialParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_material_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_material_params }
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
    /// `salvageCargoOccupancyFactorRTPC` (Class)
    #[serde(default)]
    pub salvage_cargo_occupancy_factor_rtpc: Option<Handle<AudioRtpc>>,
    /// `friendlyFireMessageCooldownScale` (Single)
    #[serde(default)]
    pub friendly_fire_message_cooldown_scale: f32,
}

impl Pooled for SSalvageRepairAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.ssalvage_repair_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.ssalvage_repair_audio_params }
}

impl<'a> Extract<'a> for SSalvageRepairAudioParams {
    const TYPE_NAME: &'static str = "SSalvageRepairAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            salvage_cargo_occupancy_factor_rtpc: match inst.get("salvageCargoOccupancyFactorRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            friendly_fire_message_cooldown_scale: inst.get_f32("friendlyFireMessageCooldownScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalSalvageRepairBeamParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalSalvageRepairBeamParams {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `cardParams` (Class)
    #[serde(default)]
    pub card_params: Option<Handle<SSalvageRepairCardParams>>,
    /// `highlightParams` (Class)
    #[serde(default)]
    pub highlight_params: Option<Handle<SSalvageRepairHighlightParams>>,
    /// `localizationParams` (Class)
    #[serde(default)]
    pub localization_params: Option<Handle<SSalvageRepairLocalizationParams>>,
    /// `materialParams` (Class)
    #[serde(default)]
    pub material_params: Option<Handle<SSalvageRepairMaterialParams>>,
    /// `globalSalvageAudioParams` (Class)
    #[serde(default)]
    pub global_salvage_audio_params: Option<Handle<SSalvageRepairAudioParams>>,
    /// `hitsPerSecond` (Single)
    #[serde(default)]
    pub hits_per_second: f32,
    /// `hitDuration` (Single)
    #[serde(default)]
    pub hit_duration: f32,
}

impl Pooled for SGlobalSalvageRepairBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalsalvagerepairbeamparams.sglobal_salvage_repair_beam_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalsalvagerepairbeamparams.sglobal_salvage_repair_beam_params }
}

impl<'a> Extract<'a> for SGlobalSalvageRepairBeamParams {
    const TYPE_NAME: &'static str = "SGlobalSalvageRepairBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            card_params: match inst.get("cardParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairCardParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairHighlightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            localization_params: match inst.get("localizationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairLocalizationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            material_params: match inst.get("materialParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairMaterialParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            global_salvage_audio_params: match inst.get("globalSalvageAudioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hits_per_second: inst.get_f32("hitsPerSecond").unwrap_or_default(),
            hit_duration: inst.get_f32("hitDuration").unwrap_or_default(),
        }
    }
}

