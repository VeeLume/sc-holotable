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

/// DCB type: `CraftingGameplayPropertyDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingGameplayPropertyDef {
    /// DCB field: `propertyName` (Locale)
    #[serde(default)]
    pub property_name: String,
    /// DCB field: `unitFormat` (Locale)
    #[serde(default)]
    pub unit_format: String,
}

impl Pooled for CraftingGameplayPropertyDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_gameplay_property_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_gameplay_property_def }
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

/// DCB type: `CraftingBlueprint_Base_NonRef`
///
/// Inherits from: `CraftingBlueprint_Base` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingBlueprint_Base_NonRef {
}

impl Pooled for CraftingBlueprint_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_blueprint_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_blueprint_base_non_ref }
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
    /// DCB field: `blueprint` (StrongPointer)
    #[serde(default)]
    pub blueprint: Option<Handle<CraftingBlueprint_Base_NonRef>>,
}

impl Pooled for CraftingBlueprintRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_blueprint_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_blueprint_record }
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

/// DCB type: `CraftingQualityDistribution_Base_NonRef`
///
/// Inherits from: `CraftingQualityDistribution_Base` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityDistribution_Base_NonRef {
}

impl Pooled for CraftingQualityDistribution_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_quality_distribution_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_quality_distribution_base_non_ref }
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
    /// DCB field: `qualityDistribution` (StrongPointer)
    #[serde(default)]
    pub quality_distribution: Option<Handle<CraftingQualityDistribution_Base_NonRef>>,
}

impl Pooled for CraftingQualityDistributionRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_quality_distribution_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_quality_distribution_record }
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
///
/// Inherits from: `CraftingQualityLocationOverride_Base` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityLocationOverride_Base_NonRef {
}

impl Pooled for CraftingQualityLocationOverride_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_quality_location_override_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_quality_location_override_base_non_ref }
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
    /// DCB field: `locationOverride` (StrongPointer)
    #[serde(default)]
    pub location_override: Option<Handle<CraftingQualityLocationOverride_Base_NonRef>>,
}

impl Pooled for CraftingQualityLocationOverrideRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_quality_location_override_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_quality_location_override_record }
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
    /// DCB field: `refiningQualityUnitMultiplier` (Single)
    #[serde(default)]
    pub refining_quality_unit_multiplier: f32,
    /// DCB field: `defaultCompositionQuality` (Int32)
    #[serde(default)]
    pub default_composition_quality: i32,
    /// DCB field: `dismantleBlacklistResources` (Reference (array))
    #[serde(default)]
    pub dismantle_blacklist_resources: Vec<CigGuid>,
    /// DCB field: `dismantleBlacklistEntityClasses` (Reference (array))
    #[serde(default)]
    pub dismantle_blacklist_entity_classes: Vec<CigGuid>,
    /// DCB field: `defaultBlueprintSelection` (StrongPointer)
    #[serde(default)]
    pub default_blueprint_selection: Option<Handle<DefaultBlueprintSelection_Base>>,
}

impl Pooled for CraftingGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crafting_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crafting_global_params }
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

/// DCB type: `CrewManifest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewManifest {
    /// DCB field: `descriptionTags` (Class)
    #[serde(default)]
    pub description_tags: Option<Handle<TagList>>,
    /// DCB field: `crew` (Class (array))
    #[serde(default)]
    pub crew: Vec<Handle<CrewData>>,
}

impl Pooled for CrewManifest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crew_manifest }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crew_manifest }
}

impl<'a> Extract<'a> for CrewManifest {
    const TYPE_NAME: &'static str = "CrewManifest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description_tags: match inst.get("descriptionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            crew: inst.get_array("crew")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CrewData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CrewData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CrewData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewData {
    /// DCB field: `entityClassDefinition` (Reference)
    #[serde(default)]
    pub entity_class_definition: Option<CigGuid>,
    /// DCB field: `outfitTags` (Reference (array))
    #[serde(default)]
    pub outfit_tags: Vec<CigGuid>,
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// DCB field: `dnfTags` (Class)
    #[serde(default)]
    pub dnf_tags: Option<Handle<TagsDNF>>,
    /// DCB field: `archetype` (Reference)
    #[serde(default)]
    pub archetype: Option<CigGuid>,
}

impl Pooled for CrewData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.crew_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.crew_data }
}

impl<'a> Extract<'a> for CrewData {
    const TYPE_NAME: &'static str = "CrewData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_class_definition: inst.get("entityClassDefinition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            outfit_tags: inst.get_array("outfitTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            dnf_tags: match inst.get("dnfTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            archetype: inst.get("archetype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `CrossSectionGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSectionGlobalParams {
    /// DCB field: `globalCSFactor` (Class)
    #[serde(default)]
    pub global_csfactor: Option<Handle<Vec3>>,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// DCB field: `lineOfSightAngle` (Single)
    #[serde(default)]
    pub line_of_sight_angle: f32,
}

impl Pooled for CrossSectionGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cr.cross_section_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cr.cross_section_global_params }
}

impl<'a> Extract<'a> for CrossSectionGlobalParams {
    const TYPE_NAME: &'static str = "CrossSectionGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_csfactor: match inst.get("globalCSFactor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            line_of_sight_angle: inst.get_f32("lineOfSightAngle").unwrap_or_default(),
        }
    }
}

