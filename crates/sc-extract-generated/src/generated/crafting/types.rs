// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `crafting`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `LegacyCraftingRecipe_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipe_Base {
}

impl Pooled for LegacyCraftingRecipe_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.legacy_crafting_recipe_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.legacy_crafting_recipe_base }
}

impl<'a> Extract<'a> for LegacyCraftingRecipe_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipe_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingRecipeDef_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeDef_Base {
}

impl Pooled for LegacyCraftingRecipeDef_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.legacy_crafting_recipe_def_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.legacy_crafting_recipe_def_base }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeDef_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeDef_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingRecipeDefRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeDefRecord {
    /// `recipe` (StrongPointer)
    #[serde(default)]
    pub recipe: Option<Handle<LegacyCraftingRecipe_Base>>,
}

impl Pooled for LegacyCraftingRecipeDefRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.legacy_crafting_recipe_def_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.legacy_crafting_recipe_def_record }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeDefRecord {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeDefRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recipe: match inst.get("recipe") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LegacyCraftingRecipe_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LegacyCraftingRecipe_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LegacyCraftingRecipeListRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeListRecord {
    /// `recipes` (StrongPointer (array))
    #[serde(default)]
    pub recipes: Vec<Handle<LegacyCraftingRecipeDef_Base>>,
}

impl Pooled for LegacyCraftingRecipeListRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.legacy_crafting_recipe_list_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.legacy_crafting_recipe_list_record }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeListRecord {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeListRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recipes: inst.get_array("recipes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LegacyCraftingRecipeDef_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LegacyCraftingRecipeDef_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CraftingGameplayPropertyDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingGameplayPropertyDef {
    /// `propertyName` (Locale)
    #[serde(default)]
    pub property_name: String,
    /// `unitFormat` (Locale)
    #[serde(default)]
    pub unit_format: String,
}

impl Pooled for CraftingGameplayPropertyDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_gameplay_property_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_gameplay_property_def }
}

impl<'a> Extract<'a> for CraftingGameplayPropertyDef {
    const TYPE_NAME: &'static str = "CraftingGameplayPropertyDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            property_name: inst.get_str("propertyName").map(String::from).unwrap_or_default(),
            unit_format: inst.get_str("unitFormat").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BlueprintCategoryRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintCategoryRecord {
}

impl Pooled for BlueprintCategoryRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.blueprint_category_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.blueprint_category_record }
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
    /// `categories` (Reference (array))
    #[serde(default)]
    pub categories: Vec<CigGuid>,
}

impl Pooled for BlueprintCategoryDatabaseRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.blueprint_category_database_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.blueprint_category_database_record }
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

/// DCB type: `CraftingBlueprint_Base_NonRef`
/// Inherits from: `CraftingBlueprint_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingBlueprint_Base_NonRef {
}

impl Pooled for CraftingBlueprint_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_blueprint_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_blueprint_base_non_ref }
}

impl<'a> Extract<'a> for CraftingBlueprint_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingBlueprint_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingBlueprintRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingBlueprintRecord {
    /// `blueprint` (StrongPointer)
    #[serde(default)]
    pub blueprint: Option<Handle<CraftingBlueprint_Base_NonRef>>,
}

impl Pooled for CraftingBlueprintRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_blueprint_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_blueprint_record }
}

impl<'a> Extract<'a> for CraftingBlueprintRecord {
    const TYPE_NAME: &'static str = "CraftingBlueprintRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blueprint: match inst.get("blueprint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CraftingBlueprint_Base_NonRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CraftingBlueprint_Base_NonRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultBlueprintSelection_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultBlueprintSelection_Base {
}

impl Pooled for DefaultBlueprintSelection_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.default_blueprint_selection_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.default_blueprint_selection_base }
}

impl<'a> Extract<'a> for DefaultBlueprintSelection_Base {
    const TYPE_NAME: &'static str = "DefaultBlueprintSelection_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingQualityDistribution_Base_NonRef`
/// Inherits from: `CraftingQualityDistribution_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityDistribution_Base_NonRef {
}

