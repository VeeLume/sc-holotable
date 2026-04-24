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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `HarvestableTagListTagEditor`
/// Inherits from: `HarvestableTagListBase`
pub struct HarvestableTagListTagEditor {
    /// `tags` (Reference (array))
    pub tags: Vec<CigGuid>,
}

impl Pooled for HarvestableTagListTagEditor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_tag_list_tag_editor
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_tag_list_tag_editor
    }
}

impl<'a> Extract<'a> for HarvestableTagListTagEditor {
    const TYPE_NAME: &'static str = "HarvestableTagListTagEditor";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst
                .get_array("tags")
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

/// DCB type: `SubHarvestableMultiConfigRecord`
pub struct SubHarvestableMultiConfigRecord {
    /// `multiConfig` (Class)
    pub multi_config: Option<Handle<SubHarvestableMultiConfig>>,
}

impl Pooled for SubHarvestableMultiConfigRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.sub_harvestable_multi_config_record
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.sub_harvestable_multi_config_record
    }
}

impl<'a> Extract<'a> for SubHarvestableMultiConfigRecord {
    const TYPE_NAME: &'static str = "SubHarvestableMultiConfigRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            multi_config: match inst.get("multiConfig") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SubHarvestableMultiConfig>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SubHarvestableConfigSingleRef`
/// Inherits from: `SubHarvestableConfigSingleBase`
pub struct SubHarvestableConfigSingleRef {
    /// `subConfigRef` (Reference)
    pub sub_config_ref: Option<CigGuid>,
}

impl Pooled for SubHarvestableConfigSingleRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.sub_harvestable_config_single_ref
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.sub_harvestable_config_single_ref
    }
}

impl<'a> Extract<'a> for SubHarvestableConfigSingleRef {
    const TYPE_NAME: &'static str = "SubHarvestableConfigSingleRef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sub_config_ref: inst
                .get("subConfigRef")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `HarvestConditionDamageMap`
/// Inherits from: `HarvestConditionBase`
pub struct HarvestConditionDamageMap {
    /// `damageRatio` (Single)
    pub damage_ratio: f32,
}

impl Pooled for HarvestConditionDamageMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvest_condition_damage_map
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvest_condition_damage_map
    }
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
pub struct HarvestableSetup {
    /// `harvestBehaviour` (Class)
    pub harvest_behaviour: Option<Handle<HarvestBehaviourParams>>,
    /// `transformParams` (Class)
    pub transform_params: Option<Handle<HarvestableTransformParams>>,
    /// `subConfigBase` (StrongPointer)
    pub sub_config_base: Option<SubHarvestableConfigBasePtr>,
    /// `respawnInSlotTime` (Single)
    pub respawn_in_slot_time: f32,
    /// `specialHarvestableString` (String)
    pub special_harvestable_string: String,
}

impl Pooled for HarvestableSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_setup
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_setup
    }
}

impl<'a> Extract<'a> for HarvestableSetup {
    const TYPE_NAME: &'static str = "HarvestableSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvest_behaviour: match inst.get("harvestBehaviour") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<HarvestBehaviourParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            transform_params: match inst.get("transformParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<HarvestableTransformParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            sub_config_base: match inst.get("subConfigBase") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SubHarvestableConfigBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            respawn_in_slot_time: inst.get_f32("respawnInSlotTime").unwrap_or_default(),
            special_harvestable_string: inst
                .get_str("specialHarvestableString")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableClusterParams`
pub struct HarvestableClusterParams {
    /// `relativeProbability` (Single)
    pub relative_probability: f32,
    /// `minSize` (Int32)
    pub min_size: i32,
    /// `maxSize` (Int32)
    pub max_size: i32,
    /// `minProximity` (Single)
    pub min_proximity: f32,
    /// `maxProximity` (Single)
    pub max_proximity: f32,
}

impl Pooled for HarvestableClusterParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_cluster_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_cluster_params
    }
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
pub struct HarvestableClusterPreset {
    /// `probabilityOfClustering` (Single)
    pub probability_of_clustering: f32,
    /// `clusterParamsArray` (Class (array))
    pub cluster_params_array: Vec<Handle<HarvestableClusterParams>>,
}

impl Pooled for HarvestableClusterPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_cluster_preset
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_cluster_preset
    }
}

impl<'a> Extract<'a> for HarvestableClusterPreset {
    const TYPE_NAME: &'static str = "HarvestableClusterPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            probability_of_clustering: inst.get_f32("probabilityOfClustering").unwrap_or_default(),
            cluster_params_array: inst
                .get_array("clusterParamsArray")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HarvestableClusterParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableClusterParams>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElement`
pub struct HarvestableElement {
    /// `harvestable` (Reference)
    pub harvestable: Option<CigGuid>,
    /// `harvestableEntityClass` (Reference)
    pub harvestable_entity_class: Option<CigGuid>,
    /// `harvestableSetup` (Reference)
    pub harvestable_setup: Option<CigGuid>,
    /// `relativeProbability` (Single)
    pub relative_probability: f32,
    /// `clustering` (Reference)
    pub clustering: Option<CigGuid>,
    /// `geometries` (Class (array))
    pub geometries: Vec<Handle<HarvestableGeometry>>,
}

impl Pooled for HarvestableElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_element
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_element
    }
}

