// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `harvestable`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `HarvestableTagListTagEditor`
/// Inherits from: `HarvestableTagListBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableTagListTagEditor {
    /// `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for HarvestableTagListTagEditor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_tag_list_tag_editor }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_tag_list_tag_editor }
}

impl<'a> Extract<'a> for HarvestableTagListTagEditor {
    const TYPE_NAME: &'static str = "HarvestableTagListTagEditor";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SubHarvestableMultiConfigRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableMultiConfigRecord {
    /// `multiConfig` (Class)
    #[serde(default)]
    pub multi_config: Option<Handle<SubHarvestableMultiConfig>>,
}

impl Pooled for SubHarvestableMultiConfigRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_multi_config_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_multi_config_record }
}

impl<'a> Extract<'a> for SubHarvestableMultiConfigRecord {
    const TYPE_NAME: &'static str = "SubHarvestableMultiConfigRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            multi_config: match inst.get("multiConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SubHarvestableMultiConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SubHarvestableConfigSingleRef`
/// Inherits from: `SubHarvestableConfigSingleBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfigSingleRef {
    /// `subConfigRef` (Reference)
    #[serde(default)]
    pub sub_config_ref: Option<CigGuid>,
}

impl Pooled for SubHarvestableConfigSingleRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_config_single_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_config_single_ref }
}

impl<'a> Extract<'a> for SubHarvestableConfigSingleRef {
    const TYPE_NAME: &'static str = "SubHarvestableConfigSingleRef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sub_config_ref: inst.get("subConfigRef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `HarvestConditionDamageMap`
/// Inherits from: `HarvestConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestConditionDamageMap {
    /// `damageRatio` (Single)
    #[serde(default)]
    pub damage_ratio: f32,
}

impl Pooled for HarvestConditionDamageMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvest_condition_damage_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvest_condition_damage_map }
}

impl<'a> Extract<'a> for HarvestConditionDamageMap {
    const TYPE_NAME: &'static str = "HarvestConditionDamageMap";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_ratio: inst.get_f32("damageRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableSetup {
    /// `harvestBehaviour` (Class)
    #[serde(default)]
    pub harvest_behaviour: Option<Handle<HarvestBehaviourParams>>,
    /// `transformParams` (Class)
    #[serde(default)]
    pub transform_params: Option<Handle<HarvestableTransformParams>>,
    /// `subConfigBase` (StrongPointer)
    #[serde(default)]
    pub sub_config_base: Option<SubHarvestableConfigBasePtr>,
    /// `respawnInSlotTime` (Single)
    #[serde(default)]
    pub respawn_in_slot_time: f32,
    /// `specialHarvestableString` (String)
    #[serde(default)]
    pub special_harvestable_string: String,
}

impl Pooled for HarvestableSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_setup }
}

impl<'a> Extract<'a> for HarvestableSetup {
    const TYPE_NAME: &'static str = "HarvestableSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvest_behaviour: match inst.get("harvestBehaviour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestBehaviourParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            transform_params: match inst.get("transformParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestableTransformParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sub_config_base: match inst.get("subConfigBase") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SubHarvestableConfigBasePtr::from_ref(b, r)),
                _ => None,
            },
            respawn_in_slot_time: inst.get_f32("respawnInSlotTime").unwrap_or_default(),
            special_harvestable_string: inst.get_str("specialHarvestableString").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableClusterParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableClusterParams {
    /// `relativeProbability` (Single)
    #[serde(default)]
    pub relative_probability: f32,
    /// `minSize` (Int32)
    #[serde(default)]
    pub min_size: i32,
    /// `maxSize` (Int32)
    #[serde(default)]
    pub max_size: i32,
    /// `minProximity` (Single)
    #[serde(default)]
    pub min_proximity: f32,
    /// `maxProximity` (Single)
    #[serde(default)]
    pub max_proximity: f32,
}

impl Pooled for HarvestableClusterParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_cluster_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_cluster_params }
}

impl<'a> Extract<'a> for HarvestableClusterParams {
    const TYPE_NAME: &'static str = "HarvestableClusterParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            relative_probability: inst.get_f32("relativeProbability").unwrap_or_default(),
            min_size: inst.get_i32("minSize").unwrap_or_default(),
            max_size: inst.get_i32("maxSize").unwrap_or_default(),
            min_proximity: inst.get_f32("minProximity").unwrap_or_default(),
            max_proximity: inst.get_f32("maxProximity").unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableClusterPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableClusterPreset {
    /// `probabilityOfClustering` (Single)
    #[serde(default)]
    pub probability_of_clustering: f32,
    /// `clusterParamsArray` (Class (array))
    #[serde(default)]
    pub cluster_params_array: Vec<Handle<HarvestableClusterParams>>,
}

impl Pooled for HarvestableClusterPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_cluster_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_cluster_preset }
}

