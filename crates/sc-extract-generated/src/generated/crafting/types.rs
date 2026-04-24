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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `LegacyCraftingOutput_ResourceAmount`
/// Inherits from: `LegacyCraftingOutput_Base`
pub struct LegacyCraftingOutput_ResourceAmount {
    /// `resource` (Reference)
    pub resource: Option<CigGuid>,
    /// `amount` (StrongPointer)
    pub amount: Option<SBaseCargoUnitPtr>,
}

impl Pooled for LegacyCraftingOutput_ResourceAmount {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.crafting.legacy_crafting_output_resource_amount
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.crafting.legacy_crafting_output_resource_amount
    }
}

impl<'a> Extract<'a> for LegacyCraftingOutput_ResourceAmount {
    const TYPE_NAME: &'static str = "LegacyCraftingOutput_ResourceAmount";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            resource: inst
                .get("resource")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            amount: match inst.get("amount") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SBaseCargoUnitPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `LegacyCraftingRecipeDef_Direct`
/// Inherits from: `LegacyCraftingRecipeDef_Base`
pub struct LegacyCraftingRecipeDef_Direct {
    /// `recipe` (StrongPointer)
    pub recipe: Option<LegacyCraftingRecipe_BasePtr>,
}

impl Pooled for LegacyCraftingRecipeDef_Direct {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.crafting.legacy_crafting_recipe_def_direct
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.crafting.legacy_crafting_recipe_def_direct
    }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeDef_Direct {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeDef_Direct";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recipe: match inst.get("recipe") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LegacyCraftingRecipe_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `BlueprintCategoryDatabaseRecord`
pub struct BlueprintCategoryDatabaseRecord {
    /// `categories` (Reference (array))
    pub categories: Vec<CigGuid>,
}

impl Pooled for BlueprintCategoryDatabaseRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.crafting.blueprint_category_database_record
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.crafting.blueprint_category_database_record
    }
}

impl<'a> Extract<'a> for BlueprintCategoryDatabaseRecord {
    const TYPE_NAME: &'static str = "BlueprintCategoryDatabaseRecord";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst
                .get_array("categories")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GenericCraftingProcess_Dismantle`
/// Inherits from: `GenericCraftingProcess_Base_NonRef`
pub struct GenericCraftingProcess_Dismantle {
    /// `efficiency` (Single)
    pub efficiency: f32,
    /// `dismantleTime` (StrongPointer)
    pub dismantle_time: Option<TimeValue_BasePtr>,
}

impl Pooled for GenericCraftingProcess_Dismantle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.crafting.generic_crafting_process_dismantle
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.crafting.generic_crafting_process_dismantle
    }
}

impl<'a> Extract<'a> for GenericCraftingProcess_Dismantle {
    const TYPE_NAME: &'static str = "GenericCraftingProcess_Dismantle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            efficiency: inst.get_f32("efficiency").unwrap_or_default(),
            dismantle_time: match inst.get("dismantleTime") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(TimeValue_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `GenericCraftingBlueprint`
/// Inherits from: `CraftingBlueprint_Base_NonRef`
pub struct GenericCraftingBlueprint {
    /// `processSpecificData` (StrongPointer)
    pub process_specific_data: Option<GenericCraftingProcess_BasePtr>,
}

impl Pooled for GenericCraftingBlueprint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.crafting.generic_crafting_blueprint
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.crafting.generic_crafting_blueprint
    }
}

impl<'a> Extract<'a> for GenericCraftingBlueprint {
    const TYPE_NAME: &'static str = "GenericCraftingBlueprint";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            process_specific_data: match inst.get("processSpecificData") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(GenericCraftingProcess_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultBlueprintSelection_Whitelist`
/// Inherits from: `DefaultBlueprintSelection_Base_NonRef`
pub struct DefaultBlueprintSelection_Whitelist {
    /// `blueprintRecords` (Reference (array))
    pub blueprint_records: Vec<CigGuid>,
}

impl Pooled for DefaultBlueprintSelection_Whitelist {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.crafting.default_blueprint_selection_whitelist
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.crafting.default_blueprint_selection_whitelist
    }
}

impl<'a> Extract<'a> for DefaultBlueprintSelection_Whitelist {
    const TYPE_NAME: &'static str = "DefaultBlueprintSelection_Whitelist";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blueprint_records: inst
                .get_array("blueprintRecords")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CraftingGlobalParams`
pub struct CraftingGlobalParams {
    /// `refiningQualityUnitMultiplier` (Single)
    pub refining_quality_unit_multiplier: f32,
    /// `defaultCompositionQuality` (Int32)
    pub default_composition_quality: i32,
    /// `dismantleBlacklistResources` (Reference (array))
    pub dismantle_blacklist_resources: Vec<CigGuid>,
    /// `dismantleBlacklistEntityClasses` (Reference (array))
    pub dismantle_blacklist_entity_classes: Vec<CigGuid>,
    /// `defaultBlueprintSelection` (StrongPointer)
    pub default_blueprint_selection: Option<DefaultBlueprintSelection_BasePtr>,
}

impl Pooled for CraftingGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.crafting.crafting_global_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.crafting.crafting_global_params
    }
}

impl<'a> Extract<'a> for CraftingGlobalParams {
    const TYPE_NAME: &'static str = "CraftingGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            refining_quality_unit_multiplier: inst
                .get_f32("refiningQualityUnitMultiplier")
                .unwrap_or_default(),
            default_composition_quality: inst
                .get_i32("defaultCompositionQuality")
                .unwrap_or_default(),
            dismantle_blacklist_resources: inst
                .get_array("dismantleBlacklistResources")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            dismantle_blacklist_entity_classes: inst
                .get_array("dismantleBlacklistEntityClasses")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_blueprint_selection: match inst.get("defaultBlueprintSelection") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(DefaultBlueprintSelection_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}