impl<'a> Extract<'a> for HarvestableElement {
    const TYPE_NAME: &'static str = "HarvestableElement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable: inst
                .get("harvestable")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            harvestable_entity_class: inst
                .get("harvestableEntityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            harvestable_setup: inst
                .get("harvestableSetup")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            relative_probability: inst.get_f32("relativeProbability").unwrap_or_default(),
            clustering: inst
                .get("clustering")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            geometries: inst
                .get_array("geometries")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HarvestableGeometry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableGeometry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElementGroup`
pub struct HarvestableElementGroup {
    /// `groupName` (String)
    pub group_name: String,
    /// `groupProbability` (Single)
    pub group_probability: f32,
    /// `harvestables` (Class (array))
    pub harvestables: Vec<Handle<HarvestableElement>>,
}

impl Pooled for HarvestableElementGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_element_group
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_element_group
    }
}

impl<'a> Extract<'a> for HarvestableElementGroup {
    const TYPE_NAME: &'static str = "HarvestableElementGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            group_name: inst
                .get_str("groupName")
                .map(String::from)
                .unwrap_or_default(),
            group_probability: inst.get_f32("groupProbability").unwrap_or_default(),
            harvestables: inst
                .get_array("harvestables")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HarvestableElement>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableElement>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElementModifier`
pub struct HarvestableElementModifier {
    /// `harvestableElement` (WeakPointer)
    pub harvestable_element: Option<Handle<HarvestableElement>>,
    /// `harvestableModifier` (Single)
    pub harvestable_modifier: f32,
    /// `geometries` (Class (array))
    pub geometries: Vec<Handle<HarvestableGeometry>>,
}

impl Pooled for HarvestableElementModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_element_modifier
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_element_modifier
    }
}

impl<'a> Extract<'a> for HarvestableElementModifier {
    const TYPE_NAME: &'static str = "HarvestableElementModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable_element: match inst.get("harvestableElement") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<HarvestableElement>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            harvestable_modifier: inst.get_f32("harvestableModifier").unwrap_or_default(),
            geometries: inst
                .get_array("geometries")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HarvestableGeometry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableGeometry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableAreaTypeObjectPreset`
/// Inherits from: `HarvestableAreaTypeBase`
pub struct HarvestableAreaTypeObjectPreset {
    /// `objectPresetPaths` (String (array))
    pub object_preset_paths: Vec<String>,
}

impl Pooled for HarvestableAreaTypeObjectPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_area_type_object_preset
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_area_type_object_preset
    }
}

impl<'a> Extract<'a> for HarvestableAreaTypeObjectPreset {
    const TYPE_NAME: &'static str = "HarvestableAreaTypeObjectPreset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            object_preset_paths: inst
                .get_array("objectPresetPaths")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableAreaPreset`
pub struct HarvestableAreaPreset {
    /// `debugGroupName` (String)
    pub debug_group_name: String,
    /// `areaType` (StrongPointer)
    pub area_type: Option<HarvestableAreaTypeBasePtr>,
    /// `globalModifier` (Single)
    pub global_modifier: f32,
    /// `modifiers` (Class (array))
    pub modifiers: Vec<Handle<HarvestableElementModifier>>,
}

impl Pooled for HarvestableAreaPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_area_preset
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_area_preset
    }
}

impl<'a> Extract<'a> for HarvestableAreaPreset {
    const TYPE_NAME: &'static str = "HarvestableAreaPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_group_name: inst
                .get_str("debugGroupName")
                .map(String::from)
                .unwrap_or_default(),
            area_type: match inst.get("areaType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(HarvestableAreaTypeBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            global_modifier: inst.get_f32("globalModifier").unwrap_or_default(),
            modifiers: inst
                .get_array("modifiers")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HarvestableElementModifier>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableElementModifier>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableProviderPreset`
pub struct HarvestableProviderPreset {
    /// `harvestableGroups` (Class (array))
    pub harvestable_groups: Vec<Handle<HarvestableElementGroup>>,
    /// `areas` (Class (array))
    pub areas: Vec<Handle<HarvestableAreaPreset>>,
}

impl Pooled for HarvestableProviderPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.harvestable.harvestable_provider_preset
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.harvestable.harvestable_provider_preset
    }
}

impl<'a> Extract<'a> for HarvestableProviderPreset {
    const TYPE_NAME: &'static str = "HarvestableProviderPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable_groups: inst
                .get_array("harvestableGroups")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HarvestableElementGroup>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableElementGroup>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            areas: inst
                .get_array("areas")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HarvestableAreaPreset>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HarvestableAreaPreset>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