impl<'a> Extract<'a> for HarvestableClusterPreset {
    const TYPE_NAME: &'static str = "HarvestableClusterPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            probability_of_clustering: inst.get_f32("probabilityOfClustering").unwrap_or_default(),
            cluster_params_array: inst.get_array("clusterParamsArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableClusterParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableClusterParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableElement {
    /// `harvestable` (Reference)
    #[serde(default)]
    pub harvestable: Option<CigGuid>,
    /// `harvestableEntityClass` (Reference)
    #[serde(default)]
    pub harvestable_entity_class: Option<CigGuid>,
    /// `harvestableSetup` (Reference)
    #[serde(default)]
    pub harvestable_setup: Option<CigGuid>,
    /// `relativeProbability` (Single)
    #[serde(default)]
    pub relative_probability: f32,
    /// `clustering` (Reference)
    #[serde(default)]
    pub clustering: Option<CigGuid>,
    /// `geometries` (Class (array))
    #[serde(default)]
    pub geometries: Vec<Handle<HarvestableGeometry>>,
}

impl Pooled for HarvestableElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_element }
}

impl<'a> Extract<'a> for HarvestableElement {
    const TYPE_NAME: &'static str = "HarvestableElement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable: inst.get("harvestable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            harvestable_entity_class: inst.get("harvestableEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            harvestable_setup: inst.get("harvestableSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            relative_probability: inst.get_f32("relativeProbability").unwrap_or_default(),
            clustering: inst.get("clustering").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            geometries: inst.get_array("geometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElementGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableElementGroup {
    /// `groupName` (String)
    #[serde(default)]
    pub group_name: String,
    /// `groupProbability` (Single)
    #[serde(default)]
    pub group_probability: f32,
    /// `harvestables` (Class (array))
    #[serde(default)]
    pub harvestables: Vec<Handle<HarvestableElement>>,
}

impl Pooled for HarvestableElementGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_element_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_element_group }
}

impl<'a> Extract<'a> for HarvestableElementGroup {
    const TYPE_NAME: &'static str = "HarvestableElementGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            group_name: inst.get_str("groupName").map(String::from).unwrap_or_default(),
            group_probability: inst.get_f32("groupProbability").unwrap_or_default(),
            harvestables: inst.get_array("harvestables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableElement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElementModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableElementModifier {
    /// `harvestableElement` (WeakPointer)
    #[serde(default)]
    pub harvestable_element: Option<Handle<HarvestableElement>>,
    /// `harvestableModifier` (Single)
    #[serde(default)]
    pub harvestable_modifier: f32,
    /// `geometries` (Class (array))
    #[serde(default)]
    pub geometries: Vec<Handle<HarvestableGeometry>>,
}

impl Pooled for HarvestableElementModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_element_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_element_modifier }
}

impl<'a> Extract<'a> for HarvestableElementModifier {
    const TYPE_NAME: &'static str = "HarvestableElementModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable_element: match inst.get("harvestableElement") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestableElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            harvestable_modifier: inst.get_f32("harvestableModifier").unwrap_or_default(),
            geometries: inst.get_array("geometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableAreaTypeObjectPreset`
/// Inherits from: `HarvestableAreaTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableAreaTypeObjectPreset {
    /// `objectPresetPaths` (String (array))
    #[serde(default)]
    pub object_preset_paths: Vec<String>,
}

impl Pooled for HarvestableAreaTypeObjectPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_area_type_object_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_area_type_object_preset }
}

impl<'a> Extract<'a> for HarvestableAreaTypeObjectPreset {
    const TYPE_NAME: &'static str = "HarvestableAreaTypeObjectPreset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            object_preset_paths: inst.get_array("objectPresetPaths")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableAreaPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableAreaPreset {
    /// `debugGroupName` (String)
    #[serde(default)]
    pub debug_group_name: String,
    /// `areaType` (StrongPointer)
    #[serde(default)]
    pub area_type: Option<HarvestableAreaTypeBasePtr>,
    /// `globalModifier` (Single)
    #[serde(default)]
    pub global_modifier: f32,
    /// `modifiers` (Class (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<HarvestableElementModifier>>,
}

impl Pooled for HarvestableAreaPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_area_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_area_preset }
}

impl<'a> Extract<'a> for HarvestableAreaPreset {
    const TYPE_NAME: &'static str = "HarvestableAreaPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_group_name: inst.get_str("debugGroupName").map(String::from).unwrap_or_default(),
            area_type: match inst.get("areaType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(HarvestableAreaTypeBasePtr::from_ref(b, r)),
                _ => None,
            },
            global_modifier: inst.get_f32("globalModifier").unwrap_or_default(),
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableElementModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableElementModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableProviderPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableProviderPreset {
    /// `harvestableGroups` (Class (array))
    #[serde(default)]
    pub harvestable_groups: Vec<Handle<HarvestableElementGroup>>,
    /// `areas` (Class (array))
    #[serde(default)]
    pub areas: Vec<Handle<HarvestableAreaPreset>>,
}

impl Pooled for HarvestableProviderPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_provider_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_provider_preset }
}

impl<'a> Extract<'a> for HarvestableProviderPreset {
    const TYPE_NAME: &'static str = "HarvestableProviderPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable_groups: inst.get_array("harvestableGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableElementGroup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableElementGroup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            areas: inst.get_array("areas")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableAreaPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableAreaPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

