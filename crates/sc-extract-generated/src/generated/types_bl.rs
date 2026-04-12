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

/// DCB type: `BlueprintCategoryRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintCategoryRecord {
}

impl Pooled for BlueprintCategoryRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blueprint_category_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blueprint_category_record }
}

impl<'a> Extract<'a> for BlueprintCategoryRecord {
    const TYPE_NAME: &'static str = "BlueprintCategoryRecord";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BlueprintCategoryDatabaseRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintCategoryDatabaseRecord {
    /// DCB field: `categories` (Reference (array))
    #[serde(default)]
    pub categories: Vec<CigGuid>,
}

impl Pooled for BlueprintCategoryDatabaseRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blueprint_category_database_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blueprint_category_database_record }
}

impl<'a> Extract<'a> for BlueprintCategoryDatabaseRecord {
    const TYPE_NAME: &'static str = "BlueprintCategoryDatabaseRecord";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BlockedTextParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedTextParams {
    /// DCB field: `blockedText` (Locale)
    #[serde(default)]
    pub blocked_text: String,
}

impl Pooled for BlockedTextParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blocked_text_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blocked_text_params }
}

impl<'a> Extract<'a> for BlockedTextParams {
    const TYPE_NAME: &'static str = "BlockedTextParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blocked_text: inst.get_str("blockedText").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BlockedCursorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedCursorParams {
    /// DCB field: `blockedCursor` (EnumChoice)
    #[serde(default)]
    pub blocked_cursor: String,
}

impl Pooled for BlockedCursorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blocked_cursor_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blocked_cursor_params }
}

impl<'a> Extract<'a> for BlockedCursorParams {
    const TYPE_NAME: &'static str = "BlockedCursorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blocked_cursor: inst.get_str("blockedCursor").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BlockedColorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedColorParams {
    /// DCB field: `selectedColor` (Reference)
    #[serde(default)]
    pub selected_color: Option<CigGuid>,
    /// DCB field: `unselectedColor` (Reference)
    #[serde(default)]
    pub unselected_color: Option<CigGuid>,
    /// DCB field: `applyImmediately` (Boolean)
    #[serde(default)]
    pub apply_immediately: bool,
}

impl Pooled for BlockedColorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blocked_color_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blocked_color_params }
}

impl<'a> Extract<'a> for BlockedColorParams {
    const TYPE_NAME: &'static str = "BlockedColorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            selected_color: inst.get("selectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            unselected_color: inst.get("unselectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            apply_immediately: inst.get_bool("applyImmediately").unwrap_or_default(),
        }
    }
}

/// DCB type: `BlockedHintParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedHintParams {
    /// DCB field: `hintTrigger` (Reference)
    #[serde(default)]
    pub hint_trigger: Option<CigGuid>,
}

impl Pooled for BlockedHintParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blocked_hint_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blocked_hint_params }
}

impl<'a> Extract<'a> for BlockedHintParams {
    const TYPE_NAME: &'static str = "BlockedHintParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hint_trigger: inst.get("hintTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `BlueprintReward`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintReward {
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// DCB field: `blueprintRecord` (Reference)
    #[serde(default)]
    pub blueprint_record: Option<CigGuid>,
}

impl Pooled for BlueprintReward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blueprint_reward }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blueprint_reward }
}

impl<'a> Extract<'a> for BlueprintReward {
    const TYPE_NAME: &'static str = "BlueprintReward";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            weight: inst.get_f32("weight").unwrap_or_default(),
            blueprint_record: inst.get("blueprintRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `BlueprintPoolRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintPoolRecord {
    /// DCB field: `blueprintRewards` (Class (array))
    #[serde(default)]
    pub blueprint_rewards: Vec<Handle<BlueprintReward>>,
}

impl Pooled for BlueprintPoolRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blueprint_pool_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blueprint_pool_record }
}

impl<'a> Extract<'a> for BlueprintPoolRecord {
    const TYPE_NAME: &'static str = "BlueprintPoolRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blueprint_rewards: inst.get_array("blueprintRewards")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BlueprintReward>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BlueprintReward>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BlobVFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobVFXSharedParams {
    /// DCB field: `snapBlobTimescaleToLifeTime` (Boolean)
    #[serde(default)]
    pub snap_blob_timescale_to_life_time: bool,
    /// DCB field: `blobVFXs` (Class (array))
    #[serde(default)]
    pub blob_vfxs: Vec<Handle<BlobVFXDistanceParams>>,
}

impl Pooled for BlobVFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blob_vfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blob_vfxshared_params }
}

impl<'a> Extract<'a> for BlobVFXSharedParams {
    const TYPE_NAME: &'static str = "BlobVFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            snap_blob_timescale_to_life_time: inst.get_bool("snapBlobTimescaleToLifeTime").unwrap_or_default(),
            blob_vfxs: inst.get_array("blobVFXs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BlobVFXDistanceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BlobVFXDistanceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BlobVFXDistanceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobVFXDistanceParams {
    /// DCB field: `effect` (Class)
    #[serde(default)]
    pub effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `distance` (Double)
    #[serde(default)]
    pub distance: f64,
}

impl Pooled for BlobVFXDistanceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bl.blob_vfxdistance_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bl.blob_vfxdistance_params }
}

impl<'a> Extract<'a> for BlobVFXDistanceParams {
    const TYPE_NAME: &'static str = "BlobVFXDistanceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect: match inst.get("effect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance: inst.get_f64("distance").unwrap_or_default(),
        }
    }
}

