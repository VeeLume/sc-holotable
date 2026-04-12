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

/// DCB type: `SurfaceAudioPropertiesDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceAudioPropertiesDefinition {
    /// DCB field: `surfaceAudioProps` (Class (array))
    #[serde(default)]
    pub surface_audio_props: Vec<Handle<SurfaceAudioProperties>>,
}

impl Pooled for SurfaceAudioPropertiesDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.surface_audio_properties_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.surface_audio_properties_definition }
}

impl<'a> Extract<'a> for SurfaceAudioPropertiesDefinition {
    const TYPE_NAME: &'static str = "SurfaceAudioPropertiesDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_audio_props: inst.get_array("surfaceAudioProps")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SurfaceAudioProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SurfaceAudioProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SurfaceAudioProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceAudioProperties {
    /// DCB field: `surfaceType` (String)
    #[serde(default)]
    pub surface_type: String,
    /// DCB field: `damping` (Single)
    #[serde(default)]
    pub damping: f32,
    /// DCB field: `collisionTrigger` (Class)
    #[serde(default)]
    pub collision_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `collisionTriggerMinPlayTime` (Single)
    #[serde(default)]
    pub collision_trigger_min_play_time: f32,
    /// DCB field: `slideStartTrigger` (Class)
    #[serde(default)]
    pub slide_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `slideStopTrigger` (Class)
    #[serde(default)]
    pub slide_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rollStartTrigger` (Class)
    #[serde(default)]
    pub roll_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rollStopTrigger` (Class)
    #[serde(default)]
    pub roll_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rtpcSlideVelocity` (Class)
    #[serde(default)]
    pub rtpc_slide_velocity: Option<Handle<AudioRtpc>>,
    /// DCB field: `rtpcRollVelocity` (Class)
    #[serde(default)]
    pub rtpc_roll_velocity: Option<Handle<AudioRtpc>>,
    /// DCB field: `rtpcMassOther` (Class)
    #[serde(default)]
    pub rtpc_mass_other: Option<Handle<AudioRtpc>>,
    /// DCB field: `rtpcTimeSinceLastOneshot` (Class)
    #[serde(default)]
    pub rtpc_time_since_last_oneshot: Option<Handle<AudioRtpc>>,
    /// DCB field: `rtpcMomentum` (Class)
    #[serde(default)]
    pub rtpc_momentum: Option<Handle<AudioRtpc>>,
    /// DCB field: `surfaceSwitchAndState` (Class)
    #[serde(default)]
    pub surface_switch_and_state: Option<Handle<AudioSwitch>>,
}

impl Pooled for SurfaceAudioProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.surface_audio_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.surface_audio_properties }
}

impl<'a> Extract<'a> for SurfaceAudioProperties {
    const TYPE_NAME: &'static str = "SurfaceAudioProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_type: inst.get_str("surfaceType").map(String::from).unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            collision_trigger: match inst.get("collisionTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            collision_trigger_min_play_time: inst.get_f32("collisionTriggerMinPlayTime").unwrap_or_default(),
            slide_start_trigger: match inst.get("slideStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            slide_stop_trigger: match inst.get("slideStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_start_trigger: match inst.get("rollStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_stop_trigger: match inst.get("rollStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_slide_velocity: match inst.get("rtpcSlideVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_roll_velocity: match inst.get("rtpcRollVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_mass_other: match inst.get("rtpcMassOther") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_time_since_last_oneshot: match inst.get("rtpcTimeSinceLastOneshot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_momentum: match inst.get("rtpcMomentum") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            surface_switch_and_state: match inst.get("surfaceSwitchAndState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SuggestedFOVSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFOVSetup {
    /// DCB field: `suggestedFOV` (Single)
    #[serde(default)]
    pub suggested_fov: f32,
    /// DCB field: `mode` (EnumChoice)
    #[serde(default)]
    pub mode: String,
}

impl Pooled for SuggestedFOVSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.suggested_fovsetup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.suggested_fovsetup }
}

impl<'a> Extract<'a> for SuggestedFOVSetup {
    const TYPE_NAME: &'static str = "SuggestedFOVSetup";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            suggested_fov: inst.get_f32("suggestedFOV").unwrap_or_default(),
            mode: inst.get_str("mode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SubHarvestableConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfig {
    /// DCB field: `initialSlotsProbability` (Single)
    #[serde(default)]
    pub initial_slots_probability: f32,
    /// DCB field: `initialSlotsProbabilityDeepest` (StrongPointer)
    #[serde(default)]
    pub initial_slots_probability_deepest: Option<Handle<OptionalProbability>>,
    /// DCB field: `configRespawnTimeMultiplier` (Single)
    #[serde(default)]
    pub config_respawn_time_multiplier: f32,
    /// DCB field: `subHarvestables` (Class (array))
    #[serde(default)]
    pub sub_harvestables: Vec<Handle<SubHarvestableSlot>>,
}

impl Pooled for SubHarvestableConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.sub_harvestable_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.sub_harvestable_config }
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
    /// DCB field: `subConfig` (Class)
    #[serde(default)]
    pub sub_config: Option<Handle<SubHarvestableConfig>>,
}

impl Pooled for SubHarvestableConfigRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.sub_harvestable_config_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.sub_harvestable_config_record }
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

/// DCB type: `SubHarvestableMultiConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableMultiConfig {
    /// DCB field: `defaultConfig` (StrongPointer)
    #[serde(default)]
    pub default_config: Option<Handle<SubHarvestableConfigSingleBase>>,
    /// DCB field: `ignoreAttachableTagsForTaggedConfigs` (Boolean)
    #[serde(default)]
    pub ignore_attachable_tags_for_tagged_configs: bool,
    /// DCB field: `taggedConfigs` (Class (array))
    #[serde(default)]
    pub tagged_configs: Vec<Handle<TaggedSubHarvestableConfig>>,
}

impl Pooled for SubHarvestableMultiConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.sub_harvestable_multi_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.sub_harvestable_multi_config }
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
    /// DCB field: `multiConfig` (Class)
    #[serde(default)]
    pub multi_config: Option<Handle<SubHarvestableMultiConfig>>,
}

impl Pooled for SubHarvestableMultiConfigRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.sub_harvestable_multi_config_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.sub_harvestable_multi_config_record }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.sub_harvestable_config_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.sub_harvestable_config_base }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.sub_harvestable_config_single_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.sub_harvestable_config_single_base }
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
    /// DCB field: `relativeProbabilityDeepest` (StrongPointer)
    #[serde(default)]
    pub relative_probability_deepest: Option<Handle<OptionalProbability>>,
    /// DCB field: `harvestableRespawnTimeMultiplier` (Single)
    #[serde(default)]
    pub harvestable_respawn_time_multiplier: f32,
    /// DCB field: `geometries` (Class (array))
    #[serde(default)]
    pub geometries: Vec<Handle<HarvestableGeometry>>,
    /// DCB field: `lootConfig` (StrongPointer)
    #[serde(default)]
    pub loot_config: Option<Handle<LootConfig>>,
}

impl Pooled for SubHarvestableSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.sub_harvestable_slot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.sub_harvestable_slot }
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

/// DCB type: `SUninsuredItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SUninsuredItem {
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `SubTypes` (EnumChoice (array))
    #[serde(default)]
    pub sub_types: Vec<String>,
}

impl Pooled for SUninsuredItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_su.suninsured_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_su.suninsured_item }
}

impl<'a> Extract<'a> for SUninsuredItem {
    const TYPE_NAME: &'static str = "SUninsuredItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            sub_types: inst.get_array("SubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

