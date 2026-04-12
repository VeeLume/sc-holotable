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

/// DCB type: `HandholdGripType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandholdGripType {
    /// DCB field: `mannequinTag` (String)
    #[serde(default)]
    pub mannequin_tag: String,
    /// DCB field: `handholdType` (EnumChoice)
    #[serde(default)]
    pub handhold_type: String,
}

impl Pooled for HandholdGripType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.handhold_grip_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.handhold_grip_type }
}

impl<'a> Extract<'a> for HandholdGripType {
    const TYPE_NAME: &'static str = "HandholdGripType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mannequin_tag: inst.get_str("mannequinTag").map(String::from).unwrap_or_default(),
            handhold_type: inst.get_str("handholdType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HandholdGripDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandholdGripDatabase {
    /// DCB field: `gripTypes` (Reference (array))
    #[serde(default)]
    pub grip_types: Vec<CigGuid>,
}

impl Pooled for HandholdGripDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.handhold_grip_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.handhold_grip_database }
}

impl<'a> Extract<'a> for HandholdGripDatabase {
    const TYPE_NAME: &'static str = "HandholdGripDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grip_types: inst.get_array("gripTypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableTagListBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableTagListBase {
}

impl Pooled for HarvestableTagListBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_tag_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_tag_list_base }
}

impl<'a> Extract<'a> for HarvestableTagListBase {
    const TYPE_NAME: &'static str = "HarvestableTagListBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HarvestableTransformParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableTransformParams {
    /// DCB field: `localRotationOffset` (Class)
    #[serde(default)]
    pub local_rotation_offset: Option<Handle<Vec3>>,
    /// DCB field: `minScale` (Single)
    #[serde(default)]
    pub min_scale: f32,
    /// DCB field: `maxScale` (Single)
    #[serde(default)]
    pub max_scale: f32,
    /// DCB field: `terrainNormalAlignment` (Single)
    #[serde(default)]
    pub terrain_normal_alignment: f32,
    /// DCB field: `rotationRange` (Class)
    #[serde(default)]
    pub rotation_range: Option<Handle<Vec3>>,
    /// DCB field: `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
    /// DCB field: `minZOffset` (Single)
    #[serde(default)]
    pub min_zoffset: f32,
    /// DCB field: `maxZOffset` (Single)
    #[serde(default)]
    pub max_zoffset: f32,
    /// DCB field: `minSlope` (Single)
    #[serde(default)]
    pub min_slope: f32,
    /// DCB field: `maxSlope` (Single)
    #[serde(default)]
    pub max_slope: f32,
    /// DCB field: `minElevation` (Single)
    #[serde(default)]
    pub min_elevation: f32,
    /// DCB field: `maxElevation` (Single)
    #[serde(default)]
    pub max_elevation: f32,
}

impl Pooled for HarvestableTransformParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_transform_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_transform_params }
}

impl<'a> Extract<'a> for HarvestableTransformParams {
    const TYPE_NAME: &'static str = "HarvestableTransformParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            local_rotation_offset: match inst.get("localRotationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_scale: inst.get_f32("minScale").unwrap_or_default(),
            max_scale: inst.get_f32("maxScale").unwrap_or_default(),
            terrain_normal_alignment: inst.get_f32("terrainNormalAlignment").unwrap_or_default(),
            rotation_range: match inst.get("rotationRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_zoffset: inst.get_f32("minZOffset").unwrap_or_default(),
            max_zoffset: inst.get_f32("maxZOffset").unwrap_or_default(),
            min_slope: inst.get_f32("minSlope").unwrap_or_default(),
            max_slope: inst.get_f32("maxSlope").unwrap_or_default(),
            min_elevation: inst.get_f32("minElevation").unwrap_or_default(),
            max_elevation: inst.get_f32("maxElevation").unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestConditionBase {
}

impl Pooled for HarvestConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvest_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvest_condition_base }
}

impl<'a> Extract<'a> for HarvestConditionBase {
    const TYPE_NAME: &'static str = "HarvestConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HarvestDespawnTimerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestDespawnTimerParams {
    /// DCB field: `despawnTimeSeconds` (Int32)
    #[serde(default)]
    pub despawn_time_seconds: i32,
    /// DCB field: `additionalWaitForNearbyPlayersSeconds` (Int32)
    #[serde(default)]
    pub additional_wait_for_nearby_players_seconds: i32,
}

