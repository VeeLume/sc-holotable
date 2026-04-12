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
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `OptionalProbability`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalProbability {
    /// `probability` (Single)
    #[serde(default)]
    pub probability: f32,
}

impl Pooled for OptionalProbability {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.optional_probability }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.optional_probability }
}

impl<'a> Extract<'a> for OptionalProbability {
    const TYPE_NAME: &'static str = "OptionalProbability";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            probability: inst.get_f32("probability").unwrap_or_default(),
        }
    }
}

/// DCB type: `SubHarvestableConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfig {
    /// `initialSlotsProbability` (Single)
    #[serde(default)]
    pub initial_slots_probability: f32,
    /// `initialSlotsProbabilityDeepest` (StrongPointer)
    #[serde(default)]
    pub initial_slots_probability_deepest: Option<Handle<OptionalProbability>>,
    /// `configRespawnTimeMultiplier` (Single)
    #[serde(default)]
    pub config_respawn_time_multiplier: f32,
    /// `subHarvestables` (Class (array))
    #[serde(default)]
    pub sub_harvestables: Vec<Handle<SubHarvestableSlot>>,
}

impl Pooled for SubHarvestableConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_config }
}

impl<'a> Extract<'a> for SubHarvestableConfig {
    const TYPE_NAME: &'static str = "SubHarvestableConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            initial_slots_probability: inst.get_f32("initialSlotsProbability").unwrap_or_default(),
            initial_slots_probability_deepest: match inst.get("initialSlotsProbabilityDeepest") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<OptionalProbability>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<OptionalProbability>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            config_respawn_time_multiplier: inst.get_f32("configRespawnTimeMultiplier").unwrap_or_default(),
            sub_harvestables: inst.get_array("subHarvestables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SubHarvestableSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SubHarvestableSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SubHarvestableConfigRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfigRecord {
    /// `subConfig` (Class)
    #[serde(default)]
    pub sub_config: Option<Handle<SubHarvestableConfig>>,
}

impl Pooled for SubHarvestableConfigRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_config_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_config_record }
}

impl<'a> Extract<'a> for SubHarvestableConfigRecord {
    const TYPE_NAME: &'static str = "SubHarvestableConfigRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sub_config: match inst.get("subConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SubHarvestableConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SubHarvestableConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HarvestableTagListBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableTagListBase {
}

impl Pooled for HarvestableTagListBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_tag_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_tag_list_base }
}

impl<'a> Extract<'a> for HarvestableTagListBase {
    const TYPE_NAME: &'static str = "HarvestableTagListBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TaggedSubHarvestableConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedSubHarvestableConfig {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `tagList` (StrongPointer)
    #[serde(default)]
    pub tag_list: Option<Handle<HarvestableTagListBase>>,
    /// `subConfig` (StrongPointer)
    #[serde(default)]
    pub sub_config: Option<Handle<SubHarvestableConfigSingleBase>>,
}

impl Pooled for TaggedSubHarvestableConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.tagged_sub_harvestable_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.tagged_sub_harvestable_config }
}

impl<'a> Extract<'a> for TaggedSubHarvestableConfig {
    const TYPE_NAME: &'static str = "TaggedSubHarvestableConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag_list: match inst.get("tagList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestableTagListBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestableTagListBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sub_config: match inst.get("subConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SubHarvestableConfigSingleBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SubHarvestableConfigSingleBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SubHarvestableMultiConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableMultiConfig {
    /// `defaultConfig` (StrongPointer)
    #[serde(default)]
    pub default_config: Option<Handle<SubHarvestableConfigSingleBase>>,
    /// `ignoreAttachableTagsForTaggedConfigs` (Boolean)
    #[serde(default)]
    pub ignore_attachable_tags_for_tagged_configs: bool,
    /// `taggedConfigs` (Class (array))
    #[serde(default)]
    pub tagged_configs: Vec<Handle<TaggedSubHarvestableConfig>>,
}

impl Pooled for SubHarvestableMultiConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_multi_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_multi_config }
}

impl<'a> Extract<'a> for SubHarvestableMultiConfig {
    const TYPE_NAME: &'static str = "SubHarvestableMultiConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_config: match inst.get("defaultConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SubHarvestableConfigSingleBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SubHarvestableConfigSingleBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ignore_attachable_tags_for_tagged_configs: inst.get_bool("ignoreAttachableTagsForTaggedConfigs").unwrap_or_default(),
            tagged_configs: inst.get_array("taggedConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TaggedSubHarvestableConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TaggedSubHarvestableConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SubHarvestableMultiConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SubHarvestableConfigBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfigBase {
}

impl Pooled for SubHarvestableConfigBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_config_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_config_base }
}

impl<'a> Extract<'a> for SubHarvestableConfigBase {
    const TYPE_NAME: &'static str = "SubHarvestableConfigBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SubHarvestableConfigSingleBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfigSingleBase {
}

impl Pooled for SubHarvestableConfigSingleBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_config_single_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_config_single_base }
}

impl<'a> Extract<'a> for SubHarvestableConfigSingleBase {
    const TYPE_NAME: &'static str = "SubHarvestableConfigSingleBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SubHarvestableSlot`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableSlot {
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
    /// `relativeProbabilityDeepest` (StrongPointer)
    #[serde(default)]
    pub relative_probability_deepest: Option<Handle<OptionalProbability>>,
    /// `harvestableRespawnTimeMultiplier` (Single)
    #[serde(default)]
    pub harvestable_respawn_time_multiplier: f32,
    /// `geometries` (Class (array))
    #[serde(default)]
    pub geometries: Vec<Handle<HarvestableGeometry>>,
    /// `lootConfig` (StrongPointer)
    #[serde(default)]
    pub loot_config: Option<Handle<LootConfig>>,
}

impl Pooled for SubHarvestableSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.sub_harvestable_slot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.sub_harvestable_slot }
}

impl<'a> Extract<'a> for SubHarvestableSlot {
    const TYPE_NAME: &'static str = "SubHarvestableSlot";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            harvestable: inst.get("harvestable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            harvestable_entity_class: inst.get("harvestableEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            harvestable_setup: inst.get("harvestableSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            relative_probability: inst.get_f32("relativeProbability").unwrap_or_default(),
            relative_probability_deepest: match inst.get("relativeProbabilityDeepest") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<OptionalProbability>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<OptionalProbability>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            harvestable_respawn_time_multiplier: inst.get_f32("harvestableRespawnTimeMultiplier").unwrap_or_default(),
            geometries: inst.get_array("geometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HarvestableGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HarvestableGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            loot_config: match inst.get("lootConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HarvestableTransformParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableTransformParams {
    /// `localRotationOffset` (Class)
    #[serde(default)]
    pub local_rotation_offset: Option<Handle<Vec3>>,
    /// `minScale` (Single)
    #[serde(default)]
    pub min_scale: f32,
    /// `maxScale` (Single)
    #[serde(default)]
    pub max_scale: f32,
    /// `terrainNormalAlignment` (Single)
    #[serde(default)]
    pub terrain_normal_alignment: f32,
    /// `rotationRange` (Class)
    #[serde(default)]
    pub rotation_range: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
    /// `minZOffset` (Single)
    #[serde(default)]
    pub min_zoffset: f32,
    /// `maxZOffset` (Single)
    #[serde(default)]
    pub max_zoffset: f32,
    /// `minSlope` (Single)
    #[serde(default)]
    pub min_slope: f32,
    /// `maxSlope` (Single)
    #[serde(default)]
    pub max_slope: f32,
    /// `minElevation` (Single)
    #[serde(default)]
    pub min_elevation: f32,
    /// `maxElevation` (Single)
    #[serde(default)]
    pub max_elevation: f32,
}

impl Pooled for HarvestableTransformParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_transform_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_transform_params }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvest_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvest_condition_base }
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
    /// `despawnTimeSeconds` (Int32)
    #[serde(default)]
    pub despawn_time_seconds: i32,
    /// `additionalWaitForNearbyPlayersSeconds` (Int32)
    #[serde(default)]
    pub additional_wait_for_nearby_players_seconds: i32,
}

impl Pooled for HarvestDespawnTimerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvest_despawn_timer_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvest_despawn_timer_params }
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
    /// `harvestConditions` (StrongPointer (array))
    #[serde(default)]
    pub harvest_conditions: Vec<Handle<HarvestConditionBase>>,
    /// `despawnTimer` (Class)
    #[serde(default)]
    pub despawn_timer: Option<Handle<HarvestDespawnTimerParams>>,
}

impl Pooled for HarvestBehaviourParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvest_behaviour_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvest_behaviour_params }
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
    /// `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
    /// `harvestBehaviour` (Class)
    #[serde(default)]
    pub harvest_behaviour: Option<Handle<HarvestBehaviourParams>>,
    /// `transformParams` (Class)
    #[serde(default)]
    pub transform_params: Option<Handle<HarvestableTransformParams>>,
    /// `subConfigBase` (StrongPointer)
    #[serde(default)]
    pub sub_config_base: Option<Handle<SubHarvestableConfigBase>>,
    /// `respawnInSlotTime` (Single)
    #[serde(default)]
    pub respawn_in_slot_time: f32,
    /// `specialHarvestableString` (String)
    #[serde(default)]
    pub special_harvestable_string: String,
}