impl Pooled for CraftingQualityDistribution_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_quality_distribution_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_quality_distribution_base_non_ref }
}

impl<'a> Extract<'a> for CraftingQualityDistribution_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingQualityDistribution_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingQualityDistributionRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityDistributionRecord {
    /// `qualityDistribution` (StrongPointer)
    #[serde(default)]
    pub quality_distribution: Option<Handle<CraftingQualityDistribution_Base_NonRef>>,
}

impl Pooled for CraftingQualityDistributionRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_quality_distribution_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_quality_distribution_record }
}

impl<'a> Extract<'a> for CraftingQualityDistributionRecord {
    const TYPE_NAME: &'static str = "CraftingQualityDistributionRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            quality_distribution: match inst.get("qualityDistribution") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CraftingQualityDistribution_Base_NonRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CraftingQualityDistribution_Base_NonRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CraftingQualityLocationOverride_Base_NonRef`
/// Inherits from: `CraftingQualityLocationOverride_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityLocationOverride_Base_NonRef {
}

impl Pooled for CraftingQualityLocationOverride_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_quality_location_override_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_quality_location_override_base_non_ref }
}

impl<'a> Extract<'a> for CraftingQualityLocationOverride_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingQualityLocationOverride_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingQualityLocationOverrideRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityLocationOverrideRecord {
    /// `locationOverride` (StrongPointer)
    #[serde(default)]
    pub location_override: Option<Handle<CraftingQualityLocationOverride_Base_NonRef>>,
}

impl Pooled for CraftingQualityLocationOverrideRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_quality_location_override_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_quality_location_override_record }
}

impl<'a> Extract<'a> for CraftingQualityLocationOverrideRecord {
    const TYPE_NAME: &'static str = "CraftingQualityLocationOverrideRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_override: match inst.get("locationOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CraftingQualityLocationOverride_Base_NonRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CraftingQualityLocationOverride_Base_NonRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CraftingGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingGlobalParams {
    /// `refiningQualityUnitMultiplier` (Single)
    #[serde(default)]
    pub refining_quality_unit_multiplier: f32,
    /// `defaultCompositionQuality` (Int32)
    #[serde(default)]
    pub default_composition_quality: i32,
    /// `dismantleBlacklistResources` (Reference (array))
    #[serde(default)]
    pub dismantle_blacklist_resources: Vec<CigGuid>,
    /// `dismantleBlacklistEntityClasses` (Reference (array))
    #[serde(default)]
    pub dismantle_blacklist_entity_classes: Vec<CigGuid>,
    /// `defaultBlueprintSelection` (StrongPointer)
    #[serde(default)]
    pub default_blueprint_selection: Option<Handle<DefaultBlueprintSelection_Base>>,
}

impl Pooled for CraftingGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.crafting_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.crafting_global_params }
}

impl<'a> Extract<'a> for CraftingGlobalParams {
    const TYPE_NAME: &'static str = "CraftingGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            refining_quality_unit_multiplier: inst.get_f32("refiningQualityUnitMultiplier").unwrap_or_default(),
            default_composition_quality: inst.get_i32("defaultCompositionQuality").unwrap_or_default(),
            dismantle_blacklist_resources: inst.get_array("dismantleBlacklistResources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            dismantle_blacklist_entity_classes: inst.get_array("dismantleBlacklistEntityClasses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            default_blueprint_selection: match inst.get("defaultBlueprintSelection") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DefaultBlueprintSelection_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DefaultBlueprintSelection_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BlueprintReward`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintReward {
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// `blueprintRecord` (Reference)
    #[serde(default)]
    pub blueprint_record: Option<CigGuid>,
}

impl Pooled for BlueprintReward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.blueprint_reward }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.blueprint_reward }
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
    /// `blueprintRewards` (Class (array))
    #[serde(default)]
    pub blueprint_rewards: Vec<Handle<BlueprintReward>>,
}

impl Pooled for BlueprintPoolRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crafting.blueprint_pool_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crafting.blueprint_pool_record }
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