impl Pooled for HarvestDespawnTimerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvest_despawn_timer_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvest_despawn_timer_params }
}

impl<'a> Extract<'a> for HarvestDespawnTimerParams {
    const TYPE_NAME: &'static str = "HarvestDespawnTimerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            despawn_time_seconds: inst.get_i32("despawnTimeSeconds").unwrap_or_default(),
            additional_wait_for_nearby_players_seconds: inst.get_i32("additionalWaitForNearbyPlayersSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestBehaviourParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestBehaviourParams {
    /// DCB field: `harvestConditions` (StrongPointer (array))
    #[serde(default)]
    pub harvest_conditions: Vec<Handle<HarvestConditionBase>>,
    /// DCB field: `despawnTimer` (Class)
    #[serde(default)]
    pub despawn_timer: Option<Handle<HarvestDespawnTimerParams>>,
}

impl Pooled for HarvestBehaviourParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvest_behaviour_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvest_behaviour_params }
}

impl<'a> Extract<'a> for HarvestBehaviourParams {
    const TYPE_NAME: &'static str = "HarvestBehaviourParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvest_conditions: inst.get_array("harvestConditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestConditionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestConditionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            despawn_timer: match inst.get("despawnTimer") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestDespawnTimerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestDespawnTimerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HarvestablePreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestablePreset {
    /// DCB field: `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
    /// DCB field: `harvestBehaviour` (Class)
    #[serde(default)]
    pub harvest_behaviour: Option<Handle<HarvestBehaviourParams>>,
    /// DCB field: `transformParams` (Class)
    #[serde(default)]
    pub transform_params: Option<Handle<HarvestableTransformParams>>,
    /// DCB field: `subConfigBase` (StrongPointer)
    #[serde(default)]
    pub sub_config_base: Option<Handle<SubHarvestableConfigBase>>,
    /// DCB field: `respawnInSlotTime` (Single)
    #[serde(default)]
    pub respawn_in_slot_time: f32,
    /// DCB field: `specialHarvestableString` (String)
    #[serde(default)]
    pub special_harvestable_string: String,
}

impl Pooled for HarvestablePreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_preset }
}

impl<'a> Extract<'a> for HarvestablePreset {
    const TYPE_NAME: &'static str = "HarvestablePreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            harvest_behaviour: match inst.get("harvestBehaviour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestBehaviourParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestBehaviourParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            transform_params: match inst.get("transformParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestableTransformParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestableTransformParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sub_config_base: match inst.get("subConfigBase") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SubHarvestableConfigBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SubHarvestableConfigBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            respawn_in_slot_time: inst.get_f32("respawnInSlotTime").unwrap_or_default(),
            special_harvestable_string: inst.get_str("specialHarvestableString").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableSetup {
    /// DCB field: `harvestBehaviour` (Class)
    #[serde(default)]
    pub harvest_behaviour: Option<Handle<HarvestBehaviourParams>>,
    /// DCB field: `transformParams` (Class)
    #[serde(default)]
    pub transform_params: Option<Handle<HarvestableTransformParams>>,
    /// DCB field: `subConfigBase` (StrongPointer)
    #[serde(default)]
    pub sub_config_base: Option<Handle<SubHarvestableConfigBase>>,
    /// DCB field: `respawnInSlotTime` (Single)
    #[serde(default)]
    pub respawn_in_slot_time: f32,
    /// DCB field: `specialHarvestableString` (String)
    #[serde(default)]
    pub special_harvestable_string: String,
}

impl Pooled for HarvestableSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_setup }
}