impl Pooled for HarvestablePreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_preset }
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
    /// `harvestBehaviour` (Class)
    #[serde(default)]
    pub harvest_behaviour: Option<Handle<HarvestBehaviourParams>>,
    /// `transformParams` (Class)
    #[serde(default)]
    pub transform_params: Option<Handle<HarvestableTransformParams>>,
    /// `subConfigBase` (StrongPointer)
    #[serde(default)]
    pub sub_config_base: Option<Handle<SubHarvestableConfigBase>>,
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
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
}

impl Pooled for HarvestableGeometry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_geometry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_geometry }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.harvestable_area_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.harvestable_area_type_base }
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
    /// `debugGroupName` (String)
    #[serde(default)]
    pub debug_group_name: String,
    /// `areaType` (StrongPointer)
    #[serde(default)]
    pub area_type: Option<Handle<HarvestableAreaTypeBase>>,
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

/// DCB type: `FloatFactorRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatFactorRange {
    /// `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// `max` (Single)
    #[serde(default)]
    pub max: f32,
}

impl Pooled for FloatFactorRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.float_factor_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.float_factor_range }
}

impl<'a> Extract<'a> for FloatFactorRange {
    const TYPE_NAME: &'static str = "FloatFactorRange";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
        }
    }
}

/// DCB type: `AdvancedLootConstraints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedLootConstraints {
    /// `pruningLevel` (EnumChoice)
    #[serde(default)]
    pub pruning_level: String,
    /// `fullnessMode` (EnumChoice)
    #[serde(default)]
    pub fullness_mode: String,
}

impl Pooled for AdvancedLootConstraints {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.advanced_loot_constraints }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.advanced_loot_constraints }
}

impl<'a> Extract<'a> for AdvancedLootConstraints {
    const TYPE_NAME: &'static str = "AdvancedLootConstraints";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pruning_level: inst.get_str("pruningLevel").map(String::from).unwrap_or_default(),
            fullness_mode: inst.get_str("fullnessMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `LootConstraints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootConstraints {
    /// `poolFilter` (Reference)
    #[serde(default)]
    pub pool_filter: Option<CigGuid>,
    /// `secondaryChoices` (StrongPointer)
    #[serde(default)]
    pub secondary_choices: Option<Handle<LootV3SecondaryChoicesRecordRef_Base>>,
    /// `fullnessFactorRange` (Class)
    #[serde(default)]
    pub fullness_factor_range: Option<Handle<FloatFactorRange>>,
    /// `totalResultsLimit` (Int32)
    #[serde(default)]
    pub total_results_limit: i32,
    /// `chanceToGenerate` (Single)
    #[serde(default)]
    pub chance_to_generate: f32,
    /// `chanceToGenerateAdditionalAttachedInventories` (Single)
    #[serde(default)]
    pub chance_to_generate_additional_attached_inventories: f32,
    /// `advanced` (Class)
    #[serde(default)]
    pub advanced: Option<Handle<AdvancedLootConstraints>>,
}

impl Pooled for LootConstraints {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.loot_constraints }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.loot_constraints }
}

impl<'a> Extract<'a> for LootConstraints {
    const TYPE_NAME: &'static str = "LootConstraints";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pool_filter: inst.get("poolFilter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            secondary_choices: match inst.get("secondaryChoices") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootV3SecondaryChoicesRecordRef_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootV3SecondaryChoicesRecordRef_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fullness_factor_range: match inst.get("fullnessFactorRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FloatFactorRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FloatFactorRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            total_results_limit: inst.get_i32("totalResultsLimit").unwrap_or_default(),
            chance_to_generate: inst.get_f32("chanceToGenerate").unwrap_or_default(),
            chance_to_generate_additional_attached_inventories: inst.get_f32("chanceToGenerateAdditionalAttachedInventories").unwrap_or_default(),
            advanced: match inst.get("advanced") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AdvancedLootConstraints>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AdvancedLootConstraints>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootConfig {
    /// `lootConstraints` (Class)
    #[serde(default)]
    pub loot_constraints: Option<Handle<LootConstraints>>,
    /// `lootTable` (Reference)
    #[serde(default)]
    pub loot_table: Option<CigGuid>,
    /// `lootTableV3` (Reference)
    #[serde(default)]
    pub loot_table_v3: Option<CigGuid>,
}

impl Pooled for LootConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.loot_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.loot_config }
}

impl<'a> Extract<'a> for LootConfig {
    const TYPE_NAME: &'static str = "LootConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loot_constraints: match inst.get("lootConstraints") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootConstraints>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootConstraints>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loot_table: inst.get("lootTable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            loot_table_v3: inst.get("lootTableV3").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `LootV3SecondaryChoicesRecordRef_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoicesRecordRef_Base {
}

impl Pooled for LootV3SecondaryChoicesRecordRef_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.harvestable.loot_v3_secondary_choices_record_ref_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.harvestable.loot_v3_secondary_choices_record_ref_base }
}

impl<'a> Extract<'a> for LootV3SecondaryChoicesRecordRef_Base {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoicesRecordRef_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