impl<'a> Extract<'a> for HarvestableSetup {
    const TYPE_NAME: &'static str = "HarvestableSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvest_behaviour: match inst.get("harvestBehaviour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestBehaviourParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestBehaviourParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            transform_params: match inst.get("transformParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestableTransformParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestableTransformParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sub_config_base: match inst.get("subConfigBase") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SubHarvestableConfigBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SubHarvestableConfigBase>(b.db.instance(r.struct_index, r.instance_index), true)),
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
    /// DCB field: `relativeProbability` (Single)
    #[serde(default)]
    pub relative_probability: f32,
    /// DCB field: `minSize` (Int32)
    #[serde(default)]
    pub min_size: i32,
    /// DCB field: `maxSize` (Int32)
    #[serde(default)]
    pub max_size: i32,
    /// DCB field: `minProximity` (Single)
    #[serde(default)]
    pub min_proximity: f32,
    /// DCB field: `maxProximity` (Single)
    #[serde(default)]
    pub max_proximity: f32,
}

impl Pooled for HarvestableClusterParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_cluster_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_cluster_params }
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
    /// DCB field: `probabilityOfClustering` (Single)
    #[serde(default)]
    pub probability_of_clustering: f32,
    /// DCB field: `clusterParamsArray` (Class (array))
    #[serde(default)]
    pub cluster_params_array: Vec<Handle<HarvestableClusterParams>>,
}

impl Pooled for HarvestableClusterPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_cluster_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_cluster_preset }
}

impl<'a> Extract<'a> for HarvestableClusterPreset {
    const TYPE_NAME: &'static str = "HarvestableClusterPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            probability_of_clustering: inst.get_f32("probabilityOfClustering").unwrap_or_default(),
            cluster_params_array: inst.get_array("clusterParamsArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableClusterParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableClusterParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableGeometry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableGeometry {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
}

impl Pooled for HarvestableGeometry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_geometry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_geometry }
}

impl<'a> Extract<'a> for HarvestableGeometry {
    const TYPE_NAME: &'static str = "HarvestableGeometry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `HarvestableElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableElement {
    /// DCB field: `harvestable` (Reference)
    #[serde(default)]
    pub harvestable: Option<CigGuid>,
    /// DCB field: `harvestableEntityClass` (Reference)
    #[serde(default)]
    pub harvestable_entity_class: Option<CigGuid>,
    /// DCB field: `harvestableSetup` (Reference)
    #[serde(default)]
    pub harvestable_setup: Option<CigGuid>,
    /// DCB field: `relativeProbability` (Single)
    #[serde(default)]
    pub relative_probability: f32,
    /// DCB field: `clustering` (Reference)
    #[serde(default)]
    pub clustering: Option<CigGuid>,
    /// DCB field: `geometries` (Class (array))
    #[serde(default)]
    pub geometries: Vec<Handle<HarvestableGeometry>>,
}

impl Pooled for HarvestableElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_element }
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
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElementGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableElementGroup {
    /// DCB field: `groupName` (String)
    #[serde(default)]
    pub group_name: String,
    /// DCB field: `groupProbability` (Single)
    #[serde(default)]
    pub group_probability: f32,
    /// DCB field: `harvestables` (Class (array))
    #[serde(default)]
    pub harvestables: Vec<Handle<HarvestableElement>>,
}

impl Pooled for HarvestableElementGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_element_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_element_group }
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
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableElementModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableElementModifier {
    /// DCB field: `harvestableElement` (WeakPointer)
    #[serde(default)]
    pub harvestable_element: Option<Handle<HarvestableElement>>,
    /// DCB field: `harvestableModifier` (Single)
    #[serde(default)]
    pub harvestable_modifier: f32,
    /// DCB field: `geometries` (Class (array))
    #[serde(default)]
    pub geometries: Vec<Handle<HarvestableGeometry>>,
}

impl Pooled for HarvestableElementModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_element_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_element_modifier }
}

impl<'a> Extract<'a> for HarvestableElementModifier {
    const TYPE_NAME: &'static str = "HarvestableElementModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable_element: match inst.get("harvestableElement") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestableElement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestableElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            harvestable_modifier: inst.get_f32("harvestableModifier").unwrap_or_default(),
            geometries: inst.get_array("geometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableAreaTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableAreaTypeBase {
}

impl Pooled for HarvestableAreaTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_area_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_area_type_base }
}

impl<'a> Extract<'a> for HarvestableAreaTypeBase {
    const TYPE_NAME: &'static str = "HarvestableAreaTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HarvestableAreaPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableAreaPreset {
    /// DCB field: `debugGroupName` (String)
    #[serde(default)]
    pub debug_group_name: String,
    /// DCB field: `areaType` (StrongPointer)
    #[serde(default)]
    pub area_type: Option<Handle<HarvestableAreaTypeBase>>,
    /// DCB field: `globalModifier` (Single)
    #[serde(default)]
    pub global_modifier: f32,
    /// DCB field: `modifiers` (Class (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<HarvestableElementModifier>>,
}

impl Pooled for HarvestableAreaPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_area_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_area_preset }
}

impl<'a> Extract<'a> for HarvestableAreaPreset {
    const TYPE_NAME: &'static str = "HarvestableAreaPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_group_name: inst.get_str("debugGroupName").map(String::from).unwrap_or_default(),
            area_type: match inst.get("areaType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestableAreaTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestableAreaTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            global_modifier: inst.get_f32("globalModifier").unwrap_or_default(),
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableElementModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableElementModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableProviderPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableProviderPreset {
    /// DCB field: `harvestableGroups` (Class (array))
    #[serde(default)]
    pub harvestable_groups: Vec<Handle<HarvestableElementGroup>>,
    /// DCB field: `areas` (Class (array))
    #[serde(default)]
    pub areas: Vec<Handle<HarvestableAreaPreset>>,
}

impl Pooled for HarvestableProviderPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.harvestable_provider_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.harvestable_provider_preset }
}

impl<'a> Extract<'a> for HarvestableProviderPreset {
    const TYPE_NAME: &'static str = "HarvestableProviderPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable_groups: inst.get_array("harvestableGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableElementGroup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableElementGroup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            areas: inst.get_array("areas")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableAreaPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableAreaPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HardwareMouseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMouseParams {
    /// DCB field: `cursor` (Class)
    #[serde(default)]
    pub cursor: Option<Handle<VirtualCursorParams>>,
    /// DCB field: `hoverFriction` (Class)
    #[serde(default)]
    pub hover_friction: Option<Handle<VirtualCursorHoverFrictionParams>>,
    /// DCB field: `wheel` (Class)
    #[serde(default)]
    pub wheel: Option<Handle<VirtualCursorWheelParams>>,
    /// DCB field: `enableDPadNavigation` (Boolean)
    #[serde(default)]
    pub enable_dpad_navigation: bool,
}

impl Pooled for HardwareMouseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.hardware_mouse_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.hardware_mouse_params }
}

impl<'a> Extract<'a> for HardwareMouseParams {
    const TYPE_NAME: &'static str = "HardwareMouseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cursor: match inst.get("cursor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VirtualCursorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hover_friction: match inst.get("hoverFriction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorHoverFrictionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VirtualCursorHoverFrictionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wheel: match inst.get("wheel") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorWheelParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VirtualCursorWheelParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_dpad_navigation: inst.get_bool("enableDPadNavigation").unwrap_or_default(),
        }
    }
}

/// DCB type: `Hauling_EntityClassListBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hauling_EntityClassListBase {
}

impl Pooled for Hauling_EntityClassListBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.hauling_entity_class_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.hauling_entity_class_list_base }
}

impl<'a> Extract<'a> for Hauling_EntityClassListBase {
    const TYPE_NAME: &'static str = "Hauling_EntityClassListBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Hauling_EntityClasses`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hauling_EntityClasses {
    /// DCB field: `entityClasses` (StrongPointer)
    #[serde(default)]
    pub entity_classes: Option<Handle<Hauling_EntityClassListBase>>,
    /// DCB field: `orderDisplayName` (Locale)
    #[serde(default)]
    pub order_display_name: String,
}

impl Pooled for Hauling_EntityClasses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.hauling_entity_classes }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.hauling_entity_classes }
}

impl<'a> Extract<'a> for Hauling_EntityClasses {
    const TYPE_NAME: &'static str = "Hauling_EntityClasses";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_classes: match inst.get("entityClasses") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Hauling_EntityClassListBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Hauling_EntityClassListBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            order_display_name: inst.get_str("orderDisplayName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HazardAwarenessParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HazardAwarenessParams {
    /// DCB field: `windHazardMinSpeed` (Single)
    #[serde(default)]
    pub wind_hazard_min_speed: f32,
    /// DCB field: `fireHandRaiseDistance` (Single)
    #[serde(default)]
    pub fire_hand_raise_distance: f32,
    /// DCB field: `handRaiseYawMin` (Single)
    #[serde(default)]
    pub hand_raise_yaw_min: f32,
    /// DCB field: `handRaiseYawMax` (Single)
    #[serde(default)]
    pub hand_raise_yaw_max: f32,
    /// DCB field: `handRaiseYawHysteresis` (Single)
    #[serde(default)]
    pub hand_raise_yaw_hysteresis: f32,
    /// DCB field: `handFollowYawMin` (Single)
    #[serde(default)]
    pub hand_follow_yaw_min: f32,
    /// DCB field: `handFollowYawMax` (Single)
    #[serde(default)]
    pub hand_follow_yaw_max: f32,
    /// DCB field: `handFollowYawSmoothing` (Single)
    #[serde(default)]
    pub hand_follow_yaw_smoothing: f32,
    /// DCB field: `handPitchOffset` (Single)
    #[serde(default)]
    pub hand_pitch_offset: f32,
    /// DCB field: `handIKDistanceOffset` (Single)
    #[serde(default)]
    pub hand_ikdistance_offset: f32,
    /// DCB field: `handIKStrength` (Single)
    #[serde(default)]
    pub hand_ikstrength: f32,
    /// DCB field: `handResumeDelay` (Single)
    #[serde(default)]
    pub hand_resume_delay: f32,
    /// DCB field: `handRaiseDelay` (Single)
    #[serde(default)]
    pub hand_raise_delay: f32,
    /// DCB field: `handSwitchDelay` (Single)
    #[serde(default)]
    pub hand_switch_delay: f32,
    /// DCB field: `handLowerDelay` (Single)
    #[serde(default)]
    pub hand_lower_delay: f32,
    /// DCB field: `handHazardStopDelay` (Single)
    #[serde(default)]
    pub hand_hazard_stop_delay: f32,
}

impl Pooled for HazardAwarenessParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ha.hazard_awareness_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ha.hazard_awareness_params }
}

impl<'a> Extract<'a> for HazardAwarenessParams {
    const TYPE_NAME: &'static str = "HazardAwarenessParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wind_hazard_min_speed: inst.get_f32("windHazardMinSpeed").unwrap_or_default(),
            fire_hand_raise_distance: inst.get_f32("fireHandRaiseDistance").unwrap_or_default(),
            hand_raise_yaw_min: inst.get_f32("handRaiseYawMin").unwrap_or_default(),
            hand_raise_yaw_max: inst.get_f32("handRaiseYawMax").unwrap_or_default(),
            hand_raise_yaw_hysteresis: inst.get_f32("handRaiseYawHysteresis").unwrap_or_default(),
            hand_follow_yaw_min: inst.get_f32("handFollowYawMin").unwrap_or_default(),
            hand_follow_yaw_max: inst.get_f32("handFollowYawMax").unwrap_or_default(),
            hand_follow_yaw_smoothing: inst.get_f32("handFollowYawSmoothing").unwrap_or_default(),
            hand_pitch_offset: inst.get_f32("handPitchOffset").unwrap_or_default(),
            hand_ikdistance_offset: inst.get_f32("handIKDistanceOffset").unwrap_or_default(),
            hand_ikstrength: inst.get_f32("handIKStrength").unwrap_or_default(),
            hand_resume_delay: inst.get_f32("handResumeDelay").unwrap_or_default(),
            hand_raise_delay: inst.get_f32("handRaiseDelay").unwrap_or_default(),
            hand_switch_delay: inst.get_f32("handSwitchDelay").unwrap_or_default(),
            hand_lower_delay: inst.get_f32("handLowerDelay").unwrap_or_default(),
            hand_hazard_stop_delay: inst.get_f32("handHazardStopDelay").unwrap_or_default(),
        }
    }
}

